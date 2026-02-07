//! Contains the parser implementation for the programming language.
pub mod types;

use std::mem::discriminant;

use lexer::types::{Keyword, Token, TokenKind};

use crate::types::{
    BinaryOperator, Expr, Expression, Literal, Program, Span, Spanned, Statement, Stmt,
    UnaryOperator,
};

/// The parser struct responsible for parsing tokens into an AST.
pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    outside_global_scope: bool,
    inside_class: Option<String>,
    inside_static: bool,
}

impl Parser {
    const COMPARISON_TOKEN: [TokenKind; 6] = [
        TokenKind::EqualsEquals,
        TokenKind::NotEquals,
        TokenKind::LeftAngle,
        TokenKind::RightAngle,
        TokenKind::LessThanOrEqual,
        TokenKind::GreaterThanOrEqual,
    ];

    /// Parses the tokens and returns the root of the AST.
    ///
    /// # Errors
    /// Unexpected end of input or invalid syntax.
    pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
        let mut parser: Self = Self {
            tokens,
            index: 0,
            outside_global_scope: false,
            inside_class: None,
            inside_static: false,
        };

        let mut statements: Vec<Stmt> = Vec::new();

        while !parser.is_eof()? {
            statements.push(parser.parse_statement()?);
        }

        Ok(Program { statements })
    }

    fn is_eof(&self) -> Result<bool, String> {
        Ok(matches!(self.peek()?.kind, TokenKind::EndOfFile))
    }

    fn peek(&self) -> Result<&Token, String> {
        self.tokens
            .get(self.index)
            .ok_or_else(|| "Unexpected end of input".to_string())
    }

    const fn advance(&mut self) {
        self.index += 1;
    }

    fn match_token(&self, kind: &lexer::types::TokenKind) -> bool {
        if let Ok(token) = self.peek()
            && &token.kind == kind
        {
            return true;
        }
        false
    }

    fn expect_token(&mut self, kind: &lexer::types::TokenKind) -> Result<&Token, String> {
        if self.match_token(kind) {
            self.advance();
            Ok(&self.tokens[self.index - 1])
        } else if let Ok(token) = self.peek() {
            Err(format!(
                "Expected token '{:?}', found '{:?}' at {}:{}",
                kind, token.kind, token.start.0, token.start.1
            ))
        } else {
            Err(format!("Expected token '{kind:?}', found end of input"))
        }
    }

    fn expect_token_kind(&mut self, kind: &lexer::types::TokenKind) -> Result<&Token, String> {
        if discriminant(&self.peek()?.kind) == discriminant(kind) {
            self.advance();
            Ok(&self.tokens[self.index - 1])
        } else if let Ok(token) = self.peek() {
            Err(format!(
                "Expected token '{:?}', found '{:?}' at {}:{}",
                kind, token.kind, token.start.0, token.start.1
            ))
        } else {
            Err(format!("Expected token '{kind:?}', found end of input"))
        }
    }

    fn check_next_tokens(&self, kinds: &[TokenKind]) -> bool {
        for (i, kind) in kinds.iter().enumerate() {
            if let Some(token) = self.tokens.get(self.index + i) {
                if discriminant(&token.kind) != discriminant(kind) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    fn parse_postfix_chain(
        &mut self,
        mut expr: Expr,
        start: (usize, usize),
    ) -> Result<Expr, String> {
        loop {
            match self.peek()?.kind.clone() {
                TokenKind::Dot => {
                    self.advance();
                    let member: &Token =
                        self.expect_token_kind(&TokenKind::Identifier(String::new()))?;

                    expr = Spanned {
                        node: Expression::MemberAccess {
                            object: Box::new(expr),
                            member: match &member.kind {
                                TokenKind::Identifier(name) => name.clone(),
                                _ => unreachable!(),
                            },
                        },
                        span: Span {
                            start,
                            end: member.end,
                        },
                    };
                }
                TokenKind::LeftParen => {
                    self.advance();
                    expr = self.parse_function_call(Box::new(expr), start)?;
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cognitive_complexity)]
    fn parse_statement(&mut self) -> Result<Stmt, String> {
        if matches!(self.peek()?.kind, TokenKind::Keyword(_)) {
            return self.parse_keyworded();
        }

        let first_token: Token = self.peek()?.clone();
        let first_ident: String = if let TokenKind::Identifier(name) = &first_token.kind {
            name.clone()
        } else {
            let expr: Spanned<Expression> = self.parse_expression()?;
            let start: (usize, usize) = expr.span.start;
            let end: (usize, usize) = self.expect_token(&TokenKind::Semicolon)?.end;
            return Ok(Spanned {
                node: Statement::Expression(expr),
                span: Span { start, end },
            });
        };

        let start: (usize, usize) = self.peek()?.start;
        self.advance();

        let second_token_kind: TokenKind = self.peek()?.kind.clone();
        self.advance();

        match second_token_kind {
            TokenKind::Identifier(_) => match self.peek()?.kind.clone() {
                TokenKind::Equals | TokenKind::Semicolon => {
                    self.index -= 2;
                    self.parse_variable_declaration()
                }
                TokenKind::LeftParen => {
                    self.index -= 2;
                    self.parse_function_declaration()
                }
                _ => {
                    let peek: &Token = self.peek()?;
                    Err(format!(
                        "Invalid token following two identifiers: '{:?}' at {}:{}",
                        peek.kind, peek.start.0, peek.start.1
                    ))
                }
            },
            TokenKind::Dot => {
                self.index -= 1; // Set current token to dot. parse_postfix_chain MUST see the dot.
                self.parse_statement_with_member(
                    &Spanned {
                        node: Expression::Identifier(first_ident),
                        span: Span {
                            start,
                            end: first_token.end,
                        },
                    },
                    start,
                )
            }
            TokenKind::Equals => {
                self.index -= 2;
                self.parse_assignment()
            }
            TokenKind::LeftParen => {
                let end: (usize, usize) = self.tokens[self.index - 1].end;
                let expr: Expr = self.parse_function_call(
                    Box::new(Spanned {
                        node: Expression::Identifier(first_ident),
                        span: Span { start, end },
                    }),
                    start,
                )?;
                if self.peek()?.kind == TokenKind::Dot {
                    return self.parse_statement_with_member(&expr, start);
                }
                let end: (usize, usize) = self.expect_token(&TokenKind::Semicolon)?.end;
                Ok(Spanned {
                    node: Statement::Expression(expr),
                    span: Span { start, end },
                })
            }
            TokenKind::Semicolon => {
                self.index -= 1;
                let expr: Expr = self.parse_expression()?;
                let end: (usize, usize) = self.expect_token(&TokenKind::Semicolon)?.end;
                Ok(Spanned {
                    node: Statement::Expression(expr),
                    span: Span { start, end },
                })
            }
            _ => {
                let err_start: (usize, usize) = self.peek()?.start;
                Err(format!(
                    "Unexpected token after identifier: {:?} at {}:{}",
                    first_ident, err_start.0, err_start.1
                ))
            }
        }
    }

    fn parse_statement_with_member(
        &mut self,
        expr: &Expr,
        start: (usize, usize),
    ) -> Result<Stmt, String> {
        let expr: Expr = self.parse_postfix_chain(expr.clone(), start)?;

        match self.peek()?.kind.clone() {
            TokenKind::Equals => self.parse_named_assignment(Box::new(expr), start),
            TokenKind::Semicolon => {
                let end: (usize, usize) = self.expect_token(&TokenKind::Semicolon)?.end;
                Ok(Spanned {
                    node: Statement::Expression(expr),
                    span: Span { start, end },
                })
            }
            TokenKind::Dot | TokenKind::LeftParen => {
                let expr: Expr = self.parse_postfix_chain(expr, start)?;
                let end: (usize, usize) = self.expect_token(&TokenKind::Semicolon)?.end;
                Ok(Spanned {
                    node: Statement::Expression(expr),
                    span: Span { start, end },
                })
            }
            _ => {
                let err_start: (usize, usize) = self.peek()?.start;
                Err(format!(
                    "Unexpected token after member access at {}:{}",
                    err_start.0, err_start.1
                ))
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn parse_keyworded(&mut self) -> Result<Stmt, String> {
        let kind: TokenKind = self.peek()?.kind.clone();
        match kind {
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::If => self.parse_if_statement(),
                Keyword::Else => Err(format!(
                    "Unexpected 'else' without matching 'if' at {}:{}",
                    self.peek()?.start.0,
                    self.peek()?.start.1
                )),
                Keyword::While => self.parse_while_loop(),
                Keyword::Return => {
                    let start: (usize, usize) =
                        self.expect_token(&TokenKind::Keyword(Keyword::Return))?.end;
                    let expr: Option<Expr> = if self.match_token(&TokenKind::Semicolon) {
                        None
                    } else {
                        Some(self.parse_expression()?)
                    };
                    let end: (usize, usize) = self.expect_token(&TokenKind::Semicolon)?.end;
                    Ok(Spanned {
                        node: Statement::Return(expr),
                        span: Span { start, end },
                    })
                }
                Keyword::Class => self.parse_class_declaration(),
                Keyword::Self_ => {
                    let token: Token = self
                        .expect_token(&TokenKind::Keyword(Keyword::Self_))?
                        .clone();
                    let start: (usize, usize) = token.start;
                    let end: (usize, usize) = token.end;

                    if self.inside_class.is_some() && !self.inside_static {
                        Ok(Spanned {
                            node: Statement::Expression(Spanned {
                                node: Expression::Self_,
                                span: Span { start, end },
                            }),
                            span: Span { start, end },
                        })
                    } else {
                        Err(format!(
                            "Illegal use of 'self' outside class instance methods at {}:{}",
                            start.0, start.1
                        ))
                    }
                }
                Keyword::SelfType => {
                    let token: Token = self
                        .expect_token(&TokenKind::Keyword(Keyword::SelfType))?
                        .clone();
                    let start: (usize, usize) = token.start;
                    let end: (usize, usize) = token.end;

                    self.inside_class.as_ref().map_or_else(
                        || {
                            Err(format!(
                                "Illegal use of 'Self' outside class at {}:{}",
                                start.0, start.1
                            ))
                        },
                        |class_name| {
                            Ok(Spanned {
                                node: Statement::Expression(Spanned {
                                    node: Expression::SelfType(class_name.clone()),
                                    span: Span { start, end },
                                }),
                                span: Span { start, end },
                            })
                        },
                    )
                }
                Keyword::Static => {
                    self.inside_static = true;

                    if self.inside_class.is_none() {
                        return Err(format!(
                            "The 'static' keyword can only be used inside a class at {}:{}",
                            self.peek()?.start.0,
                            self.peek()?.start.1
                        ));
                    }

                    let token: Token = self
                        .expect_token(&TokenKind::Keyword(Keyword::Static))?
                        .clone();
                    let stmt: Stmt = self.parse_statement()?;

                    match &stmt.node {
                        Statement::MethodDeclaration { .. }
                        | Statement::FieldDeclaration { .. } => {
                            self.inside_static = true;
                            Ok(Spanned {
                                node: stmt.node,
                                span: Span {
                                    start: token.start,
                                    end: stmt.span.end,
                                },
                            })
                        }
                        _ => Err(format!(
                            "The 'static' keyword can only be used on method and {}{}:{}",
                            " declarations at ", token.start.0, token.start.1
                        )),
                    }
                }
            },
            _ => unreachable!(),
        }
    }

    fn parse_if_statement(&mut self) -> Result<Stmt, String> {
        let mut conditional_branches: Vec<(Expr, Vec<Stmt>)> = Vec::new();
        let mut else_branch: Option<Vec<Stmt>> = None;
        let if_token: Token = self.expect_token(&TokenKind::Keyword(Keyword::If))?.clone();
        let start: (usize, usize) = if_token.start;
        let mut end: (usize, usize);

        loop {
            let cond_start: (usize, usize) = self.expect_token(&TokenKind::LeftParen)?.start;
            let condition: Expr = self.parse_expression()?;
            let cond_end: (usize, usize) = self.expect_token(&TokenKind::RightParen)?.end;

            self.expect_token(&TokenKind::LeftBrace)?;
            let mut body: Vec<Stmt> = Vec::new();
            while !self.match_token(&TokenKind::RightBrace) {
                body.push(self.parse_statement()?);
            }
            end = self.expect_token(&TokenKind::RightBrace)?.clone().end;

            conditional_branches.push((
                Spanned {
                    node: condition.node,
                    span: Span {
                        start: cond_start,
                        end: cond_end,
                    },
                },
                body,
            ));

            if !self.match_token(&TokenKind::Keyword(Keyword::Else)) {
                return Ok(Spanned {
                    node: Statement::If {
                        conditional_branches,
                        else_branch,
                    },
                    span: Span { start, end },
                });
            }

            if !self.check_next_tokens(&[
                TokenKind::Keyword(Keyword::Else),
                TokenKind::Keyword(Keyword::If),
            ]) {
                break;
            }

            self.expect_token(&TokenKind::Keyword(Keyword::Else))?;
            self.expect_token(&TokenKind::Keyword(Keyword::If))?;
        }

        self.expect_token(&TokenKind::Keyword(Keyword::Else))?;
        self.expect_token(&TokenKind::LeftBrace)?;
        let mut body: Vec<Stmt> = Vec::new();
        while !self.match_token(&TokenKind::RightBrace) {
            body.push(self.parse_statement()?);
        }
        end = self.expect_token(&TokenKind::RightBrace)?.clone().end;
        else_branch = Some(body);

        Ok(Spanned {
            node: Statement::If {
                conditional_branches,
                else_branch,
            },
            span: Span { start, end },
        })
    }

    fn parse_while_loop(&mut self) -> Result<Stmt, String> {
        let while_loop: Token = self
            .expect_token(&TokenKind::Keyword(Keyword::While))?
            .clone();
        let start: (usize, usize) = while_loop.start;

        let cond_start: (usize, usize) = self.expect_token(&TokenKind::LeftParen)?.start;
        let condition: Expr = self.parse_expression()?;
        let cond_end: (usize, usize) = self.expect_token(&TokenKind::RightParen)?.end;

        self.expect_token(&TokenKind::LeftBrace)?;
        let mut body: Vec<Stmt> = Vec::new();
        while !self.match_token(&TokenKind::RightBrace) {
            body.push(self.parse_statement()?);
        }

        let end: (usize, usize) = self.expect_token(&TokenKind::RightBrace)?.clone().end;

        Ok(Spanned {
            node: Statement::While {
                condition: Spanned {
                    node: condition.node,
                    span: Span {
                        start: cond_start,
                        end: cond_end,
                    },
                },
                body,
            },
            span: Span { start, end },
        })
    }

    fn parse_class_declaration(&mut self) -> Result<Stmt, String> {
        if self.outside_global_scope {
            return Err(format!(
                "Class declarations are only allowed in the global scope at {}:{}",
                self.peek()?.start.0,
                self.peek()?.start.1
            ));
        }

        let class_token: Token = self
            .expect_token(&TokenKind::Keyword(Keyword::Class))?
            .clone();
        let start: (usize, usize) = class_token.start;

        let identifier: String = match self
            .expect_token_kind(&TokenKind::Identifier(String::new()))?
            .kind
            .clone()
        {
            TokenKind::Identifier(name) => name,
            _ => unreachable!(),
        };

        self.expect_token(&TokenKind::LeftBrace)?;

        self.outside_global_scope = true;
        self.inside_class = Some(identifier.clone());
        self.inside_static = false;

        let mut body: Vec<Stmt> = Vec::new();
        while !self.match_token(&TokenKind::RightBrace) {
            body.push(self.parse_statement()?);
        }
        let end: (usize, usize) = self.expect_token(&TokenKind::RightBrace)?.end;

        self.outside_global_scope = false;
        self.inside_class = None;
        self.inside_static = false;

        Ok(Spanned {
            node: Statement::ClassDeclaration {
                name: identifier,
                body,
            },
            span: Span { start, end },
        })
    }

    fn parse_variable_declaration(&mut self) -> Result<Stmt, String> {
        if self.inside_class.is_some() {
            return self.parse_field_declaration();
        }
        let token: Token = self.peek()?.clone();
        let type_: String = match &token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => unreachable!(),
        };
        let start: (usize, usize) = token.start;
        self.advance();

        let name: String = match &self.peek()?.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => unreachable!(),
        };
        self.advance();

        let value: Option<Expr> = if self.match_token(&TokenKind::Equals) {
            self.expect_token(&TokenKind::Equals)?;
            Some(self.parse_expression()?)
        } else {
            None
        };

        let end: (usize, usize) = self.expect_token(&TokenKind::Semicolon)?.end;
        Ok(Spanned {
            node: Statement::VariableDeclaration { type_, name, value },
            span: Span { start, end },
        })
    }

    fn parse_field_declaration(&mut self) -> Result<Stmt, String> {
        let token: Token = self.peek()?.clone();
        let type_: String = match &token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => unreachable!(),
        };
        let start: (usize, usize) = token.start;
        self.advance();

        let name: String = match &self.peek()?.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => unreachable!(),
        };
        self.advance();

        let end: (usize, usize) = self.expect_token(&TokenKind::Semicolon)?.end;
        Ok(Spanned {
            node: Statement::FieldDeclaration {
                type_,
                name,
                static_: self.inside_static,
            },
            span: Span { start, end },
        })
    }

    fn parse_function_declaration(&mut self) -> Result<Stmt, String> {
        let token: Token = self.peek()?.clone();
        let return_type: String = match &token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => unreachable!(),
        };
        self.advance();

        let name: String = match &self.peek()?.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => unreachable!(),
        };
        self.advance();

        self.expect_token(&TokenKind::LeftParen)?;
        let parameters: Vec<(String, String)> = self.parse_function_declaration_parameters()?;
        self.expect_token(&TokenKind::RightParen)?;

        self.expect_token(&TokenKind::LeftBrace)?;
        let outside_global_scope_backup: bool = self.outside_global_scope;
        self.outside_global_scope = true;
        let mut body: Vec<Stmt> = Vec::new();
        while !self.match_token(&TokenKind::RightBrace) {
            body.push(self.parse_statement()?);
        }
        let end: (usize, usize) = self.expect_token(&TokenKind::RightBrace)?.end;
        self.outside_global_scope = outside_global_scope_backup;

        if self.inside_class.is_some() {
            Ok(Spanned {
                node: Statement::MethodDeclaration {
                    return_type,
                    name,
                    parameters,
                    body,
                    static_: self.inside_static,
                },
                span: Span {
                    start: token.start,
                    end,
                },
            })
        } else {
            Ok(Spanned {
                node: Statement::FunctionDeclaration {
                    return_type,
                    name,
                    parameters,
                    body,
                },
                span: Span {
                    start: token.start,
                    end,
                },
            })
        }
    }

    fn parse_function_declaration_parameters(&mut self) -> Result<Vec<(String, String)>, String> {
        let mut parameters: Vec<(String, String)> = Vec::new();

        loop {
            if self.match_token(&TokenKind::RightParen) {
                break;
            }

            let identifier: &Token =
                self.expect_token_kind(&TokenKind::Identifier(String::new()))?;
            let TokenKind::Identifier(type_) = &identifier.kind.clone() else {
                unreachable!()
            };

            let identifier: &Token =
                self.expect_token_kind(&TokenKind::Identifier(String::new()))?;
            let TokenKind::Identifier(name) = &identifier.kind else {
                unreachable!()
            };

            parameters.push((type_.clone(), name.clone()));

            let peek: &Token = self.peek()?;
            match peek.kind {
                TokenKind::Comma => {
                    self.advance();
                }
                TokenKind::RightParen => {
                    break;
                }
                _ => {
                    return Err(format!(
                        "Expected ',' or ')', found '{:?}' at {}:{}",
                        peek.kind, peek.start.0, peek.start.1
                    ));
                }
            }
        }

        Ok(parameters)
    }

    fn parse_named_assignment(
        &mut self,
        name: Box<Expr>,
        start: (usize, usize),
    ) -> Result<Stmt, String> {
        self.expect_token(&TokenKind::Equals)?;

        let value: Expr = self.parse_expression()?;

        self.expect_token(&TokenKind::Semicolon)?;

        let end: (usize, usize) = value.span.end;

        Ok(Spanned {
            node: Statement::Assignment {
                assignee: name,
                value,
            },
            span: Span { start, end },
        })
    }

    fn parse_assignment(&mut self) -> Result<Stmt, String> {
        let token: Token = self.peek()?.clone();
        self.advance();
        self.parse_named_assignment(
            Box::new(Spanned {
                node: Expression::Identifier(match &token.kind {
                    TokenKind::Identifier(name) => name.clone(),
                    x => unreachable!("Expected identifier, found {:?}", x),
                }),
                span: Span {
                    start: token.start,
                    end: token.end,
                },
            }),
            token.start,
        )
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_precedence(0, false)
    }

    fn operator_precedence(kind: &TokenKind) -> Option<u8> {
        match kind {
            TokenKind::Asterisk | TokenKind::Slash => Some(5),
            TokenKind::Plus | TokenKind::Minus => Some(4),
            _ if Self::COMPARISON_TOKEN.contains(kind) => Some(3),
            TokenKind::And => Some(2),
            TokenKind::Or => Some(1),
            _ => None,
        }
    }

    fn parse_precedence(&mut self, min_prec: u8, seen_comparison: bool) -> Result<Expr, String> {
        let mut left: Expr = self.parse_unary()?;

        while let Ok(next) = self.peek() {
            let op_token: Token = next.clone();
            let is_comparison_op: bool = Self::COMPARISON_TOKEN.contains(&op_token.kind);

            if seen_comparison && is_comparison_op {
                return Err(format!(
                    "Chained comparison operators are not allowed ({}:{})",
                    op_token.start.0, op_token.start.1
                ));
            }

            let prec: u8 = match Self::operator_precedence(&op_token.kind) {
                Some(p) if p >= min_prec => p,
                _ => break,
            };

            self.advance();

            let right: Expr =
                self.parse_precedence(prec + 1, seen_comparison || is_comparison_op)?;

            let operator: BinaryOperator = match op_token.kind {
                TokenKind::Plus => BinaryOperator::Add,
                TokenKind::Minus => BinaryOperator::Subtract,
                TokenKind::Asterisk => BinaryOperator::Multiply,
                TokenKind::Slash => BinaryOperator::Divide,
                TokenKind::EqualsEquals => BinaryOperator::Equals,
                TokenKind::NotEquals => BinaryOperator::NotEquals,
                TokenKind::LeftAngle => BinaryOperator::LessThan,
                TokenKind::RightAngle => BinaryOperator::GreaterThan,
                TokenKind::LessThanOrEqual => BinaryOperator::LessThanOrEqual,
                TokenKind::GreaterThanOrEqual => BinaryOperator::GreaterThanOrEqual,
                TokenKind::And => BinaryOperator::And,
                TokenKind::Or => BinaryOperator::Or,
                _ => unreachable!(),
            };

            let start: (usize, usize) = left.span.start;
            let end: (usize, usize) = right.span.end;

            left = Spanned {
                node: Expression::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                },
                span: Span { start, end },
            }
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        let token: Token = self.peek()?.clone();

        let operator: Option<UnaryOperator> = match token.kind {
            TokenKind::Exclamation => Some(UnaryOperator::Not),
            _ => None,
        };

        let operator: UnaryOperator = match operator {
            Some(op) => op,
            None => return self.parse_primary(),
        };

        self.advance();

        let operand: Expr = self.parse_unary()?;

        let start: (usize, usize) = token.start;
        let end: (usize, usize) = operand.span.end;

        Ok(Spanned {
            node: Expression::Unary {
                operator,
                operand: Box::new(operand),
            },
            span: Span { start, end },
        })
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        let token: Token = self.peek()?.clone();
        let start: (usize, usize) = token.start;
        match token.kind {
            TokenKind::Integer(_)
            | TokenKind::Float(_)
            | TokenKind::String(_)
            | TokenKind::Boolean(_) => self.parse_literal(),
            TokenKind::LeftParen => {
                self.advance();
                let expr: Expr = self.parse_expression()?;
                let end: (usize, usize) = self.expect_token(&TokenKind::RightParen)?.end;
                Ok(Spanned {
                    node: expr.node,
                    span: Span { start, end },
                })
            }
            TokenKind::Identifier(identifier) => {
                self.advance();
                if self.match_token(&TokenKind::LeftParen) {
                    self.expect_token(&TokenKind::LeftParen)?;
                    return self.parse_function_call(
                        Box::new(Spanned {
                            node: Expression::Identifier(identifier),
                            span: Span {
                                start,
                                end: token.end,
                            },
                        }),
                        start,
                    );
                }
                Ok(Spanned {
                    node: Expression::Identifier(identifier),
                    span: Span {
                        start,
                        end: token.end,
                    },
                })
            }
            _ => Err(format!(
                "Unexpected token: '{:?}' at {}:{}",
                token.kind, token.start.0, token.start.1
            )),
        }
    }

    fn parse_literal(&mut self) -> Result<Expr, String> {
        let token: Token = self.peek()?.clone();
        let start: (usize, usize) = token.start;
        let end: (usize, usize) = token.end;
        match &token.kind {
            TokenKind::Integer(value) => {
                self.advance();
                Ok(Spanned {
                    node: Expression::Literal(Literal::Integer(*value)),
                    span: Span { start, end },
                })
            }
            TokenKind::Float(value) => {
                self.advance();
                Ok(Spanned {
                    node: Expression::Literal(Literal::Float(*value)),
                    span: Span { start, end },
                })
            }
            TokenKind::String(value) => {
                self.advance();
                Ok(Spanned {
                    node: Expression::Literal(Literal::String(value.clone())),
                    span: Span { start, end },
                })
            }
            TokenKind::Boolean(value) => {
                self.advance();
                Ok(Spanned {
                    node: Expression::Literal(Literal::Boolean(*value)),
                    span: Span { start, end },
                })
            }
            _ => Err(format!(
                "Expected literal, found {:?} at {}:{}",
                token.kind, start.0, start.1
            )),
        }
    }

    fn parse_function_call(
        &mut self,
        callee: Box<Expr>,
        start: (usize, usize),
    ) -> Result<Expr, String> {
        let mut arguments: Vec<Expr> = Vec::new();

        if !self.match_token(&TokenKind::RightParen) {
            loop {
                let value: Expr = self.parse_expression()?;
                arguments.push(value);

                match self.peek()?.kind {
                    TokenKind::Comma => {
                        self.advance();
                    }
                    TokenKind::RightParen => {
                        break;
                    }
                    _ => {
                        let peek: &Token = self.peek()?;
                        return Err(format!(
                            "Expected ',' or ')', found '{:?}' at {}:{}",
                            peek.kind, peek.start.0, peek.start.1
                        ));
                    }
                }
            }
        }

        let end: (usize, usize) = self.expect_token(&TokenKind::RightParen)?.end;

        Ok(Spanned {
            node: Expression::Call { callee, arguments },
            span: Span { start, end },
        })
    }
}

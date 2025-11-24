//! Contains the parser implementation for the programming language.
pub mod types;

use lexer::types::{Keyword, Token, TokenKind};

use crate::types::{Expression, Literal, Operator, Program, Statement};

/// The parser struct responsible for parsing tokens into an AST.
pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    /// Parses the tokens and returns the root of the AST.
    ///
    /// # Errors
    /// Unexpected end of input or invalid syntax.
    pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
        let mut parser: Self = Self { tokens, index: 0 };

        let mut statements: Vec<Statement> = Vec::new();

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

    fn match_token(&mut self, kind: &lexer::types::TokenKind) -> bool {
        if let Ok(token) = self.peek()
            && &token.kind == kind
        {
            self.advance();
            return true;
        }
        false
    }

    fn expect_token(&mut self, kind: &lexer::types::TokenKind) -> Result<&Token, String> {
        if self.match_token(kind) {
            self.peek()
        } else if let Ok(token) = self.peek() {
            Err(format!(
                "Expected token '{:?}', found '{:?}'",
                kind, token.kind
            ))
        } else {
            Err(format!("Expected token '{kind:?}', found end of input"))
        }
    }

    fn is_assignment(&self) -> bool {
        if let Some(token) = self.tokens.get(self.index)
            && let TokenKind::Identifier(_) = token.kind
        {
            return self
                .tokens
                .get(self.index + 1)
                .map_or_else(|| false, |t| t.kind == TokenKind::Equals);
        }
        false
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        if self.match_token(&TokenKind::Keyword(Keyword::Let)) {
            return self.parse_variable_declaration();
        } else if self.is_assignment() {
            return self.parse_variable_assignment();
        }
        let expr: Expression = self.parse_expression()?;
        self.expect_token(&TokenKind::Semicolon)?;
        Ok(Statement::Expression(expr))
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, String> {
        let identifier: String = match &self.peek()?.kind {
            TokenKind::Identifier(name) => name.clone(),
            token => {
                return Err(format!(
                    "Expected identifier after 'let', found '{token:?}'"
                ));
            }
        };
        self.advance();

        let value: Option<Expression> = if self.match_token(&TokenKind::Equals) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect_token(&TokenKind::Semicolon)?;
        Ok(Statement::VariableDeclaration {
            name: identifier,
            value,
        })
    }

    fn parse_variable_assignment(&mut self) -> Result<Statement, String> {
        let identifier: String = match &self.peek()?.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => {
                unreachable!("Checked for identifier token before")
            }
        };

        self.advance();
        self.expect_token(&TokenKind::Equals)?;

        let value: Expression = self.parse_expression()?;

        self.expect_token(&TokenKind::Semicolon)?;

        Ok(Statement::VariableAssignment {
            name: identifier,
            value,
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_precedence(0)
    }

    const fn operator_precedence(kind: &TokenKind) -> Option<u8> {
        match kind {
            TokenKind::Asterisk | TokenKind::Slash => Some(2),
            TokenKind::Plus | TokenKind::Minus => Some(1),
            _ => None,
        }
    }

    fn parse_precedence(&mut self, min_prec: u8) -> Result<Expression, String> {
        let mut left: Expression = self.parse_primary()?;

        while let Ok(next) = self.peek() {
            let op_token: Token = next.clone();

            let prec: u8 = match Self::operator_precedence(&op_token.kind) {
                Some(p) if p >= min_prec => p,
                _ => break,
            };

            self.advance();

            let right: Expression = self.parse_precedence(prec + 1)?;

            let operator: Operator = match op_token.kind {
                TokenKind::Plus => Operator::Add,
                TokenKind::Minus => Operator::Subtract,
                TokenKind::Asterisk => Operator::Multiply,
                TokenKind::Slash => Operator::Divide,
                _ => unreachable!(),
            };

            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        let token: Token = self.peek()?.clone();
        match token.kind {
            TokenKind::Integer(_) | TokenKind::Float(_) | TokenKind::String(_) => {
                self.parse_literal()
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr: Expression = self.parse_expression()?;
                self.expect_token(&TokenKind::RightParen)?;
                Ok(expr)
            }
            TokenKind::Identifier(identifier) => {
                self.advance();
                Ok(Expression::Identifier(identifier))
            }
            _ => Err(format!("Unexpected token: {:?}", token.kind)),
        }
    }

    fn parse_literal(&mut self) -> Result<Expression, String> {
        let token: Token = self.peek()?.clone();
        match &token.kind {
            TokenKind::Integer(value) => {
                self.advance();
                Ok(Expression::Literal(Literal::Integer(*value)))
            }
            TokenKind::Float(value) => {
                self.advance();
                Ok(Expression::Literal(Literal::Float(*value)))
            }
            TokenKind::String(value) => {
                self.advance();
                Ok(Expression::Literal(Literal::String(value.clone())))
            }
            TokenKind::Boolean(value) => {
                self.advance();
                Ok(Expression::Literal(Literal::Boolean(*value)))
            }
            _ => Err(format!("Expected literal, found {:?}", token.kind)),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn simple_addition() {
        // 2 + 3.4;
        let tokens: Vec<Token> = vec![
            Token::new(TokenKind::Integer(2), 0, 1),
            Token::new(TokenKind::Plus, 0, 3),
            Token::new(TokenKind::Float(3.4), 0, 5),
            Token::new(TokenKind::Semicolon, 0, 8),
            Token::new(TokenKind::EndOfFile, 0, 9),
        ];
        let program: Program = Parser::parse(tokens).unwrap();
        let expected: Program = Program {
            statements: vec![Statement::Expression(Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Integer(2))),
                operator: Operator::Add,
                right: Box::new(Expression::Literal(Literal::Float(3.4))),
            })],
        };
        assert_eq!(program, expected);
    }

    #[test]
    fn simple_subtraction() {
        // 5.0 - 1;
        let tokens: Vec<Token> = vec![
            Token::new(TokenKind::Float(5.0), 0, 1),
            Token::new(TokenKind::Minus, 0, 5),
            Token::new(TokenKind::Integer(1), 0, 7),
            Token::new(TokenKind::Semicolon, 0, 8),
            Token::new(TokenKind::EndOfFile, 0, 9),
        ];
        let program: Program = Parser::parse(tokens).unwrap();
        let expected: Program = Program {
            statements: vec![Statement::Expression(Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Float(5.0))),
                operator: Operator::Subtract,
                right: Box::new(Expression::Literal(Literal::Integer(1))),
            })],
        };
        assert_eq!(program, expected);
    }

    #[test]
    fn simple_multiplication() {
        // 4 * 2;
        let tokens: Vec<Token> = vec![
            Token::new(TokenKind::Integer(4), 0, 1),
            Token::new(TokenKind::Asterisk, 0, 3),
            Token::new(TokenKind::Integer(2), 0, 5),
            Token::new(TokenKind::Semicolon, 0, 6),
            Token::new(TokenKind::EndOfFile, 0, 7),
        ];
        let program: Program = Parser::parse(tokens).unwrap();
        let expected: Program = Program {
            statements: vec![Statement::Expression(Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Integer(4))),
                operator: Operator::Multiply,
                right: Box::new(Expression::Literal(Literal::Integer(2))),
            })],
        };
        assert_eq!(program, expected);
    }

    #[test]
    fn simple_division() {
        // 8 / 4.0;
        let tokens: Vec<Token> = vec![
            Token::new(TokenKind::Integer(8), 0, 1),
            Token::new(TokenKind::Slash, 0, 3),
            Token::new(TokenKind::Float(4.0), 0, 5),
            Token::new(TokenKind::Semicolon, 0, 8),
            Token::new(TokenKind::EndOfFile, 0, 9),
        ];
        let program: Program = Parser::parse(tokens).unwrap();
        let expected: Program = Program {
            statements: vec![Statement::Expression(Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Integer(8))),
                operator: Operator::Divide,
                right: Box::new(Expression::Literal(Literal::Float(4.0))),
            })],
        };
        assert_eq!(program, expected);
    }

    #[test]
    fn integer_literal() {
        // 42;
        let tokens: Vec<Token> = vec![
            Token::new(TokenKind::Integer(42), 0, 1),
            Token::new(TokenKind::Semicolon, 0, 3),
            Token::new(TokenKind::EndOfFile, 0, 4),
        ];
        let program: Program = Parser::parse(tokens).unwrap();
        let expected: Program = Program {
            statements: vec![Statement::Expression(Expression::Literal(
                Literal::Integer(42),
            ))],
        };
        assert_eq!(program, expected);
    }

    #[test]
    fn float_literal() {
        // 3.24;
        let tokens: Vec<Token> = vec![
            Token::new(TokenKind::Float(3.24), 0, 1),
            Token::new(TokenKind::Semicolon, 0, 5),
            Token::new(TokenKind::EndOfFile, 0, 6),
        ];
        let program: Program = Parser::parse(tokens).unwrap();
        let expected: Program = Program {
            statements: vec![Statement::Expression(Expression::Literal(Literal::Float(
                3.24,
            )))],
        };
        assert_eq!(program, expected);
    }

    #[test]
    fn parenthesized_addition() {
        // (1 + 2);
        let tokens: Vec<Token> = vec![
            Token::new(TokenKind::LeftParen, 0, 1),
            Token::new(TokenKind::Integer(1), 0, 2),
            Token::new(TokenKind::Plus, 0, 4),
            Token::new(TokenKind::Integer(2), 0, 6),
            Token::new(TokenKind::RightParen, 0, 7),
            Token::new(TokenKind::Semicolon, 0, 8),
            Token::new(TokenKind::EndOfFile, 0, 9),
        ];
        let program: Program = Parser::parse(tokens).unwrap();
        let expected: Program = Program {
            statements: vec![Statement::Expression(Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Integer(1))),
                operator: Operator::Add,
                right: Box::new(Expression::Literal(Literal::Integer(2))),
            })],
        };
        assert_eq!(program, expected);
    }

    #[test]
    fn operator_precedence() {
        // 2 + 3 * 4;
        let tokens: Vec<Token> = vec![
            Token::new(TokenKind::Integer(2), 0, 1),
            Token::new(TokenKind::Plus, 0, 3),
            Token::new(TokenKind::Float(3.3), 0, 5),
            Token::new(TokenKind::Asterisk, 0, 9),
            Token::new(TokenKind::Integer(4), 0, 11),
            Token::new(TokenKind::Semicolon, 0, 12),
            Token::new(TokenKind::EndOfFile, 0, 13),
        ];
        let program: Program = Parser::parse(tokens).unwrap();
        let expected: Program = Program {
            statements: vec![Statement::Expression(Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Integer(2))),
                operator: Operator::Add,
                right: Box::new(Expression::Binary {
                    left: Box::new(Expression::Literal(Literal::Float(3.3))),
                    operator: Operator::Multiply,
                    right: Box::new(Expression::Literal(Literal::Integer(4))),
                }),
            })],
        };
        assert_eq!(program, expected);
    }

    #[test]
    fn parenthesized_precedence() {
        // (2.7 + 3) * 4;
        let tokens: Vec<Token> = vec![
            Token::new(TokenKind::LeftParen, 0, 1),
            Token::new(TokenKind::Float(2.7), 0, 2),
            Token::new(TokenKind::Plus, 0, 6),
            Token::new(TokenKind::Integer(3), 0, 8),
            Token::new(TokenKind::RightParen, 0, 9),
            Token::new(TokenKind::Asterisk, 0, 11),
            Token::new(TokenKind::Integer(4), 0, 13),
            Token::new(TokenKind::Semicolon, 0, 14),
            Token::new(TokenKind::EndOfFile, 0, 15),
        ];
        let program: Program = Parser::parse(tokens).unwrap();
        let expected: Program = Program {
            statements: vec![Statement::Expression(Expression::Binary {
                left: Box::new(Expression::Binary {
                    left: Box::new(Expression::Literal(Literal::Float(2.7))),
                    operator: Operator::Add,
                    right: Box::new(Expression::Literal(Literal::Integer(3))),
                }),
                operator: Operator::Multiply,
                right: Box::new(Expression::Literal(Literal::Integer(4))),
            })],
        };
        assert_eq!(program, expected);
    }

    #[test]
    fn consecutive_literals() {
        // 1 2 3
        let tokens: Vec<Token> = vec![
            Token::new(TokenKind::Integer(1), 0, 1),
            Token::new(TokenKind::Integer(2), 0, 3),
            Token::new(TokenKind::Integer(3), 0, 5),
            Token::new(TokenKind::EndOfFile, 0, 6),
        ];
        let result: String = Parser::parse(tokens).err().unwrap();
        let expected_err: String = "Expected token 'Semicolon', found 'Integer(2)'".to_string();
        assert_eq!(result, expected_err);
    }

    #[test]
    fn missing_right_paren() {
        // (1 + 2
        let tokens: Vec<Token> = vec![
            Token::new(TokenKind::LeftParen, 0, 1),
            Token::new(TokenKind::Integer(1), 0, 2),
            Token::new(TokenKind::Plus, 0, 4),
            Token::new(TokenKind::Integer(2), 0, 6),
            Token::new(TokenKind::EndOfFile, 0, 7),
        ];
        let result: String = Parser::parse(tokens).err().unwrap();
        let expected_err: String = "Expected token 'RightParen', found 'EndOfFile'".to_string();
        assert_eq!(result, expected_err);
    }

    #[test]
    fn string_literal() {
        // "Hello";
        let tokens: Vec<Token> = vec![
            Token::new(TokenKind::String("Hello".to_string()), 0, 1),
            Token::new(TokenKind::Semicolon, 0, 8),
            Token::new(TokenKind::EndOfFile, 0, 9),
        ];
        let result: Program = Parser::parse(tokens).unwrap();
        let expected: Program = Program {
            statements: vec![Statement::Expression(Expression::Literal(Literal::String(
                "Hello".to_string(),
            )))],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn string_escape_sequences() {
        // "\n\u{21A0}\x45"
        let tokens: Vec<Token> = vec![
            Token::new(TokenKind::String("\n↠E".to_string()), 0, 1),
            Token::new(TokenKind::Semicolon, 0, 15),
            Token::new(TokenKind::EndOfFile, 0, 16),
        ];
        let result: Program = Parser::parse(tokens).unwrap();
        let expected: Program = Program {
            statements: vec![Statement::Expression(Expression::Literal(Literal::String(
                "\n↠E".to_string(),
            )))],
        };
        assert_eq!(result, expected);
    }
}

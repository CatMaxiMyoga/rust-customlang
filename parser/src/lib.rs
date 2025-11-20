//! Contains the parser implementation for the programming language.
#![deny(missing_docs)]
#![deny(clippy::panic)]

pub mod types;

use lexer::types::{Token, TokenKind};

use crate::types::{Expression, Literal, Operator, Program, Statement};

/// The parser struct responsible for parsing tokens into an AST.
pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    /// Creates a new parser instance with the provided tokens.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The tokens to be parsed.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, index: 0 }
    }

    /// Parses the tokens and returns the root of the AST.
    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements: Vec<Statement> = Vec::new();

        while !self.is_eof()? {
            statements.push(self.parse_statement()?);
        }

        Ok(Program { statements })
    }

    fn is_eof(&mut self) -> Result<bool, String> {
        Ok(matches!(self.peek()?.kind, TokenKind::EndOfFile))
    }

    fn peek(&mut self) -> Result<&Token, String> {
        self.tokens
            .get(self.index)
            .ok_or("Unexpected end of input".to_string())
    }

    fn advance(&mut self) {
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
        } else {
            Err(format!(
                "Expected token {:?}, found {:?}",
                kind,
                self.peek().map(|t| &t.kind)
            ))
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        Ok(Statement::Expression(self.parse_expression()?))
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_precedence(0)
    }

    fn operator_precedence(kind: &TokenKind) -> Option<u8> {
        match kind {
            TokenKind::Asterisk | TokenKind::Slash => Some(2),
            TokenKind::Plus | TokenKind::Minus => Some(1),
            _ => None,
        }
    }

    fn parse_precedence(&mut self, min_prec: u8) -> Result<Expression, String> {
        let mut left: Expression = self.parse_primary()?;

        loop {
            let next: Result<&Token, String> = self.peek();
            if next.is_err() {
                break;
            }
            let op_token: Token = next.unwrap().clone();

            let prec: u8 = match Parser::operator_precedence(&op_token.kind) {
                Some(p) if p >= min_prec => p,
                _ => break,
            };

            self.advance();

            let right: Expression = self.parse_precedence(prec + 1)?;

            let operator = match op_token.kind {
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
            TokenKind::Integer(_) | TokenKind::Float(_) => self.parse_literal(),
            TokenKind::LeftParen => {
                self.advance();
                let expr: Expression = self.parse_expression()?;
                self.expect_token(&TokenKind::RightParen)?;
                Ok(expr)
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
            _ => Err(format!("Expected literal, found {:?}", token.kind)),
        }
    }
}

#[cfg(test)]
mod tests {}

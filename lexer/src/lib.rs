//! Contains the lexer implementation for the programming language.
pub mod types;

use crate::types::{Token, TokenKind};

/// The lexer struct responsible for tokenizing the source code.
pub struct Lexer {
    source: Vec<char>,
    index: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Creates a new lexer instance with the provided source code.
    ///
    /// # Arguments
    ///
    /// * `source` - The source code to be tokenized.
    pub fn new(source: String) -> Self {
        Lexer {
            source: source.chars().collect(),
            index: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenizes the source code and returns a vector of tokens. The Lexer must be mutable to keep
    /// track of its position in the source code.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = vec![];

        'lex: while self.index < self.source.len() {
            let current_char: char = self.source[self.index];

            let single = match current_char {
                '(' => Some(TokenKind::LeftParen),
                ')' => Some(TokenKind::RightParen),
                '+' => Some(TokenKind::Plus),
                '-' => Some(TokenKind::Minus),
                '*' => Some(TokenKind::Asterisk),
                '/' => Some(TokenKind::Slash),
                ';' => Some(TokenKind::Semicolon),
                _ => None,
            };

            if let Some(kind) = single {
                tokens.push(Token::new(kind, self.line, self.column));
                self.index += 1;
                self.column += 1;
                continue 'lex;
            }

            if current_char.is_whitespace() {
                if current_char == '\n' {
                    self.line += 1;
                    self.column = 1;
                } else {
                    self.column += 1;
                }
                self.index += 1;
                continue 'lex;
            }

            let mut number_vec: Vec<char> = vec![];
            let mut dot_seen: bool = false;
            let number_start_loc: (usize, usize) = (self.line, self.column);
            'number_loop: while self.index < self.source.len()
                && (self.source[self.index].is_numeric() || self.source[self.index] == '.')
            {
                let ch: char = self.source[self.index];
                if ch == '.' {
                    if !dot_seen {
                        dot_seen = true;
                        number_vec.push(ch);
                        self.column += 1;
                        self.index += 1;
                        continue 'number_loop;
                    }
                    return Err(format!(
                        "Invalid Number Format at {}:{}",
                        self.line, self.column
                    ));
                }

                number_vec.push(ch);
                self.column += 1;
                self.index += 1;
            }

            if !number_vec.is_empty() {
                if number_vec.last().expect("Checked non-empty") == &'.' {
                    return Err(format!(
                        "Invalid Number Format at {}:{}",
                        number_start_loc.0, number_start_loc.1
                    ));
                }

                let number_str: String = number_vec.iter().collect();

                if dot_seen {
                    let float_value: f64 = number_str.parse().map_err(|_| {
                        format!(
                            "Failed to parse float '{}' at {}:{}",
                            number_str, number_start_loc.0, number_start_loc.1
                        )
                    })?;
                    tokens.push(Token::new(
                        TokenKind::Float(float_value),
                        number_start_loc.0,
                        number_start_loc.1,
                    ));
                } else {
                    let int_value: i64 = number_str.parse().map_err(|_| {
                        format!(
                            "Failed to parse integer '{}' at {}:{}",
                            number_str, number_start_loc.0, number_start_loc.1
                        )
                    })?;
                    tokens.push(Token::new(
                        TokenKind::Integer(int_value),
                        number_start_loc.0,
                        number_start_loc.1,
                    ));
                }

                continue;
            }

            return Err(format!(
                "Unknown character '{}' at {}:{}",
                current_char, self.line, self.column
            ));
        }

        tokens.push(Token::new(TokenKind::EndOfFile, self.line, self.column));

        Ok(tokens)
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn simple_integer() {
        let mut lexer: Lexer = Lexer::new(String::from("45;"));
        let tokens: Vec<Token> = lexer.tokenize().unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Integer(45), 1, 1),
            Token::new(TokenKind::Semicolon, 1, 3),
            Token::new(TokenKind::EndOfFile, 1, 4),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn simple_float() {
        let mut lexer: Lexer = Lexer::new(String::from("1.2345;"));
        let tokens: Vec<Token> = lexer.tokenize().unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Float(1.2345), 1, 1),
            Token::new(TokenKind::Semicolon, 1, 7),
            Token::new(TokenKind::EndOfFile, 1, 8),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn dot_starting_float() {
        let mut lexer: Lexer = Lexer::new(String::from(".5678;"));
        let result: Vec<Token> = lexer.tokenize().unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Float(0.5678), 1, 1),
            Token::new(TokenKind::Semicolon, 1, 6),
            Token::new(TokenKind::EndOfFile, 1, 7),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn multiple_integers() {
        let mut lexer: Lexer = Lexer::new(String::from("12; 34; 56;"));
        let tokens: Vec<Token> = lexer.tokenize().unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Integer(12), 1, 1),
            Token::new(TokenKind::Semicolon, 1, 3),
            Token::new(TokenKind::Integer(34), 1, 5),
            Token::new(TokenKind::Semicolon, 1, 7),
            Token::new(TokenKind::Integer(56), 1, 9),
            Token::new(TokenKind::Semicolon, 1, 11),
            Token::new(TokenKind::EndOfFile, 1, 12),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn multiple_floats() {
        let mut lexer: Lexer = Lexer::new(String::from("1.1; 2.2; 3.3;"));
        let tokens: Vec<Token> = lexer.tokenize().unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Float(1.1), 1, 1),
            Token::new(TokenKind::Semicolon, 1, 4),
            Token::new(TokenKind::Float(2.2), 1, 6),
            Token::new(TokenKind::Semicolon, 1, 9),
            Token::new(TokenKind::Float(3.3), 1, 11),
            Token::new(TokenKind::Semicolon, 1, 14),
            Token::new(TokenKind::EndOfFile, 1, 15),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn single_letter_tokens() {
        let mut lexer: Lexer = Lexer::new(String::from("()+-*/"));
        let tokens: Vec<Token> = lexer.tokenize().unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::LeftParen, 1, 1),
            Token::new(TokenKind::RightParen, 1, 2),
            Token::new(TokenKind::Plus, 1, 3),
            Token::new(TokenKind::Minus, 1, 4),
            Token::new(TokenKind::Asterisk, 1, 5),
            Token::new(TokenKind::Slash, 1, 6),
            Token::new(TokenKind::EndOfFile, 1, 7),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn multiline() {
        let mut lexer: Lexer = Lexer::new(String::from("314\n159"));
        let tokens: Vec<Token> = lexer.tokenize().unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Integer(314), 1, 1),
            Token::new(TokenKind::Integer(159), 2, 1),
            Token::new(TokenKind::EndOfFile, 2, 4),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn excessive_whitespace() {
        let mut lexer: Lexer = Lexer::new(String::from("  7\t\t8  \n  9 "));
        let tokens: Vec<Token> = lexer.tokenize().unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Integer(7), 1, 3),
            Token::new(TokenKind::Integer(8), 1, 6),
            Token::new(TokenKind::Integer(9), 2, 3),
            Token::new(TokenKind::EndOfFile, 2, 5),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn simple_arithmetic_expression() {
        let mut lexer: Lexer = Lexer::new(String::from("3 + 4.5 * (2 - 1) / 6"));
        let tokens: Vec<Token> = lexer.tokenize().unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Integer(3), 1, 1),
            Token::new(TokenKind::Plus, 1, 3),
            Token::new(TokenKind::Float(4.5), 1, 5),
            Token::new(TokenKind::Asterisk, 1, 9),
            Token::new(TokenKind::LeftParen, 1, 11),
            Token::new(TokenKind::Integer(2), 1, 12),
            Token::new(TokenKind::Minus, 1, 14),
            Token::new(TokenKind::Integer(1), 1, 16),
            Token::new(TokenKind::RightParen, 1, 17),
            Token::new(TokenKind::Slash, 1, 19),
            Token::new(TokenKind::Integer(6), 1, 21),
            Token::new(TokenKind::EndOfFile, 1, 22),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn invalid_number_format() {
        let mut lexer: Lexer = Lexer::new(String::from("12.34.56"));
        let result: Result<Vec<Token>, String> = lexer.tokenize();
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid Number Format at 1:6");
    }
}

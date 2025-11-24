//! Contains the lexer implementation for the programming language.
pub mod types;

use crate::types::{Keyword, Token, TokenKind};

/// The lexer struct responsible for tokenizing the source code.
pub struct Lexer {
    source: Vec<char>,
    index: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Tokenizes the source code and returns a vector of tokens.
    ///
    /// # Errors
    /// If invalid characters or number formats are encountered.
    ///
    /// # Panics
    /// Only panics if internal assumptions are violated.
    pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
        let mut lexer: Self = Self {
            source: source.chars().collect(),
            index: 0,
            line: 1,
            column: 1,
        };

        let mut tokens: Vec<Token> = vec![];

        'lex: while lexer.index < lexer.source.len() {
            let current_char: char = lexer.source[lexer.index];

            let single: Option<TokenKind> = match current_char {
                '(' => Some(TokenKind::LeftParen),
                ')' => Some(TokenKind::RightParen),
                '+' => Some(TokenKind::Plus),
                '-' => Some(TokenKind::Minus),
                '*' => Some(TokenKind::Asterisk),
                '/' => Some(TokenKind::Slash),
                ';' => Some(TokenKind::Semicolon),
                '=' => Some(TokenKind::Equals),
                _ => None,
            };

            if let Some(kind) = single {
                tokens.push(Token::new(kind, lexer.line, lexer.column));
                lexer.index += 1;
                lexer.column += 1;
                continue 'lex;
            }

            if current_char.is_whitespace() {
                if current_char == '\n' {
                    lexer.line += 1;
                    lexer.column = 1;
                } else {
                    lexer.column += 1;
                }
                lexer.index += 1;
                continue 'lex;
            }

            if lexer.multiple_char_token(&mut tokens)? {
                continue 'lex;
            }

            return Err(format!(
                "Unknown character '{}' at {}:{}",
                current_char, lexer.line, lexer.column
            ));
        }

        tokens.push(Token::new(TokenKind::EndOfFile, lexer.line, lexer.column));

        Ok(tokens)
    }

    fn multiple_char_token(&mut self, tokens: &mut Vec<Token>) -> Result<bool, String> {
        Ok(self.number(tokens)? || self.identifier(tokens) || self.string(tokens)?)
    }

    fn number(&mut self, tokens: &mut Vec<Token>) -> Result<bool, String> {
        let mut number_vec: Vec<char> = vec![];
        let mut dot_seen: bool = false;
        let number_start_loc: (usize, usize) = (self.line, self.column);
        while self.index < self.source.len()
            && (self.source[self.index].is_numeric() || self.source[self.index] == '.')
        {
            let ch: char = self.source[self.index];
            if ch == '.' {
                if !dot_seen {
                    dot_seen = true;
                    number_vec.push(ch);
                    self.column += 1;
                    self.index += 1;
                    continue;
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

            return Ok(true);
        }

        Ok(false)
    }

    fn identifier(&mut self, tokens: &mut Vec<Token>) -> bool {
        let mut identifier_vec: Vec<char> = vec![];
        let identifier_start_loc: (usize, usize) = (self.line, self.column);
        while self.index < self.source.len()
            && (self.source[self.index].is_alphanumeric() || self.source[self.index] == '_')
        {
            identifier_vec.push(self.source[self.index]);
            self.column += 1;
            self.index += 1;
        }

        if !identifier_vec.is_empty() {
            let identifier_str: String = identifier_vec.iter().collect();
            match identifier_str.as_str() {
                "let" => tokens.push(Token::new(
                    TokenKind::Keyword(Keyword::Let),
                    identifier_start_loc.0,
                    identifier_start_loc.1,
                )),
                identifier => tokens.push(Token::new(
                    TokenKind::Identifier(String::from(identifier)),
                    identifier_start_loc.0,
                    identifier_start_loc.1,
                )),
            }
            return true;
        }

        false
    }

    fn string(&mut self, tokens: &mut Vec<Token>) -> Result<bool, String> {
        if self.source[self.index] != '"' {
            return Ok(false);
        }

        let string_start_loc: (usize, usize) = (self.line, self.column);
        self.index += 1;
        self.column += 1;

        let mut string_vec: Vec<char> = vec![];
        while self.index < self.source.len() && self.source[self.index] != '"' {
            if self.source[self.index] != '\\' {
                string_vec.push(self.source[self.index]);
                self.index += 1;
                self.column += 1;
                continue;
            }

            self.index += 1;
            self.column += 1;

            let ch: char = self.source[self.index];
            match ch {
                'n' => string_vec.push('\n'),
                't' => string_vec.push('\t'),
                'r' => string_vec.push('\r'),
                'b' => string_vec.push('\x08'),
                '0' => string_vec.push('\0'),
                'f' => string_vec.push('\x0C'),
                'v' => string_vec.push('\x0B'),
                'a' => string_vec.push('\x07'),
                'u' => self.string_unicode(&mut string_vec)?,
                'x' => self.string_ascii(&mut string_vec)?,
                other => string_vec.push(other),
            }

            self.index += 1;
            self.column += 1;
        }

        dbg!(self.index, self.source.len(), &string_vec);
        if self.index >= self.source.len() {
            return Err(format!(
                "Unterminated string starting at {}:{}",
                string_start_loc.0, string_start_loc.1
            ));
        }

        self.index += 1;
        self.column += 1;

        tokens.push(Token::new(
            TokenKind::String(string_vec.iter().collect()),
            string_start_loc.0,
            string_start_loc.1,
        ));

        Ok(true)
    }

    fn string_unicode(&mut self, string_vec: &mut Vec<char>) -> Result<(), String> {
        let start_loc: (usize, usize) = (self.line - 1, self.column - 1);

        self.index += 1;
        self.column += 1;

        if self.index >= self.source.len() || self.source[self.index] != '{' {
            return Err(format!(
                "Invalid Unicode Escape at {}:{}",
                start_loc.0, start_loc.1
            ));
        }

        self.index += 1;
        self.column += 1;

        let mut unicode_seq: String = String::new();
        while self.index < self.source.len() && self.source[self.index].is_ascii_hexdigit() {
            unicode_seq.push(self.source[self.index]);
            self.index += 1;
            self.column += 1;
        }

        if self.index >= self.source.len() || self.source[self.index] != '}' {
            return Err(format!(
                "Invalid Unicode Escape at {}:{}",
                start_loc.0, start_loc.1
            ));
        }

        let unicode_code: u32 = u32::from_str_radix(&unicode_seq, 16)
            .map_err(|_| format!("Invalid Unicode Escape at {}:{}", start_loc.0, start_loc.1))?;

        std::char::from_u32(unicode_code).map_or_else(
            || {
                Err(format!(
                    "Invalid Unicode Code Point at {}:{}",
                    start_loc.0, start_loc.1
                ))
            },
            |unicode_char| {
                string_vec.push(unicode_char);
                Ok(())
            },
        )
    }

    fn string_ascii(&mut self, string_vec: &mut Vec<char>) -> Result<(), String> {
        let start_loc: (usize, usize) = (self.line - 1, self.column - 1);

        self.index += 1;
        self.column += 1;

        if self.index + 1 >= self.source.len() {
            return Err(format!(
                "Invalid Unicode Escape at {}:{}",
                start_loc.0, start_loc.1
            ));
        }

        let hex_seq: String = self.source[self.index..self.index + 2].iter().collect();

        let byte: u8 = u8::from_str_radix(&hex_seq, 16)
            .map_err(|_| format!("Invalid Unicode Escape at {}:{}", start_loc.0, start_loc.1))?;

        if byte <= 0x7F {
            string_vec.push(byte as char);
            self.index += 1;
            self.column += 1;
            Ok(())
        } else {
            Err(format!(
                "Invalid ASCII Code Point at {}:{}",
                start_loc.0, start_loc.1
            ))
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod lexer_tests {
    use super::*;

    #[test]
    fn simple_integer() {
        let result: Vec<Token> = Lexer::tokenize("45;").unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Integer(45), 1, 1),
            Token::new(TokenKind::Semicolon, 1, 3),
            Token::new(TokenKind::EndOfFile, 1, 4),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn simple_float() {
        let result: Vec<Token> = Lexer::tokenize("1.2345;").unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Float(1.2345), 1, 1),
            Token::new(TokenKind::Semicolon, 1, 7),
            Token::new(TokenKind::EndOfFile, 1, 8),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn dot_starting_float() {
        let result: Vec<Token> = Lexer::tokenize(".5678;").unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Float(0.5678), 1, 1),
            Token::new(TokenKind::Semicolon, 1, 6),
            Token::new(TokenKind::EndOfFile, 1, 7),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn multiple_integers() {
        let result: Vec<Token> = Lexer::tokenize("12; 34; 56;").unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Integer(12), 1, 1),
            Token::new(TokenKind::Semicolon, 1, 3),
            Token::new(TokenKind::Integer(34), 1, 5),
            Token::new(TokenKind::Semicolon, 1, 7),
            Token::new(TokenKind::Integer(56), 1, 9),
            Token::new(TokenKind::Semicolon, 1, 11),
            Token::new(TokenKind::EndOfFile, 1, 12),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn multiple_floats() {
        let result: Vec<Token> = Lexer::tokenize("1.1; 2.2; 3.3;").unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Float(1.1), 1, 1),
            Token::new(TokenKind::Semicolon, 1, 4),
            Token::new(TokenKind::Float(2.2), 1, 6),
            Token::new(TokenKind::Semicolon, 1, 9),
            Token::new(TokenKind::Float(3.3), 1, 11),
            Token::new(TokenKind::Semicolon, 1, 14),
            Token::new(TokenKind::EndOfFile, 1, 15),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn single_letter_tokens() {
        let result: Vec<Token> = Lexer::tokenize("()+-*/").unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::LeftParen, 1, 1),
            Token::new(TokenKind::RightParen, 1, 2),
            Token::new(TokenKind::Plus, 1, 3),
            Token::new(TokenKind::Minus, 1, 4),
            Token::new(TokenKind::Asterisk, 1, 5),
            Token::new(TokenKind::Slash, 1, 6),
            Token::new(TokenKind::EndOfFile, 1, 7),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn multiline() {
        let result: Vec<Token> = Lexer::tokenize("314\n159").unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Integer(314), 1, 1),
            Token::new(TokenKind::Integer(159), 2, 1),
            Token::new(TokenKind::EndOfFile, 2, 4),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn excessive_whitespace() {
        let result: Vec<Token> = Lexer::tokenize("  7\t\t8  \n  9 ").unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Integer(7), 1, 3),
            Token::new(TokenKind::Integer(8), 1, 6),
            Token::new(TokenKind::Integer(9), 2, 3),
            Token::new(TokenKind::EndOfFile, 2, 5),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn simple_arithmetic_expression() {
        let result: Vec<Token> = Lexer::tokenize("3 + 4.5 * (2 - 1) / 6").unwrap();
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
        assert_eq!(result, expected);
    }

    #[test]
    fn invalid_number_format() {
        let result: Result<Vec<Token>, String> = Lexer::tokenize("12.34.56");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid Number Format at 1:6");
    }

    #[test]
    fn identifier() {
        let result: Vec<Token> = Lexer::tokenize("Hello").unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Identifier(String::from("Hello")), 1, 1),
            Token::new(TokenKind::EndOfFile, 1, 6),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn identifier_with_number() {
        let result: Vec<Token> = Lexer::tokenize("var123").unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Identifier(String::from("var123")), 1, 1),
            Token::new(TokenKind::EndOfFile, 1, 7),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn keyword_let() {
        let result: Vec<Token> = Lexer::tokenize("let").unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Keyword(Keyword::Let), 1, 1),
            Token::new(TokenKind::EndOfFile, 1, 4),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn variable_assignment() {
        let result: Vec<Token> = Lexer::tokenize("let x = 10;").unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Keyword(Keyword::Let), 1, 1),
            Token::new(TokenKind::Identifier(String::from("x")), 1, 5),
            Token::new(TokenKind::Equals, 1, 7),
            Token::new(TokenKind::Integer(10), 1, 9),
            Token::new(TokenKind::Semicolon, 1, 11),
            Token::new(TokenKind::EndOfFile, 1, 12),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn string_literal() {
        let result: Vec<Token> = Lexer::tokenize(r#""Hello, World!""#).unwrap();
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::String(String::from("Hello, World!")), 1, 1),
            Token::new(TokenKind::EndOfFile, 1, 16),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn string_escape_sequences() {
        let result: Vec<Token> = Lexer::tokenize(r#""\n\t\r\b\0\f\v\a\u{21A0}\x45\\\"""#).unwrap();
        let expected: Vec<Token> = vec![
            Token::new(
                TokenKind::String(String::from("\n\t\r\x08\0\x0C\x0B\x07↠E\\\"")),
                1,
                1,
            ),
            Token::new(TokenKind::EndOfFile, 1, 35),
        ];
        assert_eq!(result, expected);
    }
}

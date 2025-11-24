//! Contains the types used in the lexer implementation.

/// Defines the different keywords recognized by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    /// Let keyword, used for variable declarations.
    Let,
}

/// Defines the different kinds of tokens that can be recognized by the lexer. Each variant may 
/// hold associated data relevant to that token type.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// Represents an integer literal.
    Integer(i64),
    /// Represents a float literal.
    Float(f64),
    /// Represents a string literal.
    String(String),
    /// Represents a boolean literal. [true | false]
    Boolean(bool),
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Asterisk,
    /// /
    Slash,
    /// (
    LeftParen,
    /// )
    RightParen,
    /// =
    Equals,
    /// ;
    Semicolon,
    /// Represents an identifier.
    Identifier(String),
    /// Represents a keyword.
    Keyword(Keyword),
    /// Represents the end of the source code.
    EndOfFile,
}

/// Represents a token with its kind and position in the source code. The kind contains the value.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The kind (and potential value) of the token.
    pub kind: TokenKind,
    /// The line number where the token or the start of the token is located.
    pub line: usize,
    /// The column number where the token or the start of the token is located.
    pub column: usize,
}

impl Token {
    /// Creates a new token with the specified kind, line, and column.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of the token.
    /// * `line` - The line number where the token is located.
    /// * `column` - The column number where the token is located.
    #[must_use]
    pub const fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }
}

//! Contains types used throughout the parser module.

/// Contains literal values in the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// An integer literal.
    Integer(i64),
    /// A floating-point literal.
    Float(f64),
}

/// Represents operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    /// Represents binary addition.
    Add,
    /// Represents binary subtraction.
    Subtract,
    /// Represents binary multiplication.
    Multiply,
    /// Represents binary division.
    Divide,
}

/// Represents expressions in the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// A literal expression.
    Literal(Literal),
    /// A binary expression.
    Binary {
        /// The left-hand side expression.
        left: Box<Expression>,
        /// The operator.
        operator: Operator,
        /// The right-hand side expression.
        right: Box<Expression>,
    },
}

/// Represents statements in the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// An expression statement.
    Expression(Expression),
}

/// The root node of the AST.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    /// A list of statements in the program.
    pub statements: Vec<Statement>,
}

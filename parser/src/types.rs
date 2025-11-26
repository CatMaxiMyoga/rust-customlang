//! Contains types used throughout the parser module.

/// Contains literal values in the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// An integer literal.
    Integer(i64),
    /// A floating-point literal.
    Float(f64),
    /// A string literal.
    String(String),
    /// A boolean literal.
    Boolean(bool),
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
    /// An identifier expression.
    Identifier(String),
    /// A binary expression.
    Binary {
        /// The left-hand side expression.
        left: Box<Expression>,
        /// The operator.
        operator: Operator,
        /// The right-hand side expression.
        right: Box<Expression>,
    },
    /// A function call expression.
    FunctionCall {
        /// The name of the function being called.
        name: String,
        /// The arguments passed to the function.
        arguments: Vec<Expression>,
    }
}

/// Represents statements in the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// A variable declaration statement.
    VariableDeclaration {
        /// The type of the variable.
        type_: String,
        /// The name of the variable.
        name: String,
        /// The initial value of the variable.
        value: Option<Expression>,
    },
    /// Variable assignment statement.
    VariableAssignment {
        /// The name of the variable.
        name: String,
        /// The new value of the variable.
        value: Expression,
    },
    /// A function declaration statement.
    FunctionDeclaration {
        /// The return type of the function.
        return_type: String,
        /// The name of the function.
        name: String,
        /// The parameters of the function `(Type, Identifier)`.
        parameters: Vec<(String, String)>,
        /// The body of the function.
        body: Vec<Statement>,
    },
    /// A return statement.
    Return(Expression),
    /// An expression statement.
    Expression(Expression),
}

/// The root node of the AST.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    /// A list of statements in the program.
    pub statements: Vec<Statement>,
}

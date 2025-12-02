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

/// Represents binary operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperator {
    /// Represents binary addition.
    Add,
    /// Represents binary subtraction.
    Subtract,
    /// Represents binary multiplication.
    Multiply,
    /// Represents binary division.
    Divide,
    /// Represents equality comparison.
    Equals,
    /// Represents inequality comparison.
    NotEquals,
    /// Represents less than comparison.
    LessThan,
    /// Represents greater than comparison.
    GreaterThan,
    /// Represents less than or equal comparison.
    LessThanOrEqual,
    /// Represents greater than or equal comparison.
    GreaterThanOrEqual,
    /// Represents logical AND operation.
    And,
    /// Represents logical OR operation.
    Or,
}

/// Represents unary operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperator {
    /// Represents logical NOT operation.
    Not,
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
        left: Box<Expr>,
        /// The operator.
        operator: BinaryOperator,
        /// The right-hand side expression.
        right: Box<Expr>,
    },
    /// A unary expression.
    Unary {
        /// The operator.
        operator: UnaryOperator,
        /// The operand expression.
        operand: Box<Expr>,
    },
    /// A function call expression.
    FunctionCall {
        /// The name of the function being called.
        name: String,
        /// The arguments passed to the function.
        arguments: Vec<Expr>,
    },
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
        value: Option<Expr>,
    },
    /// Variable assignment statement.
    VariableAssignment {
        /// The name of the variable.
        name: String,
        /// The new value of the variable.
        value: Expr,
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
        body: Vec<Stmt>,
    },
    /// If statement.
    If {
        /// Conditional branches.
        conditional_branches: Vec<(Expr, Vec<Stmt>)>,
        /// The block to execute if the condition is false.
        else_branch: Option<Vec<Stmt>>,
    },
    /// A return statement.
    Return(Expr),
    /// An expression statement.
    Expression(Expr),
}

/// The root node of the AST.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    /// A list of statements in the program.
    pub statements: Vec<Stmt>,
}

/// Represents the starting and ending position of a node in the source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    /// The starting position (line, column).
    pub start: (usize, usize),
    /// The ending position (line, column).
    pub end: (usize, usize),
}

/// A node with its associated span in the source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> {
    /// The value with its span.
    pub node: T,
    /// The span of the node.
    pub span: Span,
}

/// Spanned statement
pub type Stmt = Spanned<Statement>;
/// Spanned expression
pub type Expr = Spanned<Expression>;

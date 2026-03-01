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
    /// A function/member call expression.
    Call {
        /// The callee being called.
        callee: Box<Expr>,
        /// The arguments passed to the function.
        arguments: Vec<Expr>,
    },
    /// A member access expression.
    MemberAccess {
        /// The object whose member is being accessed.
        object: Box<Expr>,
        /// The name of the member being accessed.
        member: String,
    },
    /// Special expression representing the current class instance.
    Self_,
}

impl Expression {
    /// Returns the name of the expression variant.
    #[must_use]
    pub const fn name(&self) -> &str {
        match self {
            Self::Literal(_) => "Literal",
            Self::Identifier(_) => "Identifier",
            Self::Binary { .. } => "Binary",
            Self::Unary { .. } => "Unary",
            Self::Call { .. } => "Call",
            Self::MemberAccess { .. } => "MemberAccess",
            Self::Self_ => "Self",
        }
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
        value: Option<Expr>,
    },
    /// A field declaration statement.
    FieldDeclaration {
        /// The type of the field.
        type_: String,
        /// The name of the field.
        name: String,
        /// Static field or not.
        static_: bool,
    },
    /// Variable/Member assignment statement.
    Assignment {
        /// The variable being assigned to.
        assignee: Box<Expr>,
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
    /// A class declaration statement.
    ClassDeclaration {
        /// The name of the class.
        name: String,
        /// The body of the class.
        body: Vec<Stmt>,
    },
    /// A method declaration statement.
    MethodDeclaration {
        /// The return type of the function.
        return_type: String,
        /// The name of the function.
        name: String,
        /// The parameters of the function `(Type, Identifier)`.
        parameters: Vec<(String, String)>,
        /// The body of the function.
        body: Vec<Stmt>,
        /// Static method or not.
        static_: bool,
    },
    /// If statement.
    If {
        /// Conditional branches.
        conditional_branches: Vec<(Expr, Vec<Stmt>)>,
        /// The block to execute if the condition is false.
        else_branch: Option<Vec<Stmt>>,
    },
    /// While loop statement.
    While {
        /// The condition expression.
        condition: Expr,
        /// The body of the while loop.
        body: Vec<Stmt>,
    },
    /// A return statement.
    Return(Option<Expr>),
    /// An expression statement.
    Expression(Expr),
}

impl Statement {
    /// Returns the name of the statement variant.
    #[must_use]
    pub const fn name(&self) -> &str {
        match self {
            Self::VariableDeclaration { .. } => "VariableDeclaration",
            Self::FieldDeclaration { .. } => "FieldDeclaration",
            Self::Assignment { .. } => "Assignment",
            Self::FunctionDeclaration { .. } => "FunctionDeclaration",
            Self::ClassDeclaration { .. } => "ClassDeclaration",
            Self::MethodDeclaration { .. } => "MethodDeclaration",
            Self::If { .. } => "If",
            Self::While { .. } => "While",
            Self::Return(_) => "Return",
            Self::Expression(_) => "Expression",
        }
    }
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

//! Contains types used throughout the interpreter.

use std::ops::{Add, Div, Mul, Sub};

/// Represents the environment mapping variable names to their values.
#[derive(Debug, Clone, Default)]
pub struct Scope {
    /// A mapping of variable names to their corresponding types and values `(Type, Value)`
    pub variables: std::collections::HashMap<Identifier, (Type, Option<RuntimeValue>)>,
    /// An optional reference to the parent scope for nested scopes.
    pub parent: Option<Box<Scope>>,
}

impl Scope {
    /// Creates a new scope with a reference to a parent scope.
    #[must_use]
    pub fn with_parent(parent: Self) -> Self {
        Self {
            variables: std::collections::HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    /// Recursively searches for a variable in parent scopes.
    #[must_use]
    pub fn find_in_parent(&self, name: &str) -> Option<&(Type, Option<RuntimeValue>)> {
        self.parent.as_ref().and_then(|parent| {
            parent
                .variables
                .get(name)
                .map_or_else(|| parent.find_in_parent(name), Some)
        })
    }
}


/// Represents the result of a runtime operation returning a value
pub type ExpressionResult = Result<RuntimeValue, RuntimeError>;

/// Represents the result of a statement at runtime
pub type StatementResult = Result<(), RuntimeError>;

// Semantic type aliases
/// Represents an identifier (variable or function name).
pub type Identifier = String;
/// Represents a parameter name in function definitions.
pub type ParamName = String;
/// Represents a type defined in builtin functions.
pub type StrType = &'static str;

/// Represents a type in the interpreter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type(pub String);
impl Type {
    /// Creates a new `Type` if the provided type string is valid.
    /// 
    /// # Errors
    /// `InvalidType` if the provided type string is not one of the valid types.
    pub fn new(type_: &str) -> Result<Self, RuntimeError> {
        const VALID_TYPES: [&str; 5] =
            ["Integer", "Float", "String", "Boolean", "Void"];

        if !VALID_TYPES.contains(&type_) {
            return Err(RuntimeError::InvalidType(type_.to_string()));
        }

        Ok(Self(type_.to_string()))
    }
}

/// Represents runtime errors that can occur during interpretation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    /// Division by zero error.
    ///
    /// # Example
    /// ```ignore
    /// 5 / 0;
    /// ```
    DivisionByZero,

    /// Type mismatch error. Holds a message describing the mismatch.
    ///
    /// # Example
    /// ```ignore
    /// Integer x = 5;
    /// x = 5.2;
    /// ```
    TypeMismatch(String),

    /// Illegal operation error. Holds a message describing the illegal operation.
    ///
    /// # Example
    /// ```ignore
    /// 5 + "Hello";
    /// >> "Cannot add Integer with non-numeric type"
    /// ```
    IllegalOperation(String),

    /// Variable not found error. Holds the name of the missing variable.
    ///
    /// # Example
    /// ```ignore
    /// Integer x = y + 5;
    /// >> "y"
    /// ```
    VariableNotFound(Identifier),

    /// Variable uninitialized error. Holds the name of the uninitialized variable.
    ///
    /// # Example
    /// ```ignore
    /// Integer x;
    /// x + 5;
    /// >> "x"
    /// ```
    VariableUninitialized(Identifier),

    /// Name conflict error. Holds a message containing specifics.
    ///
    /// # Example
    /// ```ignore
    /// Integer x = 5;
    /// fn x() {}
    /// >> "Cannot create function 'x', identifier already exists in current scope."
    /// ```
    NameConflict(String),

    /// Illegal argument error. Holds the amount of given arguments.
    ///
    /// # Example
    /// ```ignore
    /// fn add(a, b) {}
    /// add(5);
    /// >> 1
    /// ```
    IllegalArgumentCount(usize),

    /// Illegal return error. Return statement used outside of a function.
    ///
    /// # Example
    /// ```ignore
    /// return 5;
    /// ```
    IllegalReturn,

    /// Invalid type error. Holds the identifier of the invalid type.
    ///
    /// # Exmaple
    /// ```ignore
    /// something x = 5;
    /// >> "something"
    /// ```
    InvalidType(Identifier),
}

trait Operations {
    // Arithmetic operations
    fn add(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult;
    fn sub(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult;
    fn mul(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult;
    fn div(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult;

    // Comparison operations
    fn eq(&self, lsh: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult;
    fn ne(&self, lsh: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult;
    fn lt(&self, lsh: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult;
    fn le(&self, lsh: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult;
    fn gt(&self, lsh: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult;
    fn ge(&self, lsh: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult;
}

/// Represents all types of values Expressions can return when evaluated.
#[derive(Debug, Clone, PartialEq)]
#[allow(unpredictable_function_pointer_comparisons)]
pub enum RuntimeValue {
    /// An integer value.
    Integer(i64),
    /// A floating-point value.
    Float(f64),
    /// A string value.
    String(String),
    /// A boolean value.
    Boolean(bool),
    /// A function value.
    Function {
        /// The parameter names of the function.
        parameters: Vec<(Type, ParamName)>,
        /// The body of the function as a list of statements.
        body: Vec<parser::types::Statement>,
    },
    /// A builtin function value.
    BuiltinFunction {
        /// The parameter types of the builtin function.
        parameters: Vec<StrType>,
        /// The implementation of the builtin function.
        implementation: fn(&mut Scope, Vec<RuntimeValue>) -> ExpressionResult,
    },
    /// Represents no value (void).
    Void,
}

impl RuntimeValue {
    /// Returns the name of the type of the `RuntimeValue` as a string slice.
    #[must_use]
    pub const fn get_name(&self) -> &'static str {
        match self {
            Self::Integer(_) => "Integer",
            Self::Float(_) => "Float",
            Self::String(_) => "String",
            Self::Boolean(_) => "Boolean",
            Self::Function { .. } => "Function",
            Self::BuiltinFunction { .. } => "Builtin (Function)",
            Self::Void => "Void",
        }
    }

    const fn ops(&self) -> &dyn Operations {
        match self {
            Self::Integer(_) => &IntegerOperations,
            Self::Float(_) => &FloatOperations,
            Self::String(_) => &StringOperations,
            Self::Boolean(_) => &NoOperations { name: "Boolean" },
            Self::Function { .. } => &NoOperations { name: "Function" },
            Self::BuiltinFunction { .. } => &NoOperations {
                name: "Builtin (Function)",
            },
            Self::Void => &NoOperations { name: "Void" },
        }
    }

    /// `==`
    pub(crate) fn eq(&self, rhs: &Self) -> ExpressionResult {
        self.ops().eq(self, rhs)
    }

    /// `!=`
    pub(crate) fn ne(&self, rhs: &Self) -> ExpressionResult {
        self.ops().ne(self, rhs)
    }

    /// `<`
    pub(crate) fn lt(&self, rhs: &Self) -> ExpressionResult {
        self.ops().lt(self, rhs)
    }

    /// `<=`
    pub(crate) fn le(&self, rhs: &Self) -> ExpressionResult {
        self.ops().le(self, rhs)
    }

    /// `>`
    pub(crate) fn gt(&self, rhs: &Self) -> ExpressionResult {
        self.ops().gt(self, rhs)
    }

    /// `>=`
    pub(crate) fn ge(&self, rhs: &Self) -> ExpressionResult {
        self.ops().ge(self, rhs)
    }
}

impl Add for RuntimeValue {
    type Output = ExpressionResult;
    fn add(self, rhs: Self) -> Self::Output {
        self.ops().add(&self, &rhs)
    }
}

impl Sub for RuntimeValue {
    type Output = ExpressionResult;
    fn sub(self, rhs: Self) -> Self::Output {
        self.ops().sub(&self, &rhs)
    }
}

impl Mul for RuntimeValue {
    type Output = ExpressionResult;
    fn mul(self, rhs: Self) -> Self::Output {
        self.ops().mul(&self, &rhs)
    }
}

impl Div for RuntimeValue {
    type Output = ExpressionResult;
    fn div(self, rhs: Self) -> Self::Output {
        self.ops().div(&self, &rhs)
    }
}

struct IntegerOperations;
impl Operations for IntegerOperations {
    fn add(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Integer(lhs + rhs)),
            #[allow(clippy::cast_precision_loss)]
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Float(*lhs as f64 + rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot add Integer with non-numeric type".to_string(),
            )),
        }
    }

    fn sub(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Integer(lhs - rhs)),
            #[allow(clippy::cast_precision_loss)]
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Float(*lhs as f64 - rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot subtract Integer with non-numeric type".to_string(),
            )),
        }
    }

    fn mul(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Integer(lhs * rhs)),
            #[allow(clippy::cast_precision_loss)]
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Float(*lhs as f64 * rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot multiply Integer with non-numeric type".to_string(),
            )),
        }
    }

    fn div(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Integer(rhs) => {
                if *rhs == 0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(RuntimeValue::Integer(lhs / rhs))
                }
            }
            RuntimeValue::Float(rhs) => {
                if *rhs == 0.0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    #[allow(clippy::cast_precision_loss)]
                    Ok(RuntimeValue::Float(*lhs as f64 / rhs))
                }
            }
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot divide Integer with non-numeric type".to_string(),
            )),
        }
    }

    fn eq(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Boolean(lhs == rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot compare Integer with non-Integer type".to_string(),
            )),
        }
    }

    fn ne(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Boolean(lhs != rhs)),
            _ => Err(RuntimeError::IllegalOperation(
            "Cannot compare Integer with non-Integer type".to_string(),
            )),
        }
    }

    fn lt(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Boolean(lhs < rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot compare Integer with non-Integer type".to_string(),
            )),
        }
    }

    fn le(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Boolean(lhs <= rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot compare Integer with non-Integer type".to_string(),
            )),
        }
    }

    fn gt(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Boolean(lhs > rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot compare Integer with non-Integer type".to_string(),
            )),
        }
    }

    fn ge(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Boolean(lhs >= rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot compare Integer with non-Integer type".to_string(),
            )),
        }
    }
}

struct FloatOperations;
impl Operations for FloatOperations {
    fn add(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            #[allow(clippy::cast_precision_loss)]
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Float(lhs + *rhs as f64)),
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Float(lhs + rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot add Float with non-numeric type".to_string(),
            )),
        }
    }

    fn sub(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            #[allow(clippy::cast_precision_loss)]
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Float(lhs - *rhs as f64)),
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Float(lhs - rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot subtract Float with non-numeric type".to_string(),
            )),
        }
    }

    fn mul(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            #[allow(clippy::cast_precision_loss)]
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Float(lhs * *rhs as f64)),
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Float(lhs * rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot multiply Float with non-numeric type".to_string(),
            )),
        }
    }

    fn div(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Integer(rhs) => {
                if *rhs == 0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    #[allow(clippy::cast_precision_loss)]
                    Ok(RuntimeValue::Float(lhs / *rhs as f64))
                }
            }
            RuntimeValue::Float(rhs) => {
                if *rhs == 0.0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(RuntimeValue::Float(lhs / rhs))
                }
            }
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot divide Float with non-numeric type".to_string(),
            )),
        }
    }

    fn eq(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Boolean(lhs == rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot compare Float with non-Float type".to_string(),
            )),
        }
    }

    fn ne(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Boolean(lhs != rhs)),
            _ => Err(RuntimeError::IllegalOperation(
            "Cannot compare Float with non-Float type".to_string(),
            )),
        }
    }

    fn lt(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Boolean(lhs < rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot compare Float with non-Float type".to_string(),
            )),
        }
    }

    fn le(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Boolean(lhs <= rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot compare Float with non-Float type".to_string(),
            )),
        }
    }

    fn gt(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Boolean(lhs > rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot compare Float with non-Float type".to_string(),
            )),
        }
    }

    fn ge(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Boolean(lhs >= rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot compare Float with non-Float type".to_string(),
            )),
        }
    }
}

struct StringOperations;
impl StringOperations {
    const NOOP: NoOperations = NoOperations { name: "String" };
}
impl Operations for StringOperations {
    fn add(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::String(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::String(rhs) => Ok(RuntimeValue::String(lhs.clone() + rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot add String with non-String type".to_string(),
            )),
        }
    }

    fn sub(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        Self::NOOP.sub(lhs, rhs)
    }

    fn mul(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        Self::NOOP.mul(lhs, rhs)
    }

    fn div(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        Self::NOOP.div(lhs, rhs)
    }

    fn eq(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::String(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::String(rhs) => Ok(RuntimeValue::Boolean(lhs == rhs)),
            _ => Err(RuntimeError::IllegalOperation(
                "Cannot compare String with non-String type".to_string(),
            )),
        }
    }

    fn ne(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        let RuntimeValue::String(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::String(rhs) => Ok(RuntimeValue::Boolean(lhs != rhs)),
            _ => Err(RuntimeError::IllegalOperation(
            "Cannot compare String with non-String type".to_string(),
            )),
        }
    }

    fn lt(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        Self::NOOP.lt(lhs, rhs)
    }

    fn le(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        Self::NOOP.le(lhs, rhs)
    }

    fn gt(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        Self::NOOP.gt(lhs, rhs)
    }

    fn ge(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> ExpressionResult {
        Self::NOOP.ge(lhs, rhs)
    }
}

struct NoOperations {
    name: &'static str,
}
impl Operations for NoOperations {
    fn add(&self, _lhs: &RuntimeValue, _rhs: &RuntimeValue) -> ExpressionResult {
        Err(RuntimeError::IllegalOperation(format!(
            "Addition not supported for {} type",
            self.name
        )))
    }
    fn sub(&self, _lhs: &RuntimeValue, _rhs: &RuntimeValue) -> ExpressionResult {
        Err(RuntimeError::IllegalOperation(format!(
            "Subtraction not supported for {} type",
            self.name
        )))
    }
    fn mul(&self, _lhs: &RuntimeValue, _rhs: &RuntimeValue) -> ExpressionResult {
        Err(RuntimeError::IllegalOperation(format!(
            "Multiplication not supported for {} type",
            self.name
        )))
    }
    fn div(&self, _lhs: &RuntimeValue, _rhs: &RuntimeValue) -> ExpressionResult {
        Err(RuntimeError::IllegalOperation(format!(
            "Division not supported for {} type",
            self.name
        )))
    }
    fn eq(&self, _lhs: &RuntimeValue, _rhs: &RuntimeValue) -> ExpressionResult {
        Err(RuntimeError::IllegalOperation(format!(
            "Comparison '==' not supported for {} type",
            self.name
        )))
    }
    fn ne(&self, _lhs: &RuntimeValue, _rhs: &RuntimeValue) -> ExpressionResult {
        Err(RuntimeError::IllegalOperation(format!(
            "Comparison '!=' not supported for {} type",
            self.name
        )))
    }
    fn lt(&self, _lhs: &RuntimeValue, _rhs: &RuntimeValue) -> ExpressionResult {
        Err(RuntimeError::IllegalOperation(format!(
            "Comparison '<' not supported for {} type",
            self.name
        )))
    }
    fn le(&self, _lhs: &RuntimeValue, _rhs: &RuntimeValue) -> ExpressionResult {
        Err(RuntimeError::IllegalOperation(format!(
            "Comparison '<=' not supported for {} type",
            self.name
        )))
    }
    fn gt(&self, _lhs: &RuntimeValue, _rhs: &RuntimeValue) -> ExpressionResult {
        Err(RuntimeError::IllegalOperation(format!(
            "Comparison '>' not supported for {} type",
            self.name
        )))
    }
    fn ge(&self, _lhs: &RuntimeValue, _rhs: &RuntimeValue) -> ExpressionResult {
        Err(RuntimeError::IllegalOperation(format!(
            "Comparison '>=' not supported for {} type",
            self.name
        )))
    }
}

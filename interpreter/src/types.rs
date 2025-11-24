//! Contains types used throughout the interpreter.

use std::ops::{Add, Div, Mul, Sub};

/// Represents the environment mapping variable names to their values.
pub type Environment = std::collections::HashMap<String, Option<RuntimeValue>>;

type Identifier = String;

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

    /// Type mismatch error.
    ///
    /// # Example
    /// ```ignore
    /// let x = 5;
    /// x = 5.2;
    /// ```
    TypeMismatch,

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
    /// let x = y + 5;
    /// >> "y"
    /// ```
    VaiableNotFound(Identifier),

    /// Variable uninitialized error. Holds the name of the uninitialized variable.
    ///
    /// # Example
    /// ```ignore
    /// let x;
    /// x + 5;
    /// >> "x"
    /// ```
    VariableUninitialized(Identifier),
}

/// Represents the result of a runtime operation returning a value
pub type RuntimeResult = Result<RuntimeValue, RuntimeError>;

trait Operations {
    fn add(&self, lsh: &RuntimeValue, rhs: &RuntimeValue) -> RuntimeResult;
    fn sub(&self, lsh: &RuntimeValue, rhs: &RuntimeValue) -> RuntimeResult;
    fn mul(&self, lsh: &RuntimeValue, rhs: &RuntimeValue) -> RuntimeResult;
    fn div(&self, lsh: &RuntimeValue, rhs: &RuntimeValue) -> RuntimeResult;
}

/// Represents all types of values Expressions can return when evaluated.
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    /// An integer value.
    Integer(i64),
    /// A floating-point value.
    Float(f64),
    /// A string value.
    String(String),
}

impl RuntimeValue {
    fn ops(&self) -> &dyn Operations {
        match self {
            Self::Integer(_) => &IntegerOperations,
            Self::Float(_) => &FloatOperations,
            Self::String(_) => &StringOperations,
        }
    }
}

impl Add for RuntimeValue {
    type Output = RuntimeResult;
    fn add(self, rhs: Self) -> Self::Output {
        self.ops().add(&self, &rhs)
    }
}

impl Sub for RuntimeValue {
    type Output = RuntimeResult;
    fn sub(self, rhs: Self) -> Self::Output {
        self.ops().sub(&self, &rhs)
    }
}

impl Mul for RuntimeValue {
    type Output = RuntimeResult;
    fn mul(self, rhs: Self) -> Self::Output {
        self.ops().mul(&self, &rhs)
    }
}

impl Div for RuntimeValue {
    type Output = RuntimeResult;
    fn div(self, rhs: Self) -> Self::Output {
        self.ops().div(&self, &rhs)
    }
}

struct IntegerOperations;
impl Operations for IntegerOperations {
    fn add(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> RuntimeResult {
        let RuntimeValue::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Integer(lhs + rhs)),
            #[allow(clippy::cast_precision_loss)]
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Float(*lhs as f64 + rhs)),
            RuntimeValue::String(_) => Err(RuntimeError::IllegalOperation(
                "Cannot add Integer with non-numeric type".to_string(),
            )),
        }
    }

    fn sub(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> RuntimeResult {
        let RuntimeValue::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Integer(lhs - rhs)),
            #[allow(clippy::cast_precision_loss)]
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Float(*lhs as f64 - rhs)),
            RuntimeValue::String(_) => Err(RuntimeError::IllegalOperation(
                "Cannot subtract Integer with non-numeric type".to_string(),
            )),
        }
    }

    fn mul(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> RuntimeResult {
        let RuntimeValue::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Integer(lhs * rhs)),
            #[allow(clippy::cast_precision_loss)]
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Float(*lhs as f64 * rhs)),
            RuntimeValue::String(_) => Err(RuntimeError::IllegalOperation(
                "Cannot multiply Integer with non-numeric type".to_string(),
            )),
        }
    }

    fn div(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> RuntimeResult {
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
            RuntimeValue::String(_) => Err(RuntimeError::IllegalOperation(
                "Cannot divide Integer with non-numeric type".to_string(),
            )),
        }
    }
}

struct FloatOperations;
impl Operations for FloatOperations {
    fn add(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> RuntimeResult {
        let RuntimeValue::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            #[allow(clippy::cast_precision_loss)]
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Float(lhs + *rhs as f64)),
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Float(lhs + rhs)),
            RuntimeValue::String(_) => Err(RuntimeError::IllegalOperation(
                "Cannot add Float with non-numeric type".to_string(),
            )),
        }
    }

    fn sub(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> RuntimeResult {
        let RuntimeValue::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            #[allow(clippy::cast_precision_loss)]
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Float(lhs - *rhs as f64)),
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Float(lhs - rhs)),
            RuntimeValue::String(_) => Err(RuntimeError::IllegalOperation(
                "Cannot subtract Float with non-numeric type".to_string(),
            )),
        }
    }

    fn mul(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> RuntimeResult {
        let RuntimeValue::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            #[allow(clippy::cast_precision_loss)]
            RuntimeValue::Integer(rhs) => Ok(RuntimeValue::Float(lhs * *rhs as f64)),
            RuntimeValue::Float(rhs) => Ok(RuntimeValue::Float(lhs * rhs)),
            RuntimeValue::String(_) => Err(RuntimeError::IllegalOperation(
                "Cannot multiply Float with non-numeric type".to_string(),
            )),
        }
    }

    fn div(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> RuntimeResult {
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
            RuntimeValue::String(_) => Err(RuntimeError::IllegalOperation(
                "Cannot divide Float with non-numeric type".to_string(),
            )),
        }
    }
}

struct StringOperations;
impl Operations for StringOperations {
    fn add(&self, lhs: &RuntimeValue, rhs: &RuntimeValue) -> RuntimeResult {
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

    fn sub(&self, _lhs: &RuntimeValue, _rhs: &RuntimeValue) -> RuntimeResult {
        Err(RuntimeError::IllegalOperation(
            "Subtraction not supported for String type".to_string(),
        ))
    }

    fn mul(&self, _lhs: &RuntimeValue, _rhs: &RuntimeValue) -> RuntimeResult {
        Err(RuntimeError::IllegalOperation(
            "Multiplication not supported for String type".to_string(),
        ))
    }

    fn div(&self, _lhs: &RuntimeValue, _rhs: &RuntimeValue) -> RuntimeResult {
        Err(RuntimeError::IllegalOperation(
            "Division not supported for String type".to_string(),
        ))
    }
}

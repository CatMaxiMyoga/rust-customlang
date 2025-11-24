//! Contains types used throughout the interpreter.

use std::ops::{Add, Div, Mul, Sub};

/// Represents the environment mapping variable names to their values.
pub type Environment = std::collections::HashMap<String, Option<Value>>;

trait Operations {
    fn add(&self, lsh: &Value, rhs: &Value) -> Result<Value, String>;
    fn sub(&self, lsh: &Value, rhs: &Value) -> Result<Value, String>;
    fn mul(&self, lsh: &Value, rhs: &Value) -> Result<Value, String>;
    fn div(&self, lsh: &Value, rhs: &Value) -> Result<Value, String>;
}

/// Represents all types of values Expressions can return when evaluated.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// An integer value.
    Integer(i64),
    /// A floating-point value.
    Float(f64),
    /// A string value.
    String(String),
}

impl Value {
    fn ops(&self) -> &dyn Operations {
        match self {
            Self::Integer(_) => &IntegerOperations,
            Self::Float(_) => &FloatOperations,
            Self::String(_) => &StringOperations,
        }
    }
}

impl Add for Value {
    type Output = Result<Self, String>;
    fn add(self, rhs: Self) -> Self::Output {
        self.ops().add(&self, &rhs)
    }
}

impl Sub for Value {
    type Output = Result<Self, String>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.ops().sub(&self, &rhs)
    }
}

impl Mul for Value {
    type Output = Result<Self, String>;
    fn mul(self, rhs: Self) -> Self::Output {
        self.ops().mul(&self, &rhs)
    }
}

impl Div for Value {
    type Output = Result<Self, String>;
    fn div(self, rhs: Self) -> Self::Output {
        self.ops().div(&self, &rhs)
    }
}

struct IntegerOperations;
impl Operations for IntegerOperations {
    fn add(&self, lhs: &Value, rhs: &Value) -> Result<Value, String> {
        let Value::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            Value::Integer(rhs) => Ok(Value::Integer(lhs + rhs)),
            #[allow(clippy::cast_precision_loss)]
            Value::Float(rhs) => Ok(Value::Float(*lhs as f64 + rhs)),
            Value::String(_) => Err("Cannot add Integer with non-numeric type".to_string()),
        }
    }

    fn sub(&self, lhs: &Value, rhs: &Value) -> Result<Value, String> {
        let Value::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            Value::Integer(rhs) => Ok(Value::Integer(lhs - rhs)),
            #[allow(clippy::cast_precision_loss)]
            Value::Float(rhs) => Ok(Value::Float(*lhs as f64 - rhs)),
            Value::String(_) => Err("Cannot subtract Integer with non-numeric type".to_string()),
        }
    }

    fn mul(&self, lhs: &Value, rhs: &Value) -> Result<Value, String> {
        let Value::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            Value::Integer(rhs) => Ok(Value::Integer(lhs * rhs)),
            #[allow(clippy::cast_precision_loss)]
            Value::Float(rhs) => Ok(Value::Float(*lhs as f64 * rhs)),
            Value::String(_) => Err("Cannot multiply Integer with non-numeric type".to_string()),
        }
    }

    fn div(&self, lhs: &Value, rhs: &Value) -> Result<Value, String> {
        let Value::Integer(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            Value::Integer(rhs) => {
                if *rhs == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Integer(lhs / rhs))
                }
            }
            Value::Float(rhs) => {
                if *rhs == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    #[allow(clippy::cast_precision_loss)]
                    Ok(Value::Float(*lhs as f64 / rhs))
                }
            }
            Value::String(_) => Err("Cannot divide Integer with non-numeric type".to_string()),
        }
    }
}

struct FloatOperations;
impl Operations for FloatOperations {
    fn add(&self, lhs: &Value, rhs: &Value) -> Result<Value, String> {
        let Value::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            #[allow(clippy::cast_precision_loss)]
            Value::Integer(rhs) => Ok(Value::Float(lhs + *rhs as f64)),
            Value::Float(rhs) => Ok(Value::Float(lhs + rhs)),
            Value::String(_) => Err("Cannot add Float with non-numeric type".to_string()),
        }
    }

    fn sub(&self, lhs: &Value, rhs: &Value) -> Result<Value, String> {
        let Value::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            #[allow(clippy::cast_precision_loss)]
            Value::Integer(rhs) => Ok(Value::Float(lhs - *rhs as f64)),
            Value::Float(rhs) => Ok(Value::Float(lhs - rhs)),
            Value::String(_) => Err("Cannot subtract Float with non-numeric type".to_string()),
        }
    }

    fn mul(&self, lhs: &Value, rhs: &Value) -> Result<Value, String> {
        let Value::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            #[allow(clippy::cast_precision_loss)]
            Value::Integer(rhs) => Ok(Value::Float(lhs * *rhs as f64)),
            Value::Float(rhs) => Ok(Value::Float(lhs * rhs)),
            Value::String(_) => Err("Cannot multiply Float with non-numeric type".to_string()),
        }
    }

    fn div(&self, lhs: &Value, rhs: &Value) -> Result<Value, String> {
        let Value::Float(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            Value::Integer(rhs) => {
                if *rhs == 0 {
                    Err("Division by zero".to_string())
                } else {
                    #[allow(clippy::cast_precision_loss)]
                    Ok(Value::Float(lhs / *rhs as f64))
                }
            }
            Value::Float(rhs) => {
                if *rhs == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(lhs / rhs))
                }
            }
            Value::String(_) => Err("Cannot divide Float with non-numeric type".to_string()),
        }
    }
}

struct StringOperations;
impl Operations for StringOperations {
    fn add(&self, lhs: &Value, rhs: &Value) -> Result<Value, String> {
        let Value::String(lhs) = lhs else {
            unreachable!()
        };
        match rhs {
            Value::String(rhs) => Ok(Value::String(lhs.clone() + rhs)),
            _ => Err("Cannot add String with non-String type".to_string()),
        }
    }

    fn sub(&self, _lhs: &Value, _rhs: &Value) -> Result<Value, String> {
        Err("Subtraction not supported for String type".to_string())
    }

    fn mul(&self, _lhs: &Value, _rhs: &Value) -> Result<Value, String> {
        Err("Multiplication not supported for String type".to_string())
    }

    fn div(&self, _lhs: &Value, _rhs: &Value) -> Result<Value, String> {
        Err("Division not supported for String type".to_string())
    }
}

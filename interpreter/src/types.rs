//! Contains types used throughout the interpreter.

use std::ops::{Add, Div, Mul, Sub};

/// Represents the environment mapping variable names to their values.
pub type Environment = std::collections::HashMap<String, Option<Value>>;

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
    fn apply<I, F, S>(
        &self,
        other: &Self,
        int_op: I,
        float_op: F,
        string_op: S,
    ) -> Result<Self, String>
    where
        I: Fn(i64, i64) -> Result<Self, String>,
        F: Fn(f64, f64) -> Result<Self, String>,
        S: Fn(&str, &str) -> Result<Self, String>,
    {
        match (self, other) {
            (Self::Integer(lhs), Self::Integer(rhs)) => int_op(*lhs, *rhs),
            #[allow(clippy::cast_precision_loss)]
            (Self::Integer(lhs), Self::Float(rhs)) => float_op(*lhs as f64, *rhs),
            #[allow(clippy::cast_precision_loss)]
            (Self::Float(lhs), Self::Integer(rhs)) => float_op(*lhs, *rhs as f64),
            (Self::Float(lhs), Self::Float(rhs)) => float_op(*lhs, *rhs),
            (Self::String(lhs), Self::String(rhs)) => string_op(lhs, rhs),
            (Self::String(_), _) | (_, Self::String(_)) => Err(String::from(
                "Cannot perform arithmetic operations on strings with other types",
            )),
        }
    }
}

macro_rules! impl_op {
    ($trait:ident, $method:ident, $int_op:expr, $float_op:expr, $string_op:expr) => {
        impl $trait<Value> for Value {
            type Output = Result<Self, String>;
            fn $method(self, other: Self) -> Result<Self, String> {
                self.apply(&other, $int_op, $float_op, $string_op)
            }
        }

        impl $trait<&Value> for Value {
            type Output = Result<Self, String>;
            fn $method(self, other: &Self) -> Result<Self, String> {
                self.apply(other, $int_op, $float_op, $string_op)
            }
        }

        impl $trait<Value> for &Value {
            type Output = Result<Value, String>;
            fn $method(self, other: Value) -> Result<Value, String> {
                self.apply(&other, $int_op, $float_op, $string_op)
            }
        }

        impl $trait<&Value> for &Value {
            type Output = Result<Value, String>;
            fn $method(self, other: &Value) -> Result<Value, String> {
                self.apply(other, $int_op, $float_op, $string_op)
            }
        }
    };
}

impl_op!(
    Add,
    add,
    |lhs, rhs| Ok(Value::Integer(lhs + rhs)),
    |lhs, rhs| Ok(Value::Float(lhs + rhs)),
    |lhs, rhs| Ok(Value::String(lhs.to_owned() + rhs))
);

impl_op!(
    Sub,
    sub,
    |lhs, rhs| Ok(Value::Integer(lhs - rhs)),
    |lhs, rhs| Ok(Value::Float(lhs - rhs)),
    |_, _| Err(String::from("Cannot perform subtraction on strings"))
);

impl_op!(
    Mul,
    mul,
    |lhs, rhs| Ok(Value::Integer(lhs * rhs)),
    |lhs, rhs| Ok(Value::Float(lhs * rhs)),
    |_, _| Err(String::from("Cannot perform multiplication on strings"))
);

impl_op!(
    Div,
    div,
    |lhs, rhs| {
        if rhs == 0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(Value::Integer(lhs / rhs))
        }
    },
    |lhs, rhs| {
        if rhs == 0.0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(Value::Float(lhs / rhs))
        }
    },
    |_, _| Err(String::from("Cannot perform division on strings"))
);

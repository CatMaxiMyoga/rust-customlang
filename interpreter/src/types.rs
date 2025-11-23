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
}

impl Value {
    fn apply<I, F>(&self, other: &Self, int_op: I, float_op: F) -> Self
    where
        I: Fn(i64, i64) -> i64,
        F: Fn(f64, f64) -> f64,
    {
        match (self, other) {
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(int_op(*lhs, *rhs)),
            #[allow(clippy::cast_precision_loss)]
            (Self::Integer(lhs), Self::Float(rhs)) => Self::Float(float_op(*lhs as f64, *rhs)),
            #[allow(clippy::cast_precision_loss)]
            (Self::Float(lhs), Self::Integer(rhs)) => Self::Float(float_op(*lhs, *rhs as f64)),
            (Self::Float(lhs), Self::Float(rhs)) => Self::Float(float_op(*lhs, *rhs)),
        }
    }
}

macro_rules! impl_op {
    ($trait:ident, $method:ident, $int_op:expr, $float_op:expr) => {
        impl $trait<Value> for Value {
            type Output = Self;
            fn $method(self, other: Self) -> Self {
                self.apply(&other, $int_op, $float_op)
            }
        }

        impl $trait<&Value> for Value {
            type Output = Self;
            fn $method(self, other: &Self) -> Self {
                self.apply(other, $int_op, $float_op)
            }
        }
    };
}

impl_op!(Add, add, i64::add, f64::add);
impl_op!(Sub, sub, i64::sub, f64::sub);
impl_op!(Mul, mul, i64::mul, f64::mul);
impl_op!(Div, div, i64::div, f64::div);

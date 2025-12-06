//! Types for compiler

#![allow(dead_code)] // TEMP

use std::collections::HashMap;

/// Return type for compiler methods
pub type CompilerResult = Result<(), String>;
/// Functions `HashMap`
pub type Functions = HashMap<String, (Vec<(Type, String)>, String)>;

pub fn prefix(identifier: &str) -> String {
    String::from("rustmm_user_") + identifier
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Bool,
    Int,
    Float,
    String,
    Void,
}

impl Type {
    pub fn from_str(type_str: &str) -> Result<Self, String> {
        match type_str {
            "Bool" => Ok(Self::Bool),
            "Int" => Ok(Self::Int),
            "Float" => Ok(Self::Float),
            "String" => Ok(Self::String),
            "Void" => Ok(Self::Void),
            _ => Err(format!("Unknown type: {type_str}")),
        }
    }

    #[must_use]
    pub const fn to_c_type(&self) -> &'static str {
        match self {
            Self::Bool => "bool",
            Self::Int => "int",
            Self::Float => "double",
            Self::String => "rustmm_type_string",
            Self::Void => "void",
        }
    }

    /// `+`
    pub fn add(&self, other: &Self) -> Result<Self, String> {
        use Type::{Float, Int, String};
        match (self, other) {
            (Int, Int) => Ok(Int),
            (Float | Int, Float) | (Float, Int) => Ok(Float),
            (String, String) => Ok(String),
            _ => Err(format!("Cannot add types {self:?} and {other:?}")),
        }
    }

    /// `-`
    pub fn sub(&self, other: &Self) -> Result<Self, String> {
        use Type::{Float, Int};
        match (self, other) {
            (Int, Int) => Ok(Int),
            (Float | Int, Float) | (Float, Int) => Ok(Float),
            _ => Err(format!("Cannot subtract types {self:?} and {other:?}")),
        }
    }

    /// `*`
    pub fn mul(&self, other: &Self) -> Result<Self, String> {
        use Type::{Float, Int};
        match (self, other) {
            (Int, Int) => Ok(Int),
            (Float | Int, Float) | (Float, Int) => Ok(Float),
            _ => Err(format!("Cannot multiply types {self:?} and {other:?}")),
        }
    }

    /// `/`
    pub fn div(&self, other: &Self) -> Result<Self, String> {
        use Type::{Float, Int};
        match (self, other) {
            (Int, Int) => Ok(Int),
            (Float | Int, Float) | (Float, Int) => Ok(Float),
            _ => Err(format!("Cannot divide types {self:?} and {other:?}")),
        }
    }

    /// `==`
    pub fn eq(&self, other: &Self) -> Result<Self, String> {
        use Type::{Bool, Float, Int, String};
        match (self, other) {
            (Int | Float, Int | Float) | (Bool, Bool) | (String, String) => Ok(Bool),
            _ => Err(format!(
                "Cannot compare types {self:?} and {other:?} using '==' or '!='"
            )),
        }
    }

    /// `!=`
    pub fn ne(&self, other: &Self) -> Result<Self, String> {
        self.eq(other)
    }

    /// `<`
    pub fn lt(&self, other: &Self) -> Result<Self, String> {
        use Type::{Bool, Float, Int};
        match (self, other) {
            (Int | Float, Int | Float) => Ok(Bool),
            _ => Err(format!(
                "Cannot compare types {self:?} and {other:?} using '<', '>', '<=' or '>='"
            )),
        }
    }

    /// `>`
    pub fn gt(&self, other: &Self) -> Result<Self, String> {
        self.lt(other)
    }

    /// `<=`
    pub fn le(&self, other: &Self) -> Result<Self, String> {
        self.lt(other)
    }

    /// `>=`
    pub fn ge(&self, other: &Self) -> Result<Self, String> {
        self.lt(other)
    }

    /// `&&`
    pub fn and(&self, other: &Self) -> Result<Self, String> {
        use Type::Bool;
        match (self, other) {
            (Bool, Bool) => Ok(Bool),
            _ => Err(format!(
                "Cannot perform logical AND on types {self:?} and {other:?}"
            )),
        }
    }

    /// `||`
    pub fn or(&self, other: &Self) -> Result<Self, String> {
        self.and(other)
    }

    /// `!`
    pub fn not(&self) -> Result<Self, String> {
        use Type::Bool;
        match self {
            Bool => Ok(Bool),
            _ => Err(format!("Cannot perform logical NOT on type {self:?}")),
        }
    }
}

pub enum BuiltinFunction {
    Print,
    Println,
    StringToBool,
    StringToInt,
    StringToFloat,
    BoolToString,
    BoolToInt,
    BoolToFloat,
    IntToString,
    IntToBool,
    IntToFloat,
    FloatToString,
    FloatToBool,
    FloatToInt,
}

impl BuiltinFunction {
    #[must_use]
    pub const fn to_c_function(&self) -> &'static str {
        match self {
            Self::Print => "rustmm_builtin_print",
            Self::Println => "rustmm_builtin_println",
            Self::StringToBool => "rustmm_builtin_stringToBool",
            Self::StringToInt => "rustmm_builtin_stringToInt",
            Self::StringToFloat => "rustmm_builtin_stringToFloat",
            Self::BoolToString => "rustmm_builtin_boolToString",
            Self::BoolToInt => "rustmm_builtin_boolToInt",
            Self::BoolToFloat => "rustmm_builtin_boolToFloat",
            Self::IntToString => "rustmm_builtin_intToString",
            Self::IntToBool => "rustmm_builtin_intToBool",
            Self::IntToFloat => "rustmm_builtin_intToFloat",
            Self::FloatToString => "rustmm_builtin_floatToString",
            Self::FloatToBool => "rustmm_builtin_floatToBool",
            Self::FloatToInt => "rustmm_builtin_floatToInt",
        }
    }

    #[must_use]
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "print" => Some(Self::Print),
            "println" => Some(Self::Println),
            "boolToString" => Some(Self::BoolToString),
            "intToString" => Some(Self::IntToString),
            "floatToString" => Some(Self::FloatToString),
            _ => None,
        }
    }
}

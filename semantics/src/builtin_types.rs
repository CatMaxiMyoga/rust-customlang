//! Contains the builtin types' class declarations.

use std::collections::HashMap;

use crate::types::{Class, Function, Type};

/// Returns the builtin types' class declarations to be added to the global scope to allow for
/// proper semantic analysis.
#[must_use]
pub fn get_builtin_types() -> Vec<Class> {
    let classes: Vec<Class> = vec![
        builtin_string(),
        builtin_bool(),
        builtin_int(),
        builtin_float(),
    ];

    classes
}

macro_rules! methods {
    ($({ $name:ident, $return_type:expr, $is_static:literal $(,[$($parameter_type:expr),+])? $(,)? }),+ $(,)?) => {
        {
        let mut map: HashMap<String, Vec<Function>> = HashMap::new();

        $(map
            .entry(stringify!($name).into())
            .or_default()
            .push(Function {
                parameters: methods!(@params $([$($parameter_type),+])?),
                return_type: $return_type,
                is_static: $is_static
            });
        )+

        map
        }
    };
    (@params [$($parameter_type:expr),+]) => { vec![$($parameter_type),+] };
    (@params) => { vec![] };
}

fn builtin_string() -> Class {
    Class {
        name: "string".into(),
        methods: methods![
            {toBool, Type::Boolean, false},
            {toInt, Type::Int, false},
            {toFloat, Type::Float, false},

            {_bopAdd, Type::String, false, [Type::String]},
            {_bopEq, Type::Boolean, false, [Type::String]},
            {_bopNe, Type::Boolean, false, [Type::String]}
        ],
        fields: HashMap::new(),
    }
}

fn builtin_bool() -> Class {
    Class {
        name: "bool".into(),
        methods: methods![
            {toString, Type::String, false},
            {toInt, Type::Int, false},
            {toFloat, Type::Float, false},

            {_bopEq, Type::Boolean, false, [Type::Boolean]},
            {_bopNe, Type::Boolean, false, [Type::Boolean]},
            {_bopAnd, Type::Boolean, false, [Type::Boolean]},
            {_bopOr, Type::Boolean, false, [Type::Boolean]},

            {_uopNot, Type::Boolean, false}
        ],
        fields: HashMap::new(),
    }
}

fn builtin_int() -> Class {
    Class {
        name: "int".into(),
        methods: methods![
            {toString, Type::String, false},
            {toBool, Type::Boolean, false},
            {toFloat, Type::Float, false},

            {_bopAdd, Type::Int, false, [Type::Int]},
            {_bopSub, Type::Int, false, [Type::Int]},
            {_bopMul, Type::Int, false, [Type::Int]},
            {_bopDiv, Type::Int, false, [Type::Int]},
            {_bopEq, Type::Boolean, false, [Type::Int]},
            {_bopNe, Type::Boolean, false, [Type::Int]},
            {_bopLt, Type::Boolean, false, [Type::Int]},
            {_bopGt, Type::Boolean, false, [Type::Int]},
            {_bopLe, Type::Boolean, false, [Type::Int]},
            {_bopGe, Type::Boolean, false, [Type::Int]},

            {_bopAdd, Type::Float, false, [Type::Float]},
            {_bopSub, Type::Float, false, [Type::Float]},
            {_bopMul, Type::Float, false, [Type::Float]},
            {_bopDiv, Type::Float, false, [Type::Float]},
            {_bopEq, Type::Boolean, false, [Type::Float]},
            {_bopNe, Type::Boolean, false, [Type::Float]},
            {_bopLt, Type::Boolean, false, [Type::Float]},
            {_bopGt, Type::Boolean, false, [Type::Float]},
            {_bopLe, Type::Boolean, false, [Type::Float]},
            {_bopGe, Type::Boolean, false, [Type::Float]},
        ],
        fields: HashMap::new(),
    }
}

fn builtin_float() -> Class {
    Class {
        name: "float".into(),
        methods: methods![
            {toString, Type::String, false},
            {toBool, Type::Boolean, false},
            {toInt, Type::Int, false},

            {_bopAdd, Type::Float, false, [Type::Float]},
            {_bopSub, Type::Float, false, [Type::Float]},
            {_bopMul, Type::Float, false, [Type::Float]},
            {_bopDiv, Type::Float, false, [Type::Float]},
            {_bopEq, Type::Boolean, false, [Type::Float]},
            {_bopNe, Type::Boolean, false, [Type::Float]},
            {_bopLt, Type::Boolean, false, [Type::Float]},
            {_bopGt, Type::Boolean, false, [Type::Float]},
            {_bopLe, Type::Boolean, false, [Type::Float]},
            {_bopGe, Type::Boolean, false, [Type::Float]},

            {_bopAdd, Type::Float, false, [Type::Int]},
            {_bopSub, Type::Float, false, [Type::Int]},
            {_bopMul, Type::Float, false, [Type::Int]},
            {_bopDiv, Type::Float, false, [Type::Int]},
            {_bopEq, Type::Boolean, false, [Type::Int]},
            {_bopNe, Type::Boolean, false, [Type::Int]},
            {_bopLt, Type::Boolean, false, [Type::Int]},
            {_bopGt, Type::Boolean, false, [Type::Int]},
            {_bopLe, Type::Boolean, false, [Type::Int]},
            {_bopGe, Type::Boolean, false, [Type::Int]},
        ],
        fields: HashMap::new(),
    }
}

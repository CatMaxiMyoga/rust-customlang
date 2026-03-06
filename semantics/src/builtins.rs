//! Contains the builtin types' class declarations.

use std::collections::HashMap;

use crate::types::{Class, Function, Type};

macro_rules! functions {
    ($({
        $name:ident($($parameter_type:expr),*) => $return_type:expr,
        $is_static:literal
        $(,)?
    }),+ $(,)?) => {
        {
        let mut map: HashMap<String, Vec<Function>> = HashMap::new();

        $(map
            .entry(stringify!($name).into())
            .or_default()
            .push(Function {
                parameters: vec![$($parameter_type),*],
                return_type: $return_type,
                is_static: $is_static,
            });
        )+

        map
        }
    };
    (@params [$($parameter_type:expr),+]) => { vec![$($parameter_type),+] };
    (@params) => { vec![] };
}

/// Returns a hashmap of the builtin functions for the global scope <Name, Functions>.
#[must_use]
pub fn get_builtin_functions() -> HashMap<String, Vec<Function>> {
    functions![
        { print(Type::String) => Type::Void, false },
        { println(Type::String) => Type::Void, false },
        { boolToString(Type::Boolean) => Type::String, true },
        { intToString(Type::Int) => Type::String, true },
        { floatToString(Type::Float) => Type::String, true },
        { stringToBool(Type::String) => Type::Boolean, true },
        { intToBool(Type::Int) => Type::Boolean, true },
        { floatToBool(Type::Float) => Type::Boolean, true },
        { stringToInt(Type::String) => Type::Int, true },
        { boolToInt(Type::Boolean) => Type::Int, true },
        { floatToInt(Type::Float) => Type::Int, true },
        { stringToFloat(Type::String) => Type::Float, true },
        { boolToFloat(Type::Boolean) => Type::Float, true },
        { intToFloat(Type::Int) => Type::Float, true },
    ]
}

/// Returns a vector of the builtin types' class declarations for the global scope.
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

fn builtin_string() -> Class {
    Class {
        name: "string".into(),
        methods: functions![
            {toBool() => Type::Boolean, false},
            {toInt() => Type::Int, false},
            {toFloat() => Type::Float, false},

            {_bopAdd(Type::String) => Type::String, false},
            {_bopEq(Type::String) => Type::Boolean, false},
            {_bopNe(Type::String) => Type::Boolean, false}
        ],
        fields: HashMap::new(),
    }
}

fn builtin_bool() -> Class {
    Class {
        name: "bool".into(),
        methods: functions![
            {toString() => Type::String, false},
            {toInt() => Type::Int, false},
            {toFloat() => Type::Float, false},

            {_bopEq(Type::Boolean) => Type::Boolean, false},
            {_bopNe(Type::Boolean) => Type::Boolean, false},
            {_bopAnd(Type::Boolean) => Type::Boolean, false},
            {_bopOr(Type::Boolean) => Type::Boolean, false},

            {_uopNot() => Type::Boolean, false}
        ],
        fields: HashMap::new(),
    }
}

fn builtin_int() -> Class {
    Class {
        name: "int".into(),
        methods: functions![
            {toString() => Type::String, false},
            {toBool() => Type::Boolean, false},
            {toFloat() => Type::Float, false},

            {_bopAdd(Type::Int) => Type::Int, false},
            {_bopSub(Type::Int) => Type::Int, false},
            {_bopMul(Type::Int) => Type::Int, false},
            {_bopDiv(Type::Int) => Type::Int, false},
            {_bopEq(Type::Int) => Type::Boolean, false},
            {_bopNe(Type::Int) => Type::Boolean, false},
            {_bopLt(Type::Int) => Type::Boolean, false},
            {_bopGt(Type::Int) => Type::Boolean, false},
            {_bopLe(Type::Int) => Type::Boolean, false},
            {_bopGe(Type::Int) => Type::Boolean, false},

            {_bopAdd(Type::Float) => Type::Float, false},
            {_bopSub(Type::Float) => Type::Float, false},
            {_bopMul(Type::Float) => Type::Float, false},
            {_bopDiv(Type::Float) => Type::Float, false},
            {_bopEq(Type::Float) => Type::Boolean, false},
            {_bopNe(Type::Float) => Type::Boolean, false},
            {_bopLt(Type::Float) => Type::Boolean, false},
            {_bopGt(Type::Float) => Type::Boolean, false},
            {_bopLe(Type::Float) => Type::Boolean, false},
            {_bopGe(Type::Float) => Type::Boolean, false},
        ],
        fields: HashMap::new(),
    }
}

fn builtin_float() -> Class {
    Class {
        name: "float".into(),
        methods: functions![
            {toString() => Type::String, false},
            {toBool() => Type::Boolean, false},
            {toInt() => Type::Int, false},

            {_bopAdd(Type::Float) => Type::Float, false},
            {_bopSub(Type::Float) => Type::Float, false},
            {_bopMul(Type::Float) => Type::Float, false},
            {_bopDiv(Type::Float) => Type::Float, false},
            {_bopEq(Type::Float) => Type::Boolean, false},
            {_bopNe(Type::Float) => Type::Boolean, false},
            {_bopLt(Type::Float) => Type::Boolean, false},
            {_bopGt(Type::Float) => Type::Boolean, false},
            {_bopLe(Type::Float) => Type::Boolean, false},
            {_bopGe(Type::Float) => Type::Boolean, false},

            {_bopAdd(Type::Int) => Type::Float, false},
            {_bopSub(Type::Int) => Type::Float, false},
            {_bopMul(Type::Int) => Type::Float, false},
            {_bopDiv(Type::Int) => Type::Float, false},
            {_bopEq(Type::Int) => Type::Boolean, false},
            {_bopNe(Type::Int) => Type::Boolean, false},
            {_bopLt(Type::Int) => Type::Boolean, false},
            {_bopGt(Type::Int) => Type::Boolean, false},
            {_bopLe(Type::Int) => Type::Boolean, false},
            {_bopGe(Type::Int) => Type::Boolean, false},
        ],
        fields: HashMap::new(),
    }
}

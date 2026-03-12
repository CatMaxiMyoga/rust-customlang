//! Contains the builtin types' class declarations.

use std::collections::HashMap;

use crate::types::{Class, Function, Type};

/// # Example
/// ```
/// functions![
///     /* non-static function */
///     somefunc(String, Int) => String,
///     /* static function, how it's supposed to be used for readability */
///     somefunc(Int, String) => String #static,
///     /* In reality, anything except another # can follow the #, so these are valid too */
///     somefunc(Int, Int) => String #123,
///     somefunc(String, String) => String #,,
///     somefunc(Boolean) => String #/,
/// ]
/// ```
macro_rules! functions {
    ($(
        $name:ident($($parameter_type:ident),*) => $return_type:ident $(# $is_static:tt)? $(,)?
    ),+) => {
        {
        let mut map: HashMap<String, Vec<Function>> = HashMap::new();

        $(map
            .entry(stringify!($name).into())
            .or_default()
            .push(Function {
                parameters: vec![$(Type::$parameter_type),*],
                return_type: Type::$return_type,
                is_static: functions!(@s $($is_static)?),
            });
        )+

        map
        }
    };
    (@s $is_static:tt) => { true };
    (@s) => { false };
}

/// Returns a vector of the builtin types' class declarations for the global scope.
#[must_use]
pub fn get_builtin_types() -> Vec<Class> {
    let classes: Vec<Class> = vec![
        builtin_builtin(),
        builtin_string(),
        builtin_bool(),
        builtin_int(),
        builtin_float(),
    ];

    classes
}

fn builtin_builtin() -> Class {
    Class {
        name: "Builtin".into(),
        methods: functions![
            print(String) => Void #static,
            print(Boolean) => Void #static,
            print(Int) => Void #static,
            print(Float) => Void #static,

            println(String) => Void #static,
            println(Boolean) => Void #static,
            println(Int) => Void #static,
            println(Float) => Void #static,

            parseString(Boolean) => String #static,
            parseString(Int) => String #static,
            parseString(Float) => String #static,

            parseBool(String) => Boolean #static,
            parseBool(Int) => Boolean #static,
            parseBool(Float) => Boolean #static,

            parseInt(String) => Int #static,
            parseInt(Boolean) => Int #static,
            parseInt(Float) => Int #static,

            parseFloat(String) => Float #static,
            parseFloat(Boolean) => Float #static,
            parseFloat(Int) => Float #static,
        ],
        fields: HashMap::new(),
    }
}

fn builtin_string() -> Class {
    Class {
        name: "string".into(),
        methods: functions![
            toBool() => Boolean,
            toInt() => Int,
            toFloat() => Float,

            _bopAdd(String) => String,
            _bopEq(String) => Boolean,
            _bopNe(String) => Boolean,
        ],
        fields: HashMap::new(),
    }
}

fn builtin_bool() -> Class {
    Class {
        name: "bool".into(),
        methods: functions![
            toString() => String,
            toInt() => Int,
            toFloat() => Float,

            _bopEq(Boolean) => Boolean,
            _bopNe(Boolean) => Boolean,
            _bopAnd(Boolean) => Boolean,
            _bopOr(Boolean) => Boolean,

            _uopNot() => Boolean,
        ],
        fields: HashMap::new(),
    }
}

fn builtin_int() -> Class {
    Class {
        name: "int".into(),
        methods: functions![
            toString() => String,
            toBool() => Boolean,
            toFloat() => Float,

            _bopAdd(Int) => Int,
            _bopSub(Int) => Int,
            _bopMul(Int) => Int,
            _bopDiv(Int) => Int,
            _bopEq(Int) => Boolean,
            _bopNe(Int) => Boolean,
            _bopLt(Int) => Boolean,
            _bopGt(Int) => Boolean,
            _bopLe(Int) => Boolean,
            _bopGe(Int) => Boolean,

            _bopAdd(Float) => Float,
            _bopSub(Float) => Float,
            _bopMul(Float) => Float,
            _bopDiv(Float) => Float,
            _bopEq(Float) => Boolean,
            _bopNe(Float) => Boolean,
            _bopLt(Float) => Boolean,
            _bopGt(Float) => Boolean,
            _bopLe(Float) => Boolean,
            _bopGe(Float) => Boolean,
        ],
        fields: HashMap::new(),
    }
}

fn builtin_float() -> Class {
    Class {
        name: "float".into(),
        methods: functions![
            toString() => String,
            toBool() => Boolean,
            toInt() => Int,

            _bopAdd(Float) => Float,
            _bopSub(Float) => Float,
            _bopMul(Float) => Float,
            _bopDiv(Float) => Float,
            _bopEq(Float) => Boolean,
            _bopNe(Float) => Boolean,
            _bopLt(Float) => Boolean,
            _bopGt(Float) => Boolean,
            _bopLe(Float) => Boolean,
            _bopGe(Float) => Boolean,

            _bopAdd(Int) => Float,
            _bopSub(Int) => Float,
            _bopMul(Int) => Float,
            _bopDiv(Int) => Float,
            _bopEq(Int) => Boolean,
            _bopNe(Int) => Boolean,
            _bopLt(Int) => Boolean,
            _bopGt(Int) => Boolean,
            _bopLe(Int) => Boolean,
            _bopGe(Int) => Boolean,
        ],
        fields: HashMap::new(),
    }
}

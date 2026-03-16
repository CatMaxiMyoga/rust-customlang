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
        $return_type:ident $name:ident($($parameter_type:ident),*) $(# $is_static:tt)? $(,)?
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
            Void print(String) #static,
            Void print(Boolean) #static,
            Void print(Int) #static,
            Void print(Float) #static,

            Void println() #static,
            Void println(String) #static,
            Void println(Boolean) #static,
            Void println(Int) #static,
            Void println(Float) #static,

            String parseString(Boolean) #static,
            String parseString(Int) #static,
            String parseString(Float) #static,

            Boolean parseBool(String) #static,
            Boolean parseBool(Int) #static,
            Boolean parseBool(Float) #static,

            Int parseInt(String) #static,
            Int parseInt(Boolean) #static,
            Int parseInt(Float) #static,

            Float parseFloat(String) #static,
            Float parseFloat(Boolean) #static,
            Float parseFloat(Int) #static,
        ],
        fields: HashMap::new(),
    }
}

fn builtin_string() -> Class {
    Class {
        name: "string".into(),
        methods: functions![
            Boolean toBool(),
            Int toInt(),
            Float toFloat(),

            String _bopAdd(String),
            String _bopMul(Int),
            String _bopDiv(String),
            Boolean _bopEq(String),
            Boolean _bopNe(String),
        ],
        fields: HashMap::new(),
    }
}

fn builtin_bool() -> Class {
    Class {
        name: "bool".into(),
        methods: functions![
            String toString(),
            Int toInt(),
            Float toFloat(),

            Boolean _bopEq(Boolean),
            Boolean _bopNe(Boolean),
            Boolean _bopAnd(Boolean),
            Boolean _bopOr(Boolean),

            Boolean _uopNot(),
        ],
        fields: HashMap::new(),
    }
}

fn builtin_int() -> Class {
    Class {
        name: "int".into(),
        methods: functions![
            String toString(),
            Boolean toBool(),
            Float toFloat(),

            Int _bopAdd(Int),
            Int _bopSub(Int),
            Int _bopMul(Int),
            Int _bopDiv(Int),
            Boolean _bopEq(Int),
            Boolean _bopNe(Int),
            Boolean _bopLt(Int),
            Boolean _bopGt(Int),
            Boolean _bopLe(Int),
            Boolean _bopGe(Int),

            Float _bopAdd(Float),
            Float _bopSub(Float),
            Float _bopMul(Float),
            Float _bopDiv(Float),
            Boolean _bopEq(Float),
            Boolean _bopNe(Float),
            Boolean _bopLt(Float),
            Boolean _bopGt(Float),
            Boolean _bopLe(Float),
            Boolean _bopGe(Float),
        ],
        fields: HashMap::new(),
    }
}

fn builtin_float() -> Class {
    Class {
        name: "float".into(),
        methods: functions![
            String toString(),
            Boolean toBool(),
            Int toInt(),

            Float _bopAdd(Float),
            Float _bopSub(Float),
            Float _bopMul(Float),
            Float _bopDiv(Float),
            Boolean _bopEq(Float),
            Boolean _bopNe(Float),
            Boolean _bopLt(Float),
            Boolean _bopGt(Float),
            Boolean _bopLe(Float),
            Boolean _bopGe(Float),

            Float _bopAdd(Int),
            Float _bopSub(Int),
            Float _bopMul(Int),
            Float _bopDiv(Int),
            Boolean _bopEq(Int),
            Boolean _bopNe(Int),
            Boolean _bopLt(Int),
            Boolean _bopGt(Int),
            Boolean _bopLe(Int),
            Boolean _bopGe(Int),
        ],
        fields: HashMap::new(),
    }
}

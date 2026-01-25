//! Utilities for transpiling to C#.

pub fn prefix(string: &str) -> String {
    String::from("rmm_") + string
}

pub struct Type;

impl Type {
    pub fn from(t: &str) -> String {
        match t {
            "string" => String::from("CustomLang.Types.rmm_String"),
            "int" => String::from("CustomLang.Types.rmm_Int"),
            "float" => String::from("CustomLang.Types.rmm_Float"),
            "bool" => String::from("CustomLang.Types.rmm_Bool"),
            "void" => String::from("void"),
            x => String::from("CustomLang.Users.") + x,
        }
    }
}

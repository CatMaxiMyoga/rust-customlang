//! Main library file for the compiler module.

use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    process::{Command, ExitStatus},
};

use parser::types::{
    BinaryOperator, Expr, Expression, Literal, Program, Statement, Stmt, UnaryOperator,
};

use crate::types::{BuiltinFunction, CompilerResult, Functions, Type, prefix};
mod types;

const MAIN_HEADER: &str = r#"
#include "rustmm_builtins.h"
#include "rustmm_internals.h"
#include "rustmm_operators.h"
#include "rustmm_user_functions.h"
#include <stdbool.h>

int main(void) {
"#;

const FUNC_HEADER: &str = r#"
#include "rustmm_builtins.h"
#include "rustmm_operators.h"
#include "rustmm_internals.h"
#include <stdbool.h>

"#;

/// Compiler struct responsible for compiling source code from the AST
#[derive(Debug, Clone)]
pub struct Compiler {
    output: String,
    functions: Functions,
    environment: HashMap<String, Type>,
    indent_level: usize,
    inside_function: bool,
}

impl Default for Compiler {
    fn default() -> Self {
        let mut funcs: HashMap<String, (Vec<(Type, String)>, String)> = HashMap::new();
        let mut env_: HashMap<String, Type> = HashMap::new();

        Self::add_builtin_func(
            &mut funcs,
            &mut env_,
            "print",
            vec![(Type::String, "s")],
            Type::Void,
        );

        Self::add_builtin_func(
            &mut funcs,
            &mut env_,
            "println",
            vec![(Type::String, "s")],
            Type::Void,
        );

        Self::add_builtin_func(
            &mut funcs,
            &mut env_,
            "intToString",
            vec![(Type::Int, "i")],
            Type::String,
        );

        Self::add_builtin_func(
            &mut funcs,
            &mut env_,
            "floatToString",
            vec![(Type::Float, "f")],
            Type::String,
        );

        Self::add_builtin_func(
            &mut funcs,
            &mut env_,
            "boolToString",
            vec![(Type::Bool, "b")],
            Type::String,
        );

        Self {
            output: String::new(),
            functions: funcs,
            environment: env_,
            indent_level: 1,
            inside_function: false,
        }
    }
}

impl Compiler {
    /// Compiles the given program AST into target code.
    ///
    /// # Errors
    /// If compilation fails, returns a `String` describing the error.
    pub fn compile(
        program: Program,
        out_file: &str,
        transpile_only: bool,
        gcc_args: &str,
        clean_up: bool,
    ) -> CompilerResult {
        let mut compiler: Self = Self {
            output: String::from(MAIN_HEADER),
            ..Default::default()
        };

        for statement in program.statements {
            compiler.statement(statement)?;
        }

        compiler.output.push_str("  return 0;\n}\n\n");

        fs::create_dir_all("out").map_err(|_| String::from("Unable to create out/ directory"))?;

        if clean_up {
            Command::new("sh")
                .arg("-c")
                .arg("rm out/* 2>/dev/null")
                .status()
                .ok();
        }

        let mut main_file: File = File::create("out/rustmm_user_code.c")
            .map_err(|_| String::from("Unable to create c main file."))?;
        main_file
            .write_all(compiler.output.as_bytes())
            .map_err(|_| String::from("Unable to write to c main file."))?;

        let mut func_file: File = File::create("out/rustmm_user_functions.c")
            .map_err(|_| String::from("Unable to create c functions file."))?;
        func_file
            .write_all(FUNC_HEADER.as_bytes())
            .map_err(|_| String::from("Unable to write to c functions file."))?;

        let mut func_header_file: File = File::create("out/rustmm_user_functions.h")
            .map_err(|_| String::from("Unable to create c functions header file."))?;
        func_header_file
            .write_all(FUNC_HEADER.as_bytes())
            .map_err(|_| String::from("Unable to write to c functions header file."))?;

        for (name, (params, func_code)) in compiler.functions {
            if func_code != "builtin" {
                func_file
                    .write_all(func_code.as_bytes())
                    .map_err(|_| String::from("Unable to write to c functions file."))?;

                let stripped: &str = match name.strip_prefix("rustmm_user_") {
                    Some(s) => s,
                    None => {
                        return Err(format!(
                            "Function name '{name}' does not have expected prefix 'rustmm_user_'"
                        ))?;
                    }
                };

                func_header_file
                    .write_all(
                        format!("{} {}(", compiler.environment[stripped].to_c_type(), name)
                            .as_bytes(),
                    )
                    .map_err(|_| String::from("Unable to write to c functions header file."))?;

                for (i, (param_type, param_name)) in params.iter().enumerate() {
                    func_header_file
                        .write_all(format!("{} {}", param_type.to_c_type(), param_name).as_bytes())
                        .map_err(|_| String::from("Unable to write to c functions header file."))?;
                    if i < params.len() - 1 {
                        func_header_file.write_all(b", ").map_err(|_| {
                            String::from("Unable to write to c functions header file.")
                        })?;
                    }
                }

                func_header_file
                    .write_all(b");\n")
                    .map_err(|_| String::from("Unable to write to c functions header file."))?;
            }
        }

        let status: ExitStatus = Command::new("sh")
            .arg("-c")
            .arg("cp compiler/c_runtime/*.c compiler/c_runtime/*.h out/")
            .status()
            .map_err(|_| String::from("Failed to copy runtime files"))?;

        if !status.success() {
            return Err(String::from("Failed to copy runtime files"));
        }

        if transpile_only {
            println!("Transpiled C code is in out/");
            return Ok(());
        }

        let out_arg: String = match out_file {
            "" => String::new(),
            x => format!(r#"-o "{x}""#),
        };

        let status: ExitStatus = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "cd ~/dev/rust-customlang/out && gcc ./*.c {out_arg} {gcc_args}"
            ))
            .status()
            .map_err(|_| String::from("GCC compilation failed"))?;

        if status.success() {
            if out_arg.is_empty() {
                println!("Output is in out/");
            } else {
                println!("Output binary is at out/{out_file}");
            }
        } else {
            return Err(String::from("GCC compilation failed"));
        }

        let status: ExitStatus = Command::new("sh")
            .arg("-c")
            .arg("rm out/*.c out/*.h")
            .status()
            .map_err(|_| String::from("Failed to clean up temporary files"))?;

        if !status.success() {
            return Err(String::from("Failed to clean up temporary files"));
        }

        Ok(())
    }

    fn add_builtin_func(
        functions: &mut Functions,
        environment: &mut HashMap<String, Type>,
        name: &str,
        param_types: Vec<(Type, &str)>,
        return_type: Type,
    ) {
        let param_types: Vec<(Type, String)> = param_types
            .into_iter()
            .map(|(t, n)| (t, String::from(n)))
            .collect();

        functions.insert(
            String::from("rustmm_builtin_") + name,
            (param_types, String::from("builtin")),
        );
        environment.insert(String::from(name), return_type);
    }

    fn indent(&mut self) {
        for _ in 0..self.indent_level {
            self.output.push_str("  ");
        }
    }

    fn statement(&mut self, stmt: Stmt) -> CompilerResult {
        match stmt.node {
            Statement::Expression(expr) => {
                self.indent();
                let _ = self.expression(expr)?;
            }
            Statement::VariableDeclaration { type_, name, value } => {
                self.indent();
                self.var_decl_stmt(&type_, name, value)?;
            }
            Statement::VariableAssignment { name, value } => {
                self.indent();
                self.var_assign_stmt(&name, value)?;
            }
            Statement::FunctionDeclaration {
                return_type,
                name,
                parameters,
                body,
            } => {
                if self.inside_function {
                    Err(String::from("Nested function declarations are not allowed"))?;
                } else {
                    self.func_decl_stmt(&return_type, &name, &parameters, body)?;
                    return Ok(());
                }
            }
            Statement::Return(_) => return Err(String::from("Illegal Return")),
        }

        self.output.push_str(";\n");
        Ok(())
    }

    fn var_decl_stmt(&mut self, type_: &str, name: String, value: Option<Expr>) -> CompilerResult {
        if self.environment.contains_key(&name) {
            return Err(format!("Variable '{name}' already declared"));
        }

        let type_: Type = Type::from_str(type_)?;
        self.output.push_str(type_.to_c_type());
        self.output.push_str(&prefix(&name));
        self.output.push_str(" = ");
        if let Some(expr) = value {
            self.expression(expr)?;
        }

        self.environment.insert(name, type_);

        Ok(())
    }

    fn var_assign_stmt(&mut self, name: &str, value: Expr) -> CompilerResult {
        if !self.environment.contains_key(name) {
            return Err(format!("Variable '{name}' not declared"));
        }

        self.output.push_str(&prefix(name));
        self.output.push_str(" = ");
        self.expression(value)?;
        Ok(())
    }

    fn func_decl_stmt(
        &mut self,
        return_type: &str,
        name: &str,
        parameters: &[(String, String)],
        body: Vec<parser::types::Stmt>,
    ) -> CompilerResult {
        if self.environment.contains_key(name) {
            return Err(format!("Variable '{name}' already declared"));
        }

        let mut function_output: String = String::new();

        let return_type: Type = Type::from_str(return_type)?;
        let pname: String = prefix(name);

        if BuiltinFunction::from_str(name).is_some() {
            return Err(format!(
                "Function name '{name}' is reserved as a built-in function"
            ));
        }

        function_output.push_str(return_type.to_c_type());
        function_output.push(' ');
        function_output.push_str(&pname);
        function_output.push('(');

        self.environment.insert(name.to_owned(), return_type);

        let mut param_types: Vec<(Type, String)> = Vec::new();

        for (i, (param_type, param_name)) in parameters.iter().enumerate() {
            let param_type: Type = Type::from_str(param_type)?;
            let name: String = prefix(param_name);

            param_types.push((param_type.clone(), name.clone()));

            function_output.push_str(param_type.to_c_type());
            function_output.push(' ');
            function_output.push_str(&name);

            if i < parameters.len() - 1 {
                function_output.push_str(", ");
            }
        }

        function_output.push_str(") {\n");

        let functions: Functions = self.functions.clone();
        let mut environment: HashMap<String, Type> = HashMap::new();

        for name in functions.keys() {
            let stripped: &str = match name.strip_prefix("rustmm_user_") {
                Some(s) => s,
                None => match name.strip_prefix("rustmm_builtin_") {
                    Some(s) => s,
                    None => {
                        return Err(format!(
                            "Function name '{name}' does not have expected prefix"
                        ));
                    }
                },
            };

            if self.environment.contains_key(stripped) {
                environment.insert(stripped.to_string(), self.environment[stripped].clone());
            } else {
                return Err(format!(
                    "Function name '{stripped}' not found in environment"
                ));
            }
        }

        for (param_type, param_name) in &param_types {
            environment.insert(
                param_name.trim_start_matches("rustmm_user_").to_string(),
                param_type.clone(),
            );
        }

        let mut function_compiler: Self = Self {
            output: String::new(),
            functions,
            environment,
            indent_level: 1,
            inside_function: true,
        };

        for stmt in body {
            if matches!(stmt.node, Statement::Return(_)) {
                function_compiler.indent();
                function_compiler.output.push_str("return ");
                if let Statement::Return(expr) = stmt.node {
                    function_compiler.expression(expr)?;
                }
                function_compiler.output.push_str(";\n");
                break;
            }
            function_compiler.statement(stmt)?;
        }

        function_output.push_str(&function_compiler.output);
        function_output.push_str("}\n\n");

        self.functions.insert(pname, (param_types, function_output));

        Ok(())
    }

    fn expression(&mut self, expr: Expr) -> Result<Type, String> {
        match expr.node {
            Expression::Literal(literal) => Ok(self.literal_expr(literal)),
            Expression::Binary {
                left,
                operator,
                right,
            } => self.binary_expr(left, &operator, right),
            Expression::Unary { operator, operand } => self.unary_expr(&operator, operand),
            Expression::Identifier(identifier) => {
                self.output.push_str(&prefix(&identifier));
                self.environment
                    .get(&identifier)
                    .cloned()
                    .ok_or_else(|| format!("Undefined variable: {identifier}"))
            }
            Expression::FunctionCall { name, arguments } => self.func_call_expr(&name, arguments),
        }
    }

    fn literal_expr(&mut self, literal: Literal) -> Type {
        match literal {
            Literal::Integer(value) => {
                self.output.push_str(&value.to_string());
                Type::Int
            }
            Literal::Float(value) => {
                self.output.push_str(&value.to_string());
                Type::Float
            }
            Literal::String(value) => {
                self.output.push_str("rustmm_internal_make_string(\"");
                self.output.push_str(&value);
                self.output.push_str("\")");
                Type::String
            }
            Literal::Boolean(value) => {
                self.output.push_str(if value { "true" } else { "false" });
                Type::Bool
            }
        }
    }

    fn binary_expr(
        &mut self,
        left: Box<Expr>,
        operator: &BinaryOperator,
        right: Box<Expr>,
    ) -> Result<Type, String> {
        type OpResult = (&'static str, fn(&Type, &Type) -> Result<Type, String>);

        let mut binary_compiler: Self = self.clone();
        binary_compiler.output.clear();

        let (left_type, handled): (Type, bool) =
            self.binary_part_expr(&left, &mut binary_compiler)?;
        if !handled {
            binary_compiler.expression(*left)?;
        }
        let left: String = binary_compiler.output.clone();
        binary_compiler.output.clear();

        let (right_type, handled): (Type, bool) =
            self.binary_part_expr(&right, &mut binary_compiler)?;
        if !handled {
            binary_compiler.expression(*right)?;
        }
        let right: String = binary_compiler.output.clone();
        binary_compiler.output.clear();

        {
            use BinaryOperator::{And, Or};
            match operator {
                Or => {
                    let operation: String = format!("({left} || {right})");
                    self.output.push_str(&operation);
                    return Type::or(&left_type, &right_type);
                }
                And => {
                    let operation: String = format!("({left} && {right})");
                    self.output.push_str(&operation);
                    return Type::and(&left_type, &right_type);
                }
                _ => {}
            }
        }

        let op_result: OpResult = {
            use BinaryOperator::{
                Add, Divide, Equals, GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual,
                Multiply, NotEquals, Subtract,
            };
            match operator {
                Add => ("add", Type::add),
                Subtract => ("sub", Type::sub),
                Multiply => ("mul", Type::mul),
                Divide => ("div", Type::div),
                Equals => ("eq", Type::eq),
                NotEquals => ("ne", Type::ne),
                GreaterThan => ("gt", Type::gt),
                LessThan => ("lt", Type::lt),
                GreaterThanOrEqual => ("ge", Type::ge),
                LessThanOrEqual => ("le", Type::le),
                _ => unreachable!(),
            }
        };
        let func_op: &'static str = op_result.0;
        let result_type: Type = op_result.1(&left_type, &right_type)?;

        let func_left: &'static str = {
            #[allow(clippy::enum_glob_use)]
            use Type::*;
            match left_type {
                Int => "int",
                Float => "float",
                String => "string",
                Bool => "bool",
                Void => {
                    return Err(std::string::String::from(
                        "Void type cannot be used in binary expressions",
                    ));
                }
            }
        };

        let func_right: &'static str = {
            #[allow(clippy::enum_glob_use)]
            use Type::*;
            match right_type {
                Int => "int",
                Float => "float",
                String => "string",
                Bool => "bool",
                Void => {
                    return Err(std::string::String::from(
                        "Void type cannot be used in binary expressions",
                    ));
                }
            }
        };

        let func_name: String = format!("rustmm_operator_{func_op}_{func_left}_{func_right}");
        let func_call: String = format!("{func_name}({left}, {right})");

        self.output.push_str(&func_call);

        Ok(result_type)
    }

    fn unary_expr(&mut self, operator: &UnaryOperator, operand: Box<Expr>) -> Result<Type, String> {
        type OpResult = (&'static str, fn(&Type) -> Result<Type, String>);
        let result: OpResult = {
            use UnaryOperator::Not;
            match operator {
                Not => ("-!", Type::not),
            }
        };

        let mut unary_compiler: Self = Self::default();

        let (operand_type, handled): (Type, bool) =
            self.binary_part_expr(&operand, &mut unary_compiler)?;
        if !handled {
            unary_compiler.expression(*operand)?;
        }
        let operand: String = unary_compiler.output.clone();
        unary_compiler.output.clear();

        let result_type: Type = result.1(&operand_type)?;

        if result.0.starts_with('-') {
            let op: &str = &result.0[1..];
            let operation: String = format!("{op}({operand})");
            self.output.push_str(&operation);
            return Ok(result_type);
        }

        /* Implement function calling logic like in binary_expr here if later needed. */

        Ok(result_type)
    }

    fn binary_part_expr(&self, expr: &Expr, compiler: &mut Self) -> Result<(Type, bool), String> {
        match expr.node.clone() {
            Expression::Literal(literal) => match literal {
                Literal::Integer(_) => Ok((Type::Int, false)),
                Literal::Float(_) => Ok((Type::Float, false)),
                Literal::String(_) => Ok((Type::String, false)),
                Literal::Boolean(_) => Ok((Type::Bool, false)),
            },
            Expression::Identifier(identifier) => self.environment.get(&identifier).map_or_else(
                || Err(format!("Undefined variable: {identifier}")),
                |var_type| Ok((var_type.clone(), false)),
            ),
            Expression::Binary {
                left,
                operator,
                right,
            } => Ok((compiler.binary_expr(left, &operator, right)?, true)),
            Expression::Unary { operator, operand } => {
                Ok((compiler.unary_expr(&operator, operand)?, true))
            }
            Expression::FunctionCall { name, .. } => {
                if self.environment.contains_key(&name) {
                    Ok((self.environment[&name].clone(), false))
                } else {
                    Err(format!("Undefined function: {name}"))
                }
            }
        }
    }

    fn func_call_expr(&mut self, name: &str, arguments: Vec<Expr>) -> Result<Type, String> {
        let uname: String = prefix(name);
        let bname: String = String::from("rustmm_builtin_") + name;

        let pname: String = if self.functions.contains_key(&bname) {
            if self.functions[&bname].1 == "builtin" {
                bname
            } else {
                unreachable!()
            }
        } else if self.functions.contains_key(&uname) {
            uname
        } else {
            return Err(format!("Undefined function: {name}"));
        };

        self.output.push_str(&pname);
        self.output.push('(');

        let param_types: Vec<(Type, String)> = self.functions[&pname].0.clone();

        if arguments.len() != param_types.len() {
            return Err(format!(
                "Function '{}' expects {} arguments, got {}",
                name,
                param_types.len(),
                arguments.len()
            ));
        }

        for (i, argument) in arguments.into_iter().enumerate() {
            if self.expression(argument)? == param_types[i].0 {
                if i < param_types.len() - 1 {
                    self.output.push_str(", ");
                }
            } else {
                return Err(format!(
                    "Argument {} for function '{}' has incorrect type",
                    i + 1,
                    name
                ));
            }
            if i < param_types.len() - 1 {
                self.output.push_str(", ");
            }
        }

        self.output.push(')');

        Ok(self.environment[name].clone())
    }
}

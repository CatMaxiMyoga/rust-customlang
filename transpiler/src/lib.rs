//! Main library file for the compiler module

mod csharp;

use parser::types::{
    BinaryOperator, Expr, Expression, Literal, Program, Statement, Stmt, UnaryOperator,
};

use crate::csharp::{Type, prefix};

const BUILTIN_FUNCTIONS: [&str; 14] = [
    "print",
    "println",
    "boolToString",
    "intToString",
    "floatToString",
    "stringToBool",
    "intToBool",
    "floatToBool",
    "stringToInt",
    "boolToInt",
    "floatToInt",
    "stringToFloat",
    "boolToFloat",
    "intToFloat",
];

/// Transpiler struct responsible for transpiling source code into C# code
#[derive(Debug, Clone)]
pub struct Transpiler {
    /// The output C# code
    output: String,
    /// The indent level for formatting
    indent_level: usize,
    /// Class declarations to be added at the end of the output
    class_declarations: String,
}

impl Transpiler {
    /// Transpiles the given source code into C# code
    ///
    /// # Errors
    /// When something goes wrong during transpilation, for example an invalid AST
    pub fn transpile(program: Program) -> Result<String, String> {
        let mut transpiler: Self = Self {
            output: String::new(),
            indent_level: 0,
            class_declarations: String::new(),
        };

        for statement in program.statements {
            transpiler.statement(statement)?;
        }

        transpiler.output.push_str("\n\n// CLASS DECLARATIONS\n\n");
        transpiler.output.push_str(&transpiler.class_declarations);

        Ok(transpiler.output)
    }

    fn indent(&mut self) {
        for _ in 0..self.indent_level {
            self.output.push_str("  ");
        }
    }

    fn expr_path(src: Expr) -> Result<String, String> {
        match src.node {
            Expression::Identifier(identifier) => Ok(prefix(&identifier)),
            Expression::Call { callee, arguments } => {
                let mut args: Vec<String> = Vec::new();

                let mut arg_compiler: Self = Self {
                    output: String::new(),
                    indent_level: 0,
                    class_declarations: String::new(),
                };

                for arg in &arguments {
                    arg_compiler.expression(arg.clone())?;
                    args.push(arg_compiler.output.clone());
                    arg_compiler.output.clear();
                }

                Ok(format!(
                    "{}({})",
                    Self::expr_path(*callee)?,
                    args.join(", ")
                ))
            }
            Expression::MemberAccess { object, member } => {
                Ok(format!("{}.{}", Self::expr_path(*object)?, prefix(&member)))
            }
            Expression::Self_ => Ok(String::from("this")),
            _ => Err(format!("Unsupported expression in path: {src:?}")),
        }
    }

    fn statement(&mut self, statement: Stmt) -> Result<(), String> {
        match statement.node {
            Statement::VariableDeclaration { type_, name, value } => {
                self.indent();
                self.variable_declaration_statement(&type_, &name, value)?;
            }
            Statement::FieldDeclaration {
                type_,
                name,
                static_,
            } => {
                self.indent();
                self.field_declaration_statement(&type_, &name, static_);
            }
            Statement::FunctionDeclaration {
                return_type,
                name,
                parameters,
                body,
            } => {
                self.indent();
                self.function_declaration_statement(&return_type, &name, &parameters, body)?;
                return Ok(());
            }
            Statement::MethodDeclaration {
                return_type,
                name,
                parameters,
                body,
                static_,
            } => {
                self.indent();
                self.method_declaration_statement(&return_type, &name, &parameters, body, static_)?;
                return Ok(());
            }
            Statement::ClassDeclaration { name, body } => {
                self.indent();
                self.class_declaration_statement(&name, body)?;
                return Ok(());
            }
            Statement::Assignment { assignee, value } => {
                self.indent();
                self.variable_assignment_statement(*assignee, value)?;
            }
            Statement::If {
                conditional_branches,
                else_branch,
            } => {
                self.if_statement(&conditional_branches, else_branch)?;
                return Ok(());
            }
            Statement::While { condition, body } => {
                self.while_loop_statement(condition, &body)?;
                return Ok(());
            }
            Statement::Return(ret) => {
                self.indent();
                self.return_statement(ret)?;
            }
            Statement::Expression(expr) => {
                self.indent();
                self.expression(expr)?;
            }
        }

        self.output.push_str(";\n");
        Ok(())
    }

    fn variable_declaration_statement(
        &mut self,
        type_: &str,
        name: &str,
        value: Option<Expr>,
    ) -> Result<(), String> {
        let type_: String = Type::from(type_);

        self.output.push_str(&type_);
        self.output.push(' ');
        self.output.push_str(&prefix(name));

        if let Some(expr) = value {
            self.output.push_str(" = ");
            self.expression(expr)?;
        }

        Ok(())
    }

    fn field_declaration_statement(&mut self, type_: &str, name: &str, static_: bool) {
        let type_: String = Type::from(type_.strip_prefix("##").unwrap_or(type_));

        self.output.push_str("public ");
        if static_ {
            self.output.push_str("static ");
        }
        self.output.push_str(&type_);
        self.output.push(' ');
        self.output.push_str(&prefix(name));
    }

    fn variable_assignment_statement(&mut self, assignee: Expr, value: Expr) -> Result<(), String> {
        let expr_path = Self::expr_path(assignee)?;
        self.output.push_str(&expr_path);
        self.output.push_str(" = ");
        self.expression(value)?;
        Ok(())
    }

    fn function_declaration_statement(
        &mut self,
        return_type: &str,
        name: &str,
        params: &[(String, String)],
        body: Vec<Stmt>,
    ) -> Result<(), String> {
        self.output.push_str(&Type::from(return_type));
        self.output.push(' ');
        self.output.push_str(&prefix(name));
        self.output.push('(');

        for (i, (type_, parameter_name)) in params.iter().enumerate() {
            self.output.push_str(&Type::from(type_));
            self.output.push(' ');
            self.output.push_str(&prefix(parameter_name));
            if i < params.len() - 1 {
                self.output.push_str(", ");
            }
        }

        self.output.push_str(") {\n");

        let mut function_compiler: Self = Self {
            output: String::new(),
            indent_level: self.indent_level + 1,
            class_declarations: String::new(),
        };

        for stmt in body {
            function_compiler.statement(stmt)?;
        }

        self.output.push_str(&function_compiler.output);
        self.indent();
        self.output.push_str("}\n\n");
        Ok(())
    }

    fn method_declaration_statement(
        &mut self,
        return_type: &str,
        name: &str,
        params: &[(String, String)],
        body: Vec<Stmt>,
        static_: bool,
    ) -> Result<(), String> {
        self.output.push_str("public ");

        let return_type: String = Type::from(return_type);
        let pname: String = prefix(name);

        if static_ {
            self.output.push_str("static ");
        }

        if static_ || return_type != pname {
            self.output.push_str(&return_type);
            self.output.push(' ');
        }

        self.output.push_str(&pname);
        self.output.push('(');

        for (i, (type_, parameter_name)) in params.iter().enumerate() {
            self.output.push_str(&Type::from(type_));
            self.output.push(' ');
            self.output.push_str(&prefix(parameter_name));
            if i < params.len() - 1 {
                self.output.push_str(", ");
            }
        }

        self.output.push_str(") {\n");

        let mut function_compiler: Self = Self {
            output: String::new(),
            indent_level: self.indent_level + 1,
            class_declarations: String::new(),
        };

        for stmt in body {
            function_compiler.statement(stmt)?;
        }

        self.output.push_str(&function_compiler.output);
        self.indent();
        self.output.push_str("}\n\n");
        Ok(())
    }

    fn class_declaration_statement(&mut self, name: &str, body: Vec<Stmt>) -> Result<(), String> {
        self.class_declarations.push_str("class ");
        self.class_declarations.push_str(&prefix(name));

        self.class_declarations.push_str(" {\n");

        let mut class_compiler: Self = Self {
            output: String::new(),
            indent_level: self.indent_level + 1,
            class_declarations: String::new(),
        };

        for stmt in body {
            class_compiler.statement(stmt)?;
        }

        self.class_declarations.push_str(&class_compiler.output);
        self.class_declarations.push_str("}\n\n");
        Ok(())
    }

    fn if_statement(
        &mut self,
        conditional_branches: &[(Expr, Vec<Stmt>)],
        else_branch: Option<Vec<Stmt>>,
    ) -> Result<(), String> {
        self.output.push('\n');
        self.indent();

        for (i, (condition, body)) in conditional_branches.iter().enumerate() {
            self.output
                .push_str(if i == 0 { "if " } else { "else if " });
            self.output.push('(');

            self.expression(condition.clone())?;
            self.output.push_str(") {\n");

            self.indent_level += 1;
            for stmt in body {
                self.statement(stmt.clone())?;
            }
            self.indent_level -= 1;

            self.indent();
            self.output.push_str("} ");
        }

        if let Some(else_block) = else_branch {
            self.output.push_str("else {\n");

            self.indent_level += 1;
            for stmt in else_block {
                self.statement(stmt.clone())?;
            }
            self.indent_level -= 1;

            self.indent();
            self.output.push('}');
        }

        self.output.push_str("\n\n");
        Ok(())
    }

    fn while_loop_statement(&mut self, condition: Expr, body: &[Stmt]) -> Result<(), String> {
        self.output.push('\n');
        self.indent();
        self.output.push_str("while (");

        self.expression(condition)?;

        self.output.push_str(") {\n");

        self.indent_level += 1;
        for stmt in body {
            self.statement(stmt.clone())?;
        }
        self.indent_level -= 1;

        self.indent();
        self.output.push_str("}\n\n");
        Ok(())
    }

    fn return_statement(&mut self, ret: Option<Expr>) -> Result<(), String> {
        self.output.push_str("return");

        if let Some(expr) = ret {
            self.output.push(' ');
            self.expression(expr)?;
        }
        Ok(())
    }

    fn expression(&mut self, expr: Expr) -> Result<(), String> {
        match expr.node {
            Expression::Literal(literal) => self.literal_expression(literal),
            Expression::Binary {
                left,
                operator,
                right,
            } => self.binary_expression(*left, &operator, *right)?,
            Expression::Unary { operator, operand } => {
                self.unary_expression(&operator, *operand)?;
            }
            Expression::Identifier(identifier) => self.output.push_str(&prefix(&identifier)),
            Expression::Call { callee, arguments } => {
                self.function_call_expression(*callee, &arguments)?;
            }
            Expression::MemberAccess { object, member } => {
                let var_name = Self::expr_path(*object)? + "." + &prefix(&member);
                self.output.push_str(&var_name);
            }
            Expression::Self_ => self.output.push_str("this"),
        }

        Ok(())
    }

    fn literal_expression(&mut self, literal: Literal) {
        match literal {
            Literal::Integer(value) => {
                self.output.push_str("new CustomLang.Types.rmm_Int(");
                self.output.push_str(&value.to_string());
                self.output.push(')');
            }
            Literal::Float(value) => {
                self.output.push_str("new CustomLang.Types.rmm_Float(");
                self.output.push_str(&value.to_string());
                self.output.push(')');
            }
            Literal::String(value) => {
                self.output.push_str("new CustomLang.Types.rmm_String(\"");
                self.output.push_str(&value);
                self.output.push_str("\")");
            }
            Literal::Boolean(value) => {
                self.output.push_str("new CustomLang.Types.rmm_Bool(");
                self.output.push_str(&value.to_string());
                self.output.push(')');
            }
        }
    }

    fn binary_expression(
        &mut self,
        left: Expr,
        operator: &BinaryOperator,
        right: Expr,
    ) -> Result<(), String> {
        self.expression(left)?;
        self.output.push('.');

        self.output.push_str(
            &(String::from("rmm__bop")
                + match operator {
                    BinaryOperator::Add => "Add",
                    BinaryOperator::Subtract => "Sub",
                    BinaryOperator::Multiply => "Mul",
                    BinaryOperator::Divide => "Div",
                    BinaryOperator::Equals => "Eq",
                    BinaryOperator::NotEquals => "Ne",
                    BinaryOperator::LessThan => "Lt",
                    BinaryOperator::GreaterThan => "Gt",
                    BinaryOperator::LessThanOrEqual => "Le",
                    BinaryOperator::GreaterThanOrEqual => "Ge",
                    BinaryOperator::And => "And",
                    BinaryOperator::Or => "Or",
                }),
        );

        self.output.push('(');
        self.expression(right)?;
        self.output.push(')');
        Ok(())
    }

    fn unary_expression(&mut self, operator: &UnaryOperator, operand: Expr) -> Result<(), String> {
        self.expression(operand)?;
        self.output.push('.');

        self.output.push_str(
            &(String::from("rmm__uop")
                + match operator {
                    UnaryOperator::Not => "Not",
                }),
        );

        self.output.push_str("()");
        Ok(())
    }

    fn function_call_expression(&mut self, callee: Expr, arguments: &[Expr]) -> Result<(), String> {
        let builtin: bool = if let Expression::Identifier(identifier) = callee.node.clone() {
            BUILTIN_FUNCTIONS.contains(&identifier.as_str())
        } else {
            false
        };

        let constructor_call: Option<String> = if let Expression::MemberAccess { object, member } =
            callee.node.clone()
            && let Expression::Identifier(identifier) = object.node
            && member == "new"
        {
            Some(identifier)
        } else {
            None
        };

        let prefixed_name: String = if let Some(name) = &constructor_call {
            format!("(new {}", prefix(name))
        } else {
            Self::expr_path(callee)?
        };

        if builtin {
            self.output.push_str("CustomLang.BuiltinFunctions");
            self.output.push('.');
        }

        self.output.push_str(&prefixed_name);
        self.output.push('(');

        for (i, argument) in arguments.iter().enumerate() {
            self.expression(argument.clone())?;
            if i < arguments.len() - 1 {
                self.output.push_str(", ");
            }
        }

        self.output.push(')');
        if constructor_call.is_some() {
            self.output.push(')');
        }
        Ok(())
    }
}

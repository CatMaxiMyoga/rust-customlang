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
}

impl Transpiler {
    /// Transpiles the given source code into C# code
    #[must_use]
    pub fn transpile(program: Program) -> String {
        let mut transpiler: Self = Self {
            output: String::new(),
            indent_level: 0,
        };

        for statement in program.statements {
            transpiler.statement(statement);
        }

        transpiler.output
    }

    fn indent(&mut self) {
        for _ in 0..self.indent_level {
            self.output.push_str("  ");
        }
    }

    fn statement(&mut self, statement: Stmt) {
        match statement.node {
            Statement::VariableDeclaration { type_, name, value } => {
                self.indent();
                self.variable_declaration_statement(&type_, &name, value);
            }
            Statement::VariableAssignment { name, value } => {
                self.indent();
                self.variable_assignment_statement(&name, value);
            }
            Statement::FunctionDeclaration {
                return_type,
                name,
                parameters,
                body,
            } => {
                self.function_declaration_statement(&return_type, &name, &parameters, body);
                return;
            }
            Statement::If {
                conditional_branches,
                else_branch,
            } => {
                self.if_statement(&conditional_branches, else_branch);
                return;
            }
            Statement::While { condition, body } => {
                self.while_loop_statement(condition, &body);
                return;
            }
            Statement::Return(ret) => self.return_statement(ret),
            Statement::Expression(expr) => {
                self.indent();
                self.expression(expr);
            }
        }

        self.output.push_str(";\n");
    }

    fn variable_declaration_statement(&mut self, type_: &str, name: &str, value: Option<Expr>) {
        let type_: String = Type::from(type_);

        self.output.push_str(&type_);
        self.output.push(' ');
        self.output.push_str(&prefix(name));

        if let Some(expr) = value {
            self.output.push_str(" = ");
            self.expression(expr);
        }
    }

    fn variable_assignment_statement(&mut self, name: &str, value: Expr) {
        self.output.push_str(&prefix(name));
        self.output.push_str(" = ");
        self.expression(value);
    }

    fn function_declaration_statement(
        &mut self,
        return_type: &str,
        name: &str,
        params: &[(String, String)],
        body: Vec<Stmt>,
    ) {
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
        };

        for stmt in body {
            function_compiler.statement(stmt);
        }

        self.output.push_str(&function_compiler.output);
        self.output.push_str("}\n\n");
    }

    fn if_statement(
        &mut self,
        conditional_branches: &[(Expr, Vec<Stmt>)],
        else_branch: Option<Vec<Stmt>>,
    ) {
        self.output.push('\n');
        self.indent();

        for (i, (condition, body)) in conditional_branches.iter().enumerate() {
            self.output
                .push_str(if i == 0 { "if " } else { "else if " });
            self.output.push('(');

            self.expression(condition.clone());
            self.output.push_str(") {\n");

            self.indent_level += 1;
            for stmt in body {
                self.statement(stmt.clone());
            }
            self.indent_level -= 1;

            self.indent();
            self.output.push_str("} ");
        }

        if let Some(else_block) = else_branch {
            self.output.push_str("else {\n");

            self.indent_level += 1;
            for stmt in else_block {
                self.statement(stmt.clone());
            }
            self.indent_level -= 1;

            self.indent();
            self.output.push('}');
        }

        self.output.push_str("\n\n");
    }

    fn while_loop_statement(&mut self, condition: Expr, body: &[Stmt]) {
        self.output.push('\n');
        self.indent();
        self.output.push_str("while (");

        self.expression(condition);

        self.output.push_str(") {\n");

        self.indent_level += 1;
        for stmt in body {
            self.statement(stmt.clone());
        }
        self.indent_level -= 1;

        self.indent();
        self.output.push_str("}\n\n");
    }

    fn return_statement(&mut self, ret: Option<Expr>) {
        self.output.push_str("return");

        if let Some(expr) = ret {
            self.output.push(' ');
            self.expression(expr);
        }
    }

    fn expression(&mut self, expr: Expr) {
        match expr.node {
            Expression::Literal(literal) => self.literal_expression(literal),
            Expression::Binary {
                left,
                operator,
                right,
            } => self.binary_expression(*left, &operator, *right),
            Expression::Unary { operator, operand } => self.unary_expression(&operator, *operand),
            Expression::Identifier(identifier) => self.output.push_str(&prefix(&identifier)),
            Expression::FunctionCall { name, arguments } => {
                self.function_call_expression(&name, &arguments);
            }
        }
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

    fn binary_expression(&mut self, left: Expr, operator: &BinaryOperator, right: Expr) {
        self.expression(left);
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
        self.expression(right);
        self.output.push(')');
    }

    fn unary_expression(&mut self, operator: &UnaryOperator, operand: Expr) {
        self.expression(operand);
        self.output.push('.');

        self.output.push_str(
            &(String::from("rmm__uop")
                + match operator {
                    UnaryOperator::Not => "Not",
                }),
        );

        self.output.push_str("()");
    }

    fn function_call_expression(&mut self, name: &str, arguments: &[Expr]) {
        let builtin: bool = BUILTIN_FUNCTIONS.contains(&name);
        let prefixed_name: &str = &prefix(name);

        if builtin {
            self.output.push_str("CustomLang.BuiltinFunctions");
            self.output.push('.');
        }

        self.output.push_str(prefixed_name);
        self.output.push('(');

        for (i, argument) in arguments.iter().enumerate() {
            self.expression(argument.clone());
            if i < arguments.len() - 1 {
                self.output.push_str(", ");
            }
        }

        self.output.push(')');
    }
}

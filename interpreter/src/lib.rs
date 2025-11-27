//! Contains the interpreter for the programming language

pub mod types;

use parser::types::{Expression, Literal, Operator, Program, Statement};
use types::{ExpressionResult, RuntimeError, RuntimeValue};

use crate::types::{Scope, StatementResult, Type};

/// The interpreter for the programming language.
pub struct Interpreter<'a> {
    scope: &'a mut Scope,
}

impl<'a> Interpreter<'a> {
    /// Interprets the AST and executes the program.
    ///
    /// # Errors
    /// Errors if runtime errors.
    pub fn run(program: Program, scope: &'a mut Scope) -> Result<(), RuntimeError> {
        let mut interpreter: Self = Self { scope };
        interpreter.builtins()?;

        for statement in program.statements {
            interpreter.statement(statement)?;
        }

        Ok(())
    }

    #[cfg(test)]
    const fn new(scope: &'a mut Scope) -> Self {
        Self { scope }
    }

    fn builtins(&mut self) -> Result<(), RuntimeError> {
        self.scope.variables.insert(
            String::from("print"),
            (
                Type::new("Void")?,
                Some(RuntimeValue::BuiltinFunction {
                    parameters: vec!["String"],
                    implementation: |_, args: Vec<RuntimeValue>| {
                        match &args[0] {
                            RuntimeValue::String(c) => print!("{c}"),
                            _ => unreachable!(),
                        }
                        Ok(RuntimeValue::Void)
                    },
                }),
            ),
        );

        self.scope.variables.insert(
            String::from("println"),
            (
                Type::new("Void")?,
                Some(RuntimeValue::BuiltinFunction {
                    parameters: vec!["String"],
                    implementation: |_, args: Vec<RuntimeValue>| {
                        match &args[0] {
                            RuntimeValue::String(c) => println!("{c}"),
                            _ => unreachable!(),
                        }
                        Ok(RuntimeValue::Void)
                    },
                }),
            ),
        );

        self.scope.variables.insert(
            String::from("IntToString"),
            (
                Type::new("String")?,
                Some(RuntimeValue::BuiltinFunction {
                    parameters: vec!["Integer"],
                    implementation: |_, args: Vec<RuntimeValue>| match &args[0] {
                        RuntimeValue::Integer(i) => Ok(RuntimeValue::String(i.to_string())),
                        _ => unreachable!(),
                    },
                }),
            ),
        );

        self.scope.variables.insert(
            String::from("FloatToString"),
            (
                Type::new("String")?,
                Some(RuntimeValue::BuiltinFunction {
                    parameters: vec!["Float"],
                    implementation: |_, args: Vec<RuntimeValue>| match &args[0] {
                        RuntimeValue::Float(i) => Ok(RuntimeValue::String(i.to_string())),
                        _ => unreachable!(),
                    },
                }),
            ),
        );

        self.scope.variables.insert(
            String::from("BoolToString"),
            (
                Type::new("String")?,
                Some(RuntimeValue::BuiltinFunction {
                    parameters: vec!["Boolean"],
                    implementation: |_, args: Vec<RuntimeValue>| match &args[0] {
                        RuntimeValue::Boolean(i) => Ok(RuntimeValue::String(i.to_string())),
                        _ => unreachable!(),
                    },
                }),
            ),
        );

        Ok(())
    }

    fn statement(&mut self, statement: Statement) -> Result<(), RuntimeError> {
        match statement {
            Statement::Expression(expr) => _ = self.expression(expr)?,
            Statement::VariableDeclaration { type_, name, value } => {
                self.variable_declaration_statement(&type_, &name, value)?;
            }
            Statement::VariableAssignment { name, value } => {
                self.variable_assignment_statement(&name, value)?;
            }
            Statement::FunctionDeclaration {
                return_type,
                name,
                parameters,
                body,
            } => self.function_declaration_statement(&return_type, &name, &parameters, body)?,
            Statement::Return(_) => return Err(RuntimeError::IllegalReturn),
        }

        Ok(())
    }

    fn variable_declaration_statement(
        &mut self,
        type_: &str,
        name: &str,
        value: Option<Expression>,
    ) -> StatementResult {
        if let Some((_, Some(old_var))) = self.scope.variables.get(name)
            && (old_var.get_name() == "Function" || old_var.get_name() == "Builtin (Function)")
        {
            return Err(RuntimeError::IllegalOperation(format!(
                "Cannot declare variable '{name}' with same name as function"
            )));
        }

        let value: Option<RuntimeValue> = if let Some(expr) = value {
            Some(self.expression(expr)?)
        } else {
            None
        };

        if value == Some(RuntimeValue::Void) {
            return Err(RuntimeError::TypeMismatch(
                "Cannot assign 'Void' type to variable.".to_owned(),
            ));
        }

        if type_ == "Void" {
            return Err(RuntimeError::TypeMismatch(
                "Cannot declare variable of type 'Void'.".to_owned(),
            ));
        }

        self.scope
            .variables
            .insert(name.to_owned(), (Type::new(type_)?, value));

        Ok(())
    }

    fn variable_assignment_statement(&mut self, name: &str, value: Expression) -> StatementResult {
        let old: (Type, Option<RuntimeValue>) = if let Some(val) = self.scope.variables.get(name) {
            val.clone()
        } else {
            return Err(RuntimeError::VariableNotFound(name.to_owned()));
        };

        if old.0.0 == "Function" || old.0.0 == "Builtin (Function)" {
            return Err(RuntimeError::TypeMismatch(format!(
                "Cannot assign value to function variable '{name}'"
            )));
        }

        let value: RuntimeValue = self.expression(value)?;

        if old.0.0 != value.get_name() {
            let old_name: String = old.0.0;
            let value_name: &'static str = value.get_name();
            return Err(RuntimeError::TypeMismatch(format!(
                "Cannot assign value of type '{value_name}' to variable of type '{old_name}'"
            )));
        }

        if value == RuntimeValue::Void {
            return Err(RuntimeError::TypeMismatch(
                "Cannot assign 'Void' type to variable.".to_owned(),
            ));
        }

        self.scope
            .variables
            .insert(name.to_owned(), (old.0, Some(value)));

        Ok(())
    }

    fn function_declaration_statement(
        &mut self,
        return_type: &str,
        name: &str,
        parameters: &Vec<(String, String)>,
        body: Vec<Statement>,
    ) -> StatementResult {
        if self.scope.variables.contains_key(name) {
            return Err(RuntimeError::NameConflict(format!(
                "Cannot create function '{name}', identifier already exists in current scope."
            )));
        }

        let mut params: Vec<(Type, String)> = Vec::new();
        for (type_, name) in parameters {
            params.push((Type::new(type_)?, name.to_owned()));
        }

        let function: RuntimeValue = RuntimeValue::Function {
            parameters: params,
            body,
        };
        self.scope
            .variables
            .insert(name.to_owned(), (Type::new(return_type)?, Some(function)));

        Ok(())
    }

    fn expression(&mut self, expression: Expression) -> ExpressionResult {
        match expression {
            Expression::Literal(literal) => Ok(Self::literal_expression(&literal)),
            Expression::Binary {
                left,
                operator,
                right,
            } => self.binary_expression(*left, &operator, *right),
            Expression::Identifier(identifier) => {
                if self.scope.variables.contains_key(&identifier) {
                    self.scope.variables[&identifier].1.as_ref().map_or_else(
                        || Err(RuntimeError::VariableUninitialized(identifier)),
                        |value| Ok(value.clone()),
                    )
                } else if let Some(variable) = &self.scope.find_in_parent(&identifier) {
                    match variable {
                        (_, Some(value)) => match value {
                            RuntimeValue::Function { .. }
                            | RuntimeValue::BuiltinFunction { .. } => Ok(value.clone()),
                            _ => Err(RuntimeError::VariableNotFound(identifier)),
                        },
                        (_, None) => Err(RuntimeError::VariableUninitialized(identifier)),
                    }
                } else {
                    Err(RuntimeError::VariableNotFound(identifier))
                }
            }
            Expression::FunctionCall { name, arguments } => {
                self.function_call_expression(&name, &arguments)
            }
        }
    }

    fn literal_expression(literal: &Literal) -> RuntimeValue {
        match literal {
            Literal::Integer(value) => RuntimeValue::Integer(*value),
            Literal::Float(value) => RuntimeValue::Float(*value),
            Literal::String(value) => RuntimeValue::String(value.clone()),
            Literal::Boolean(value) => RuntimeValue::Boolean(*value),
        }
    }

    fn binary_expression(
        &mut self,
        left: Expression,
        operator: &Operator,
        right: Expression,
    ) -> ExpressionResult {
        let left: RuntimeValue = self.expression(left)?;
        let right: RuntimeValue = self.expression(right)?;

        match operator {
            Operator::Add => left + right,
            Operator::Subtract => left - right,
            Operator::Multiply => left * right,
            Operator::Divide => left / right,
            Operator::Equals => left.eq(&right),
            Operator::NotEquals => left.ne(&right),
            Operator::LessThan => left.lt(&right),
            Operator::GreaterThan => left.gt(&right),
            Operator::LessThanOrEqual => left.le(&right),
            Operator::GreaterThanOrEqual => left.ge(&right),
        }
    }

    fn function_call_expression(
        &mut self,
        name: &str,
        arguments: &[Expression],
    ) -> ExpressionResult {
        type FunctionData = (Type, Vec<(Type, String)>, Vec<Statement>);

        if !self.scope.variables.contains_key(name) {
            if let Some((_, Some(func))) = self.scope.find_in_parent(name) {
                match func {
                    RuntimeValue::BuiltinFunction { .. } => {
                        return self.builtin_function_call_expression(name, arguments);
                    }
                    RuntimeValue::Function { .. } => {}
                    _ => {
                        return Err(RuntimeError::VariableNotFound(name.to_owned()));
                    }
                }
            } else {
                return Err(RuntimeError::VariableNotFound(name.to_owned()));
            }
        }

        let function: FunctionData = match self.scope.variables.get(name) {
            Some((return_type, Some(RuntimeValue::Function { parameters, body }))) => {
                (return_type.to_owned(), parameters.clone(), body.clone())
            }
            Some((_, Some(RuntimeValue::BuiltinFunction { .. }))) => {
                return self.builtin_function_call_expression(name, arguments);
            }
            None => {
                if let Some((return_type, Some(func))) = self.scope.find_in_parent(name) {
                    match func {
                        RuntimeValue::BuiltinFunction { .. } => {
                            return self.builtin_function_call_expression(name, arguments);
                        }
                        RuntimeValue::Function { parameters, body } => {
                            (return_type.to_owned(), parameters.clone(), body.clone())
                        }
                        _ => {
                            return Err(RuntimeError::VariableNotFound(name.to_owned()));
                        }
                    }
                } else {
                    return Err(RuntimeError::VariableNotFound(name.to_owned()));
                }
            }
            _ => {
                return Err(RuntimeError::TypeMismatch(format!(
                    "Tried to call non-function variable '{name}'"
                )));
            }
        };

        let mut scope: Scope = Scope::with_parent(self.scope.clone());

        let mut interpreter: Interpreter = Interpreter { scope: &mut scope };

        if arguments.len() != function.1.len() {
            return Err(RuntimeError::IllegalArgumentCount(arguments.len()));
        }

        for (i, (type_, name)) in function.1.iter().enumerate() {
            let argument_value: RuntimeValue = self.expression(arguments[i].clone())?;
            interpreter
                .scope
                .variables
                .insert(name.clone(), (type_.clone(), Some(argument_value)));
        }

        for statement in function.2.iter().cloned() {
            if let Statement::Return(value) = statement {
                let value: RuntimeValue = interpreter.expression(value)?;
                if function.0.0 != value.get_name() {
                    return Err(RuntimeError::TypeMismatch(
                        format!(
                            "Function '{name}' expected to return type '{}' ",
                            function.0.0,
                        ) + &format!("but returned type '{}'", value.get_name()),
                    ));
                }
                return Ok(value);
            }
            interpreter.statement(statement)?;
        }

        if function.0.0 != "Void" {
            return Err(RuntimeError::TypeMismatch(format!(
                "Function '{name}' expected to return type '{}', but no return statement was found.",
                function.0.0
            )));
        }

        Ok(RuntimeValue::Void)
    }

    fn builtin_function_call_expression(
        &mut self,
        name: &str,
        arguments: &[Expression],
    ) -> ExpressionResult {
        type BuiltinFunctionImpl = fn(&mut Scope, Vec<RuntimeValue>) -> ExpressionResult;
        type BuiltinFunction = (Type, Vec<&'static str>, BuiltinFunctionImpl);

        let builtin: BuiltinFunction = match self.scope.variables.get(name) {
            Some((
                return_type,
                Some(RuntimeValue::BuiltinFunction {
                    parameters,
                    implementation,
                }),
            )) => (return_type.clone(), parameters.clone(), *implementation),
            _ => {
                unreachable!()
            }
        };

        if arguments.len() != builtin.1.len() {
            return Err(RuntimeError::IllegalArgumentCount(arguments.len()));
        }

        let mut args: Vec<RuntimeValue> = Vec::new();

        for (i, type_) in builtin.1.iter().enumerate() {
            let argument: RuntimeValue = self.expression(arguments[i].clone())?;

            if argument.get_name() != *type_ {
                return Err(RuntimeError::TypeMismatch(
                    format!(
                        "Builtin function '{name}' expected argument {} to be of type '{}' ",
                        i + 1,
                        type_,
                    ) + &format!("but got type '{}'", argument.get_name()),
                ));
            }

            args.push(argument);
        }

        let result: RuntimeValue = builtin.2(self.scope, args)?;

        if builtin.0.0 != result.get_name() {
            return Err(RuntimeError::TypeMismatch(
                format!(
                    "Builtin function '{name}' expected to return type '{}' ",
                    builtin.0.0,
                ) + &format!("but returned type '{}'", result.get_name()),
            ));
        }
        Ok(result)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    macro_rules! test_all_ops {
        ($name:ident, $left:expr, $right:expr, $result:expr, $op:ident, $suffix:ident) => {
            paste::paste! {
                #[test]
                fn [<$name _ $suffix>]() {
                    let mut scope: Scope = Scope::default();
                    let mut interpreter: Interpreter = Interpreter::new(&mut scope);
                    let expression: Expression = Expression::Binary{
                        left: Box::new($left),
                        operator: Operator::$op,
                        right: Box::new($right)
                    };
                    let result: RuntimeValue = interpreter.expression(expression).unwrap();
                    assert_eq!(result, $result);
                }
            }
        };
        ($name:ident, $left:expr, $right:expr, $add:expr, $sub:expr, $mul:expr, $div:expr) => {
            test_all_ops!($name, $left, $right, $add, Add, addition);
            test_all_ops!($name, $left, $right, $sub, Subtract, subtraction);
            test_all_ops!($name, $left, $right, $mul, Multiply, multiplication);
            test_all_ops!($name, $left, $right, $div, Divide, division);
        };
    }

    test_all_ops!(
        integer,
        Expression::Literal(Literal::Integer(5)),
        Expression::Literal(Literal::Integer(2)),
        RuntimeValue::Integer(7),
        RuntimeValue::Integer(3),
        RuntimeValue::Integer(10),
        RuntimeValue::Integer(2)
    );

    test_all_ops!(
        float,
        Expression::Literal(Literal::Float(5.0)),
        Expression::Literal(Literal::Float(2.0)),
        RuntimeValue::Float(7.0),
        RuntimeValue::Float(3.0),
        RuntimeValue::Float(10.0),
        RuntimeValue::Float(2.5)
    );

    test_all_ops!(
        mixed,
        Expression::Literal(Literal::Integer(5)),
        Expression::Literal(Literal::Float(2.0)),
        RuntimeValue::Float(7.0),
        RuntimeValue::Float(3.0),
        RuntimeValue::Float(10.0),
        RuntimeValue::Float(2.5)
    );

    test_all_ops!(
        mixed_reverse,
        Expression::Literal(Literal::Float(5.0)),
        Expression::Literal(Literal::Integer(2)),
        RuntimeValue::Float(7.0),
        RuntimeValue::Float(3.0),
        RuntimeValue::Float(10.0),
        RuntimeValue::Float(2.5)
    );

    #[test]
    fn variable_declaration() {
        let mut scope: Scope = Scope::default();
        let mut interpreter: Interpreter = Interpreter::new(&mut scope);
        let declaration: Statement = Statement::VariableDeclaration {
            type_: String::from("Integer"),
            name: String::from("x"),
            value: None,
        };
        interpreter.statement(declaration).unwrap();
        assert!(scope.variables.contains_key("x"));
        assert!(scope.variables["x"].1.is_none());
    }

    #[test]
    fn variable_initialization() {
        let mut scope: Scope = Scope::default();
        let mut interpreter: Interpreter = Interpreter::new(&mut scope);

        let declaration: Statement = Statement::VariableDeclaration {
            type_: String::from("Integer"),
            name: String::from("x"),
            value: Some(Expression::Literal(Literal::Integer(10))),
        };
        interpreter.statement(declaration).unwrap();

        assert!(scope.variables.contains_key("x"));
        assert_eq!(scope.variables["x"].1, Some(RuntimeValue::Integer(10)));
    }

    #[test]
    fn variable_delayed_initialization() {
        let mut scope: Scope = Scope::default();
        scope
            .variables
            .insert(String::from("x"), (Type::new("Float").unwrap(), None));
        let mut interpreter: Interpreter = Interpreter::new(&mut scope);

        let assignment: Statement = Statement::VariableAssignment {
            name: String::from("x"),
            value: Expression::Literal(Literal::Float(20.0)),
        };
        interpreter.statement(assignment).unwrap();

        assert!(scope.variables.contains_key("x"));
        assert_eq!(scope.variables["x"].1, Some(RuntimeValue::Float(20.0)));
    }

    #[test]
    fn variable_reassignment() {
        let mut scope: Scope = Scope::default();
        scope.variables.insert(
            String::from("x"),
            (
                Type::new("Integer").unwrap(),
                Some(RuntimeValue::Integer(10)),
            ),
        );
        let mut interpreter: Interpreter = Interpreter::new(&mut scope);

        let assignment: Statement = Statement::VariableAssignment {
            name: String::from("x"),
            value: Expression::Literal(Literal::Integer(30)),
        };
        interpreter.statement(assignment).unwrap();

        assert!(scope.variables.contains_key("x"));
        assert_eq!(scope.variables["x"].1, Some(RuntimeValue::Integer(30)));
    }

    #[test]
    fn variable_type_mismatch() {
        let mut scope: Scope = Scope::default();
        scope.variables.insert(
            String::from("x"),
            (
                Type::new("Integer").unwrap(),
                Some(RuntimeValue::Integer(10)),
            ),
        );
        let mut interpreter: Interpreter = Interpreter::new(&mut scope);

        let assignment: Statement = Statement::VariableAssignment {
            name: String::from("x"),
            value: Expression::Literal(Literal::Float(20.0)),
        };
        let result: Result<(), RuntimeError> = interpreter.statement(assignment);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            RuntimeError::TypeMismatch(String::from(
                "Cannot assign value of type 'Float' to variable of type 'Integer'"
            ))
        );
    }

    #[test]
    fn string_arithmetic() {
        let mut scope: Scope = Scope::default();
        let mut interpreter: Interpreter = Interpreter::new(&mut scope);

        let expression: Expression = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::String(String::from("hello")))),
            operator: Operator::Multiply,
            right: Box::new(Expression::Literal(Literal::Integer(5))),
        };
        let result: ExpressionResult = interpreter.expression(expression);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            RuntimeError::IllegalOperation(String::from(
                "Multiplication not supported for String type"
            ))
        );
    }

    #[test]
    fn string_concatenation() {
        let mut scope: Scope = Scope::default();
        let mut interpreter: Interpreter = Interpreter::new(&mut scope);
        let expression: Expression = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::String(String::from("hello")))),
            operator: Operator::Add,
            right: Box::new(Expression::Literal(Literal::String(String::from("world")))),
        };
        let result: RuntimeValue = interpreter.expression(expression).unwrap();
        assert_eq!(result, RuntimeValue::String(String::from("helloworld")));
    }

    #[test]
    fn boolean_literal() {
        let mut scope: Scope = Scope::default();
        let mut interpreter: Interpreter = Interpreter::new(&mut scope);
        let expression: Expression = Expression::Literal(Literal::Boolean(true));
        let result: RuntimeValue = interpreter.expression(expression).unwrap();
        assert_eq!(result, RuntimeValue::Boolean(true));
        let expression: Expression = Expression::Literal(Literal::Boolean(false));
        let result: RuntimeValue = interpreter.expression(expression).unwrap();
        assert_eq!(result, RuntimeValue::Boolean(false));
    }
}

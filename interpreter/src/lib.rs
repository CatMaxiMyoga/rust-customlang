//! Contains the interpreter for the programming language

pub mod types;

use parser::types::{Expression, Literal, Operator, Program, Statement};
use std::mem::discriminant;
use types::{ExpressionResult, RuntimeError, RuntimeValue};

use crate::types::{Environment, StatementResult};

/// The interpreter for the programming language.
pub struct Interpreter<'a> {
    environment: &'a mut Environment,
}

impl<'a> Interpreter<'a> {
    /// Interprets the AST and executes the program.
    ///
    /// # Errors
    /// Errors if runtime errors.
    pub fn run(program: Program, environment: &'a mut Environment) -> Result<(), RuntimeError> {
        let mut interpreter: Self = Self { environment };
        interpreter.builtins();

        for statement in program.statements {
            interpreter.statement(statement)?;
        }

        Ok(())
    }

    #[cfg(test)]
    const fn new(environment: &'a mut Environment) -> Self {
        Self { environment }
    }

    fn builtins(&mut self) {
        self.environment.insert(
            String::from("print"),
            Some(RuntimeValue::BuiltinFunction {
                parameters: 1,
                implementation: |_, args| {
                    match &args[0] {
                        RuntimeValue::String(c) => print!("{c}"),
                        RuntimeValue::Integer(c) => print!("{c}"),
                        RuntimeValue::Float(c) => print!("{c}"),
                        RuntimeValue::Boolean(c) => print!("{c}"),
                        _ => {
                            return Err(RuntimeError::TypeMismatch(
                                "Unsupported type for print function".to_owned(),
                            ));
                        }
                    }
                    Ok(RuntimeValue::Void)
                },
            }),
        );

        self.environment.insert(
            String::from("println"),
            Some(RuntimeValue::BuiltinFunction {
                parameters: 1,
                implementation: |_, args| {
                    match &args[0] {
                        RuntimeValue::String(c) => println!("{c}"),
                        RuntimeValue::Integer(c) => println!("{c}"),
                        RuntimeValue::Float(c) => println!("{c}"),
                        RuntimeValue::Boolean(c) => println!("{c}"),
                        _ => {
                            return Err(RuntimeError::TypeMismatch(
                                "Unsupported type for println function".to_owned(),
                            ));
                        }
                    }
                    Ok(RuntimeValue::Void)
                },
            }),
        );
    }

    fn statement(&mut self, statement: Statement) -> Result<(), RuntimeError> {
        match statement {
            Statement::Expression(expr) => _ = self.expression(expr)?,
            Statement::VariableDeclaration { name, value } => {
                self.variable_declaration_statement(&name, value)?;
            }
            Statement::VariableAssignment { name, value } => {
                self.variable_assignment_statement(&name, value)?;
            }
            Statement::FunctionDeclaration {
                name,
                parameters,
                body,
            } => self.function_declaration_statement(&name, parameters, body)?,
            Statement::Return(_) => return Err(RuntimeError::IllegalReturn),
        }

        Ok(())
    }

    fn variable_declaration_statement(
        &mut self,
        name: &str,
        value: Option<Expression>,
    ) -> StatementResult {
        if let Some(Some(old_var)) = self.environment.get(name)
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

        self.environment.insert(name.to_owned(), value);

        Ok(())
    }

    fn variable_assignment_statement(&mut self, name: &str, value: Expression) -> StatementResult {
        let old: Option<RuntimeValue> = if let Some(val) = self.environment.get(name) {
            val.clone()
        } else {
            return Err(RuntimeError::VaiableNotFound(name.to_owned()));
        };

        if let Some(old_var) = old.clone()
            && (old_var.get_name() == "Function" || old_var.get_name() == "Builtin (Function)")
        {
            return Err(RuntimeError::TypeMismatch(format!(
                "Cannot assign value to function variable '{name}'"
            )));
        }

        let value: RuntimeValue = self.expression(value)?;

        if let Some(old) = old
            && discriminant(&old) != discriminant(&value)
        {
            let old_name: &'static str = old.get_name();
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

        self.environment.insert(name.to_owned(), Some(value));

        Ok(())
    }

    fn function_declaration_statement(
        &mut self,
        name: &str,
        parameters: Vec<String>,
        body: Vec<Statement>,
    ) -> StatementResult {
        if self.environment.contains_key(name) {
            return Err(RuntimeError::NameConflict(format!(
                "Cannot create function '{name}', identifier already exists in current scope."
            )));
        }

        let function: RuntimeValue = RuntimeValue::Function { parameters, body };
        self.environment.insert(name.to_owned(), Some(function));

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
                if self.environment.contains_key(&identifier) {
                    self.environment[&identifier].as_ref().map_or_else(
                        || Err(RuntimeError::VariableUninitialized(identifier)),
                        |value| Ok(value.clone()),
                    )
                } else {
                    Err(RuntimeError::VaiableNotFound(identifier))
                }
            }
            Expression::FunctionCall { name, arguments } => {
                self.function_call_expression(&name, arguments)
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
        }
    }

    fn function_call_expression(
        &mut self,
        name: &str,
        arguments: Vec<Expression>,
    ) -> ExpressionResult {
        if !self.environment.contains_key(name) {
            return Err(RuntimeError::VaiableNotFound(name.to_owned()));
        }

        let (parameters, body): (Vec<String>, Vec<Statement>) = match self.environment.get(name) {
            Some(Some(RuntimeValue::Function { parameters, body })) => {
                (parameters.clone(), body.clone())
            }
            Some(Some(RuntimeValue::BuiltinFunction {
                parameters: _,
                implementation: _,
            })) => {
                return self.builtin_function_call_expression(name, arguments);
            }
            _ => {
                return Err(RuntimeError::TypeMismatch(format!(
                    "Tried to call non-function variable '{name}'"
                )));
            }
        };

        let mut environment: Environment = Environment::new();

        for (key, value) in self.environment.iter() {
            environment.insert(key.clone(), value.clone());
        }

        let mut interpreter: Interpreter = Interpreter {
            environment: &mut environment,
        };

        if arguments.len() != parameters.len() {
            return Err(RuntimeError::IllegalArgumentCount(arguments.len()));
        }

        for (i, parameter) in parameters.iter().enumerate() {
            let argument_value: RuntimeValue = self.expression(arguments[i].clone())?;
            interpreter.environment.insert(parameter.clone(), Some(argument_value));
        }

        for statement in body.iter().cloned() {
            if let Statement::Return(value) = statement {
                return interpreter.expression(value);
            }
            interpreter.statement(statement)?;
        }

        Ok(RuntimeValue::Void)
    }

    fn builtin_function_call_expression(
        &mut self,
        name: &str,
        arguments: Vec<Expression>,
    ) -> ExpressionResult {
        type BuiltinFunctionImpl = fn(&mut Environment, Vec<RuntimeValue>) -> ExpressionResult;
        type BuiltinFunction = (usize, BuiltinFunctionImpl);

        let builtin: BuiltinFunction = match self.environment.get(name) {
            Some(Some(RuntimeValue::BuiltinFunction {
                parameters,
                implementation,
            })) => (*parameters, *implementation),
            _ => {
                unreachable!()
            }
        };

        if arguments.len() != builtin.0 {
            return Err(RuntimeError::IllegalArgumentCount(arguments.len()));
        }

        let mut args: Vec<RuntimeValue> = Vec::new();

        for argument in arguments {
            args.push(self.expression(argument)?);
        }

        builtin.1(self.environment, args)
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
                    let mut environment: Environment = Environment::new();
                    let mut interpreter: Interpreter = Interpreter::new(&mut environment);
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
        let mut environment: Environment = Environment::new();
        let mut interpreter: Interpreter = Interpreter::new(&mut environment);
        let declaration: Statement = Statement::VariableDeclaration {
            name: String::from("x"),
            value: None,
        };
        interpreter.statement(declaration).unwrap();
        assert!(environment.contains_key("x"));
        assert!(environment["x"].is_none());
    }

    #[test]
    fn variable_initialization() {
        let mut environment: Environment = Environment::new();
        let mut interpreter: Interpreter = Interpreter::new(&mut environment);

        let declaration: Statement = Statement::VariableDeclaration {
            name: String::from("x"),
            value: Some(Expression::Literal(Literal::Integer(10))),
        };
        interpreter.statement(declaration).unwrap();

        assert!(environment.contains_key("x"));
        assert_eq!(environment["x"], Some(RuntimeValue::Integer(10)));
    }

    #[test]
    fn variable_delayed_initialization() {
        let mut environment: Environment = Environment::new();
        environment.insert(String::from("x"), None);
        let mut interpreter: Interpreter = Interpreter::new(&mut environment);

        let assignment: Statement = Statement::VariableAssignment {
            name: String::from("x"),
            value: Expression::Literal(Literal::Float(20.0)),
        };
        interpreter.statement(assignment).unwrap();

        assert!(environment.contains_key("x"));
        assert_eq!(environment["x"], Some(RuntimeValue::Float(20.0)));
    }

    #[test]
    fn variable_reassignment() {
        let mut environment: Environment = Environment::new();
        environment.insert(String::from("x"), Some(RuntimeValue::Integer(10)));
        let mut interpreter: Interpreter = Interpreter::new(&mut environment);

        let assignment: Statement = Statement::VariableAssignment {
            name: String::from("x"),
            value: Expression::Literal(Literal::Integer(30)),
        };
        interpreter.statement(assignment).unwrap();

        assert!(environment.contains_key("x"));
        assert_eq!(environment["x"], Some(RuntimeValue::Integer(30)));
    }

    #[test]
    fn variable_type_mismatch() {
        let mut environment: Environment = Environment::new();
        environment.insert(String::from("x"), Some(RuntimeValue::Integer(10)));
        let mut interpreter: Interpreter = Interpreter::new(&mut environment);

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
        let mut environment: Environment = Environment::new();
        let mut interpreter: Interpreter = Interpreter::new(&mut environment);

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
        let mut environment: Environment = Environment::new();
        let mut interpreter: Interpreter = Interpreter::new(&mut environment);
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
        let mut environment: Environment = Environment::new();
        let mut interpreter: Interpreter = Interpreter::new(&mut environment);
        let expression: Expression = Expression::Literal(Literal::Boolean(true));
        let result: RuntimeValue = interpreter.expression(expression).unwrap();
        assert_eq!(result, RuntimeValue::Boolean(true));
        let expression: Expression = Expression::Literal(Literal::Boolean(false));
        let result: RuntimeValue = interpreter.expression(expression).unwrap();
        assert_eq!(result, RuntimeValue::Boolean(false));
    }
}

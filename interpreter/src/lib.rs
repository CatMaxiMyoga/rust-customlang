//! Contains the interpreter for the programming language

pub mod types;

use parser::types::{Expression, Literal, Operator, Program, Statement};
use std::mem::discriminant;
use types::{RuntimeError, RuntimeResult, RuntimeValue};

use crate::types::Environment;

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
        for statement in program.statements {
            interpreter.statement(statement)?;
        }

        Ok(())
    }

    #[cfg(test)]
    const fn new(environment: &'a mut Environment) -> Self {
        Self { environment }
    }

    fn statement(&mut self, statement: Statement) -> Result<(), RuntimeError> {
        match statement {
            Statement::Expression(expr) => {
                // TEMP: prints the result of expression statements
                let expression_result: RuntimeValue = self.expression(expr)?;
                println!("{expression_result:?}");
            }
            Statement::VariableDeclaration { name, value } => {
                let value: Option<RuntimeValue> = if let Some(expr) = value {
                    Some(self.expression(expr)?)
                } else {
                    None
                };
                self.environment.insert(name, value);
            }
            Statement::VariableAssignment { name, value } => {
                let old: Option<RuntimeValue> = if let Some(val) = self.environment.get(&name) {
                    val.clone()
                } else {
                    return Err(RuntimeError::VaiableNotFound(name));
                };

                let value: RuntimeValue = self.expression(value)?;

                if let Some(old) = old
                    && discriminant(&old) != discriminant(&value)
                {
                    return Err(RuntimeError::TypeMismatch);
                }

                self.environment.insert(name, Some(value));
            }
        }

        Ok(())
    }

    fn expression(&mut self, expression: Expression) -> RuntimeResult {
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
        }
    }

    fn literal_expression(literal: &Literal) -> RuntimeValue {
        match literal {
            Literal::Integer(value) => RuntimeValue::Integer(*value),
            Literal::Float(value) => RuntimeValue::Float(*value),
            Literal::String(value) => RuntimeValue::String(value.clone()),
        }
    }

    fn binary_expression(
        &mut self,
        left: Expression,
        operator: &Operator,
        right: Expression,
    ) -> RuntimeResult {
        let left: RuntimeValue = self.expression(left)?;
        let right: RuntimeValue = self.expression(right)?;

        match operator {
            Operator::Add => left + right,
            Operator::Subtract => left - right,
            Operator::Multiply => left * right,
            Operator::Divide => left / right,
        }
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
        assert_eq!(result.unwrap_err(), RuntimeError::TypeMismatch);
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
        let result: RuntimeResult = interpreter.expression(expression);

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
}

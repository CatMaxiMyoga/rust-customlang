//! Contains the interpreter for the programming language

pub mod types;

use std::mem::discriminant;

use parser::types::{Expression, Literal, Operator, Program, Statement};
use types::Value;

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
    pub fn run(program: Program, environment: &'a mut Environment) -> Result<(), String> {
        let mut interpreter: Self = Self { environment };
        for statement in program.statements {
            interpreter.statement(statement)?;
        }

        Ok(())
    }

    #[cfg(test)]
    pub(crate) const fn new(environment: &'a mut Environment) -> Self {
        Self { environment }
    }

    fn statement(&mut self, statement: Statement) -> Result<(), String> {
        match statement {
            Statement::Expression(expr) => {
                // TEMP: prints the result of expression statements
                let expression_result: Value = self.expression(expr)?;
                println!("{expression_result:?}");
            }
            Statement::VariableDeclaration { name, value } => {
                let value: Option<Value> = if let Some(expr) = value {
                    Some(self.expression(expr)?)
                } else {
                    None
                };
                self.environment.insert(name, value);
            }
            Statement::VariableAssignment { name, value } => {
                let old: Option<Value> = if let Some(val) = self.environment.get(&name) {
                    val.clone()
                } else {
                    return Err(format!("Variable '{name}' not declared"));
                };

                let value: Value = self.expression(value)?;

                if let Some(old) = old
                    && discriminant(&old) != discriminant(&value)
                {
                    return Err(format!("Type mismatch in assignment to variable '{name}'"));
                }

                self.environment.insert(name, Some(value));
            }
        }

        Ok(())
    }

    pub(crate) fn expression(&mut self, expression: Expression) -> Result<Value, String> {
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
                        || Err(String::from("Variable '{identifier}' is uninitialized")),
                        |value| Ok(value.clone()),
                    )
                } else {
                    Err(format!("Variable '{identifier}' not declared"))
                }
            }
        }
    }

    const fn literal_expression(literal: &Literal) -> Value {
        match literal {
            Literal::Integer(value) => Value::Integer(*value),
            Literal::Float(value) => Value::Float(*value),
        }
    }

    fn binary_expression(
        &mut self,
        left: Expression,
        operator: &Operator,
        right: Expression,
    ) -> Result<Value, String> {
        let left: Value = self.expression(left)?;
        let right: Value = self.expression(right)?;

        Ok(match operator {
            Operator::Add => left + right,
            Operator::Subtract => left - right,
            Operator::Multiply => left * right,
            Operator::Divide => left / right,
        })
    }
}

#[cfg(test)]
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
                    let result: Value = interpreter.expression(expression).unwrap();
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
        Value::Integer(7),
        Value::Integer(3),
        Value::Integer(10),
        Value::Integer(2)
    );

    test_all_ops!(
        float,
        Expression::Literal(Literal::Float(5.0)),
        Expression::Literal(Literal::Float(2.0)),
        Value::Float(7.0),
        Value::Float(3.0),
        Value::Float(10.0),
        Value::Float(2.5)
    );

    test_all_ops!(
        mixed,
        Expression::Literal(Literal::Integer(5)),
        Expression::Literal(Literal::Float(2.0)),
        Value::Float(7.0),
        Value::Float(3.0),
        Value::Float(10.0),
        Value::Float(2.5)
    );

    test_all_ops!(
        mixed_reverse,
        Expression::Literal(Literal::Float(5.0)),
        Expression::Literal(Literal::Integer(2)),
        Value::Float(7.0),
        Value::Float(3.0),
        Value::Float(10.0),
        Value::Float(2.5)
    );
}

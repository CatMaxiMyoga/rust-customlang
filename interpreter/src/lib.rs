//! Contains the interpreter for the programming language

pub mod types;

use parser::types::{Expression, Literal, Operator, Program, Statement};
use types::Value;

/// The interpreter for the programming language.
pub struct Interpreter;

impl Interpreter {
    /// Interprets the AST and executes the program.
    pub fn run(program: Program) {
        for statement in program.statements {
            Self::statement(statement);
        }
    }

    fn statement(statement: Statement) {
        match statement {
            Statement::Expression(expr) => {
                // TEMP: prints the result of expression statements
                let expression_result: Value = Self::expression(expr);
                println!("{expression_result:?}");
            }
        }
    }

    pub(crate) fn expression(expression: Expression) -> Value {
        match expression {
            Expression::Literal(literal) => Self::literal_expression(&literal),
            Expression::Binary {
                left,
                operator,
                right,
            } => Self::binary_expression(*left, &operator, *right),
        }
    }

    const fn literal_expression(literal: &Literal) -> Value {
        match literal {
            Literal::Integer(value) => Value::Integer(*value),
            Literal::Float(value) => Value::Float(*value),
        }
    }

    fn binary_expression(left: Expression, operator: &Operator, right: Expression) -> Value {
        let left: Value = Self::expression(left);
        let right: Value = Self::expression(right);

        match operator {
            Operator::Add => left + right,
            Operator::Subtract => left - right,
            Operator::Multiply => left * right,
            Operator::Divide => left / right,
        }
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
                    let expression: Expression = Expression::Binary{
                        left: Box::new($left),
                        operator: Operator::$op,
                        right: Box::new($right)
                    };
                    let result: Value = Interpreter::expression(expression);
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

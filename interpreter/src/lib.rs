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

    fn expression(expression: Expression) -> Value {
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
mod tests {}

//! The semantic analysis crate for the custom language's AST.

use parser::types::{BinaryOperator, Expr, Expression, Literal, Statement, Stmt};

use crate::{
    errors::SemanticErrorType,
    types::{ExpressionReturn, LValue, Scope, StatementReturn, Type},
};

pub mod errors;
pub mod types;

/// Analyzes the AST for semantic correctness, such as type checking and scope resolution (later on)
pub struct SemanticAnalyzer {
    scope: Scope,
}

impl SemanticAnalyzer {
    /// Analyzes the given AST for semantic correctness.
    ///
    /// # Parameters
    /// - `ast`: The abstract syntax tree to analyze.
    ///
    /// # Errors
    /// TODO: Add errors later
    pub fn analyze(ast: parser::types::Program) -> StatementReturn {
        let mut analyzer: Self = Self {
            scope: Scope::new(None),
        };

        for statement in ast.statements {
            analyzer.statement(statement)?;
        }

        Ok(())
    }

    fn statement(&mut self, stmt: Stmt) -> StatementReturn {
        match stmt.node {
            Statement::VariableDeclaration { type_, name, value } => {
                self.variable_declaration(type_, &name, value)
            }
            Statement::Assignment { assignee, value } => self.assignment(*assignee, value),
            _ => todo!(),
        }
    }

    fn variable_declaration(
        &mut self,
        var_type: String,
        name: &str,
        value: Option<Expr>,
    ) -> StatementReturn {
        let var_type: Type = Type::from(var_type);
        self.scope.add_variable(name.to_string(), var_type)?;

        if let Some(value) = value {
            let value_type: Type = self.expression(value)?;
            self.scope.assign_variable(name, &value_type)?;
        }

        Ok(())
    }

    // TODO: Remove temporary allow attributes once implemented.
    #[allow(clippy::needless_pass_by_ref_mut)]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(unused_variables)]
    fn assignment(&mut self, assignee: Expr, value: Expr) -> StatementReturn {
        let lvalue: LValue = self.resolve_lvalue(assignee)?;
        let value_type: Type = self.expression(value)?;
        todo!()
    }

    // TODO: Remove temporary allow attributes once implemented.
    #[allow(clippy::needless_pass_by_ref_mut)]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(unused_variables)]
    fn resolve_lvalue(&self, expr: Expr) -> Result<LValue, SemanticErrorType> {
        todo!()
    }

    // TODO: Remove temporary allow attributes once implemented.
    #[allow(clippy::needless_pass_by_ref_mut)]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(unused_variables)]
    fn expression(&mut self, expr: Expr) -> ExpressionReturn {
        match expr.node {
            Expression::Literal(literal) => self.literal(literal),
            Expression::Binary {
                left,
                operator,
                right,
            } => self.binary(*left, operator, *right),
            _ => todo!(),
        }
    }

    // TODO: Remove temporary allow attributes once implemented.
    #[allow(clippy::needless_pass_by_ref_mut)]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(unused_variables)]
    fn literal(&self, literal: Literal) -> ExpressionReturn {
        todo!()
    }

    // TODO: Remove temporary allow attributes once implemented.
    #[allow(clippy::needless_pass_by_ref_mut)]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(unused_variables)]
    fn binary(&self, left: Expr, operator: BinaryOperator, right: Expr) -> ExpressionReturn {
        todo!()
    }
}

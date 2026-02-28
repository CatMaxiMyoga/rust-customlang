//! The semantic analysis crate for the custom language's AST.

use parser::types::{BinaryOperator, Expr, Expression, Literal, Span, Statement, Stmt};

use crate::{
    errors::SemanticError,
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
        let loc: Span = stmt.span;
        let loc: (usize, usize) = (loc.start.0, loc.start.1);
        match stmt.node {
            Statement::VariableDeclaration { type_, name, value } => {
                self.variable_declaration(type_, &name, value, loc)
            }
            Statement::Assignment { assignee, value } => self.assignment(*assignee, value, loc),
            _ => todo!(),
        }
    }

    fn variable_declaration(
        &mut self,
        var_type: String,
        name: &str,
        value: Option<Expr>,
        loc: (usize, usize),
    ) -> StatementReturn {
        let var_type: Type = Type::from(var_type);
        self.scope.add_variable(name.to_string(), var_type, loc)?;

        if let Some(value) = value {
            let value_type: Type = self.expression(value, loc)?;
            self.scope.assign_variable(name, &value_type, loc)?;
        }

        Ok(())
    }

    // TODO: Remove temporary allow attributes once implemented.
    #[allow(clippy::needless_pass_by_ref_mut)]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(unused_variables)]
    fn assignment(&mut self, assignee: Expr, value: Expr, loc: (usize, usize)) -> StatementReturn {
        let lvalue: LValue = self.resolve_lvalue(assignee, loc)?;
        let value_type: Type = self.expression(value, loc)?;
        todo!()
    }

    // TODO: Remove temporary allow attributes once implemented.
    #[allow(clippy::needless_pass_by_ref_mut)]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(unused_variables)]
    fn resolve_lvalue(&self, expr: Expr, loc: (usize, usize)) -> Result<LValue, SemanticError> {
        todo!()
    }

    // TODO: Remove temporary allow attributes once implemented.
    #[allow(clippy::needless_pass_by_ref_mut)]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(unused_variables)]
    fn expression(&mut self, expr: Expr, loc: (usize, usize)) -> ExpressionReturn {
        match expr.node {
            Expression::Literal(literal) => self.literal(literal, loc),
            Expression::Binary {
                left,
                operator,
                right,
            } => self.binary(*left, operator, *right, loc),
            _ => todo!(),
        }
    }

    // TODO: Remove temporary allow attributes once implemented.
    #[allow(clippy::needless_pass_by_ref_mut)]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(unused_variables)]
    fn literal(&self, literal: Literal, loc: (usize, usize)) -> ExpressionReturn {
        todo!()
    }

    // TODO: Remove temporary allow attributes once implemented.
    #[allow(clippy::needless_pass_by_ref_mut)]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(unused_variables)]
    fn binary(
        &self,
        left: Expr,
        operator: BinaryOperator,
        right: Expr,
        loc: (usize, usize),
    ) -> ExpressionReturn {
        todo!()
    }
}

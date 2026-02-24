//! The semantic analysis crate for the custom language's AST.

use parser::types::{Expr, Statement, Stmt};

use crate::types::{ExpressionReturn, Scope, StatementReturn, Type};

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
                let var_type: Type = Type::from(type_);
                self.scope.add_variable(name.clone(), var_type)?;

                if let Some(value) = value {
                    let value_type: Type = self.expression(value)?;
                    self.scope.assign_variable(&name, &value_type)?;
                }
            }
            Statement::Assignment { assignee, value } => {
                let _value_type: Type = self.expression(value)?;
                let _assignee_type: Type = self.expression(*assignee)?;
                // TODO: Somehow assign to correct scope...?
                todo!()
            }
            _ => todo!(),
        }

        Ok(())
    }

    // TODO: Remove temporary allow attributes once implemented.
    #[allow(clippy::needless_pass_by_ref_mut)]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(unused_variables)]
    fn expression(&mut self, expr: Expr) -> ExpressionReturn {
        // TODO: Implement
        todo!()
    }
}

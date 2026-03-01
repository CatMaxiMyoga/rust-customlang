//! The semantic analysis crate for the custom language's AST.

use std::collections::HashMap;

use parser::types::{Expr, Expression, Span, Statement, Stmt};

use crate::{
    errors::{SemanticError, SemanticErrorType},
    types::{
        Class, ExpressionReturn, Field, FieldDeclarationInfo, Function, LValue,
        MethodDeclarationInfo, Scope, StatementReturn, Type,
    },
};

pub mod errors;
pub mod types;

/// Analyzes the AST for semantic correctness, such as type checking and scope resolution (later on)
pub struct SemanticAnalyzer {
    function_return: Option<Type>,
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
            function_return: None,
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
                self.variable_declaration(&type_, &name, value, loc)
            }
            Statement::Assignment { assignee, value } => self.assignment(*assignee, value, loc),
            Statement::FunctionDeclaration {
                return_type,
                name,
                parameters,
                body,
            } => self.function_declaration(&return_type, &name, parameters, body, loc),
            Statement::ClassDeclaration { name, body } => self.class_declaration(&name, body, loc),
            Statement::FieldDeclaration { .. } | Statement::MethodDeclaration { .. } => {
                unreachable!(
                    "Field and Method declarations outside class declarations should be impossible."
                )
            }
            // TODO: Add missing statements
            _ => todo!(),
        }
    }

    fn variable_declaration(
        &mut self,
        var_type: &str,
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

    fn assignment(&mut self, assignee: Expr, value: Expr, loc: (usize, usize)) -> StatementReturn {
        let lvalue: LValue = self.resolve_lvalue(assignee, loc)?;
        let value_type: Type = self.expression(value, loc)?;

        match lvalue {
            LValue::Variable(name) => self.scope.assign_variable(&name, &value_type, loc),
            LValue::Field { base, field_name } => {
                let class: Class = self.scope.get_class(&(String::from(&base)), loc)?;
                self.scope
                    .assign_field(&class.name, &field_name, &value_type, loc)?;

                Ok(())
            }
            LValue::StaticField { class, field_name } => {
                let class: Class = self.scope.get_class(&(String::from(&class)), loc)?;
                self.scope
                    .assign_field(&class.name, &field_name, &value_type, loc)?;

                Ok(())
            }
        }
    }

    fn resolve_lvalue(&self, expr: Expr, loc: (usize, usize)) -> Result<LValue, SemanticError> {
        match expr.node {
            Expression::Identifier(name) => {
                self.scope.get_local_variable(&name, loc)?;
                Ok(LValue::Variable(name))
            }
            Expression::MemberAccess { object, member } => {
                let expr_type: Type = self.expression(object.as_ref().clone(), loc)?;
                let field: Field =
                    self.scope
                        .get_class_field(&String::from(&expr_type), &member, loc)?;

                if field.is_static {
                    Ok(LValue::StaticField {
                        class: expr_type,
                        field_name: member,
                    })
                } else if matches!(object.node, Expression::Identifier(_)) {
                    Ok(LValue::Field {
                        base: expr_type,
                        field_name: member,
                    })
                } else {
                    Err(SemanticError {
                        error_type: SemanticErrorType::IllegalInstanceFieldAssignment(member),
                        line: loc.0,
                        column: loc.1,
                    })
                }
            }
            e => Err(SemanticError {
                error_type: SemanticErrorType::InvalidAssignmentTarget(e.name().to_string()),
                line: loc.0,
                column: loc.1,
            }),
        }
    }

    fn function_declaration(
        &mut self,
        return_type: &str,
        name: &str,
        parameters: Vec<(String, String)>,
        body: Vec<Stmt>,
        loc: (usize, usize),
    ) -> StatementReturn {
        if self.function_return.is_some() {
            unreachable!("Nested functions are illegal and should have been caught by the parser");
        }

        let return_type: Type = Type::from(return_type);

        let mut function_analyzer: Self = Self {
            scope: Scope::new(Some(Box::new(self.scope.clone()))),
            function_return: Some(return_type.clone()),
        };

        let mut param_types: Vec<Type> = Vec::new();

        for (param_type, param_name) in parameters {
            let param_type: Type = Type::from(&param_type);
            function_analyzer
                .scope
                .add_variable(param_name.clone(), param_type.clone(), loc)?;
            function_analyzer
                .scope
                .assign_variable(&param_name, &param_type, loc)?;
            param_types.push(param_type);
        }

        for statement in body {
            function_analyzer.statement(statement)?;
        }

        self.scope.add_function(
            name.to_string(),
            Function {
                parameters: param_types,
                return_type,
                is_static: false,
            },
            loc,
        )?;

        Ok(())
    }

    fn class_declaration(
        &mut self,
        name: &str,
        body: Vec<Stmt>,
        loc: (usize, usize),
    ) -> StatementReturn {
        let mut fields: HashMap<String, Field> = HashMap::new();
        let mut methods: HashMap<String, Function> = HashMap::new();

        for statement in body {
            match statement.node {
                Statement::FieldDeclaration {
                    type_,
                    name,
                    static_,
                    value,
                } => self.field_declaration(
                    &mut fields,
                    &methods,
                    FieldDeclarationInfo {
                        field_type: type_,
                        name,
                        static_,
                        value,
                    },
                    loc,
                )?,
                Statement::MethodDeclaration {
                    return_type,
                    name,
                    parameters,
                    body,
                    static_,
                } => self.method_declaration(
                    &mut methods,
                    &fields,
                    MethodDeclarationInfo {
                        return_type,
                        name,
                        parameters,
                        body,
                        static_,
                    },
                    loc,
                )?,
                _ => unreachable!(
                    "The parser should only allow field and method declarations in calsses."
                ),
            }
        }

        self.scope.add_class(
            Class {
                name: name.to_owned(),
                fields,
                methods,
            },
            loc,
        )?;

        Ok(())
    }

    fn field_declaration(
        &self,
        fields: &mut HashMap<String, Field>,
        methods: &HashMap<String, Function>,
        field_info: FieldDeclarationInfo,
        loc: (usize, usize),
    ) -> Result<(), SemanticError> {
        if fields.contains_key(&field_info.name) {
            return Err(SemanticError {
                error_type: SemanticErrorType::DuplicateField(field_info.name),
                line: loc.0,
                column: loc.1,
            });
        } else if methods.contains_key(&field_info.name) {
            return Err(SemanticError {
                error_type: SemanticErrorType::FieldMethodNameConflict(field_info.name),
                line: loc.0,
                column: loc.1,
            });
        }

        let field_type: Type = Type::from(&field_info.field_type);

        if let Some(value) = field_info.value {
            let value_type: Type = self.expression(value, loc)?;

            if field_type != value_type {
                return Err(SemanticError {
                    error_type: SemanticErrorType::FieldInitializationTypeMismatch {
                        expected: (&field_type).into(),
                        found: (&value_type).into(),
                    },
                    line: loc.0,
                    column: loc.1,
                });
            }
        }

        fields.insert(
            field_info.name,
            Field {
                field_type,
                is_static: field_info.static_,
            },
        );

        Ok(())
    }

    // TODO: Remove temporary allow attributes once implemented.
    #[allow(clippy::needless_pass_by_ref_mut)]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(clippy::unused_self)]
    #[allow(unused_variables)]
    fn method_declaration(
        &mut self,
        methods: &mut HashMap<String, Function>,
        fields: &HashMap<String, Field>,
        method_info: MethodDeclarationInfo,
        loc: (usize, usize),
    ) -> Result<(), SemanticError> {
        // TODO: Implement next
        todo!()
    }

    // TODO: Remove temporary allow attributes once implemented.
    #[allow(clippy::needless_pass_by_value)]
    #[allow(clippy::unused_self)]
    #[allow(unused_variables)]
    fn expression(&self, expr: Expr, loc: (usize, usize)) -> ExpressionReturn {
        todo!()
    }
}

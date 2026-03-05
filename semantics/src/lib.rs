//! The semantic analysis crate for the custom language's AST.

use std::collections::HashMap;

use parser::types::{
    BinaryOperator, Expr, Expression, Literal, Span, Statement, Stmt, UnaryOperator,
};

use crate::{
    errors::{SemanticError, SemanticErrorType},
    types::{
        Class, ExpressionReturn, Field, FieldDeclarationInfo, Function, LValue,
        MethodDeclarationInfo, Scope, StatementReturn, Type,
    },
};

pub mod builtin_types;
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
            analyzer.statement(statement, true)?;
        }

        Ok(())
    }

    fn statement(&mut self, stmt: Stmt, allows_definitions: bool) -> StatementReturn {
        let loc: (usize, usize) = Self::get_loc(&stmt.span);
        match stmt.node {
            Statement::VariableDeclaration { type_, name, value } => {
                self.variable_declaration(&type_, &name, value, loc)
            }
            Statement::Assignment { assignee, value } => self.assignment(*assignee, value),
            Statement::FunctionDeclaration {
                return_type,
                name,
                parameters,
                body,
            } => self.function_declaration(
                &return_type,
                &name,
                parameters,
                body,
                allows_definitions,
                loc,
            ),
            Statement::ClassDeclaration { name, body } => {
                self.class_declaration(&name, body, allows_definitions, loc)
            }
            Statement::FieldDeclaration { .. } | Statement::MethodDeclaration { .. } => {
                unreachable!(
                    "Field and Method declarations outside class declarations should be impossible."
                )
            }
            Statement::If {
                conditional_branches,
                else_branch,
            } => self.if_statement(conditional_branches, else_branch, loc),
            Statement::While { condition, body } => self.while_statement(condition, body, loc),
            Statement::Return(expr) => self.return_statement(expr, loc),
            Statement::Expression(expr) => self.expression(expr).map(|_| ()),
        }
    }

    #[must_use]
    const fn get_loc(span: &Span) -> (usize, usize) {
        (span.start.0, span.start.1)
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
            let value_type: Type = self.expression(value)?;
            self.scope.assign_variable(name, &value_type, loc)?;
        }

        Ok(())
    }

    fn assignment(&mut self, assignee: Expr, value: Expr) -> StatementReturn {
        let aloc: (usize, usize) = Self::get_loc(&assignee.span);

        let lvalue: LValue = self.resolve_lvalue(assignee)?;
        let value_type: Type = self.expression(value)?;

        match lvalue {
            LValue::Variable(name) => self.scope.assign_variable(&name, &value_type, aloc),
            LValue::Field { base, field_name } => {
                let class: Class = self.scope.get_class(&(String::from(&base)), aloc)?;
                self.scope
                    .assign_field(&class.name, &field_name, &value_type, aloc)?;

                Ok(())
            }
            LValue::StaticField { class, field_name } => {
                let class: Class = self.scope.get_class(&(String::from(&class)), aloc)?;
                self.scope
                    .assign_field(&class.name, &field_name, &value_type, aloc)?;

                Ok(())
            }
        }
    }

    fn resolve_lvalue(&self, expr: Expr) -> Result<LValue, SemanticError> {
        let loc: (usize, usize) = Self::get_loc(&expr.span);

        match expr.node {
            Expression::Identifier(name) => {
                self.scope.get_local_variable(&name, loc)?;
                Ok(LValue::Variable(name))
            }
            Expression::MemberAccess { object, member } => {
                let expr_type: Type = self.expression(object.as_ref().clone())?;
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
        allowed: bool,
        loc: (usize, usize),
    ) -> StatementReturn {
        if !allowed {
            return Err(SemanticError {
                error_type: SemanticErrorType::IllegalFunctionDeclaration(name.to_string()),
                line: loc.0,
                column: loc.1,
            });
        }

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
            function_analyzer.statement(statement, false)?;
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
        allowed: bool,
        loc: (usize, usize),
    ) -> StatementReturn {
        if !allowed {
            return Err(SemanticError {
                error_type: SemanticErrorType::IllegalClassDeclaration(name.to_string()),
                line: loc.0,
                column: loc.1,
            });
        }

        let mut fields: HashMap<String, Field> = HashMap::new();
        let mut methods: HashMap<String, Vec<Function>> = HashMap::new();

        for statement in body {
            let loc: (usize, usize) = Self::get_loc(&statement.span);

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
                        class_name: name.to_string(),
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
        methods: &HashMap<String, Vec<Function>>,
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
            let value_type: Type = self.expression(value)?;

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

    fn method_declaration(
        &self,
        methods: &mut HashMap<String, Vec<Function>>,
        fields: &HashMap<String, Field>,
        mut method_info: MethodDeclarationInfo,
        loc: (usize, usize),
    ) -> Result<(), SemanticError> {
        if fields.contains_key(&method_info.name) {
            return Err(SemanticError {
                error_type: SemanticErrorType::MethodFieldNameConflict(method_info.name),
                line: loc.0,
                column: loc.1,
            });
        }

        if self.function_return.is_some() {
            unreachable!("Nested methods are illegal and should have been caught by the parser");
        }

        if method_info.name == "new" {
            return Err(SemanticError {
                error_type: SemanticErrorType::IllegalMethodName(method_info.name),
                line: loc.0,
                column: loc.1,
            });
        }

        let return_type: Type = if method_info.return_type.is_empty() {
            method_info.name = "new".into();
            Type::from(&method_info.class_name)
        } else {
            Type::from(&method_info.return_type)
        };

        let mut method_analyzer: Self = Self {
            scope: Scope::new(Some(Box::new(self.scope.clone()))),
            function_return: Some(return_type.clone()),
        };

        if !method_info.static_ {
            method_analyzer.scope.add_variable(
                "self".to_string(),
                Type::Class(method_info.class_name.clone()),
                loc,
            )?;
            method_analyzer.scope.assign_variable(
                "self",
                &Type::Class(method_info.class_name.clone()),
                loc,
            )?;
        }

        let mut param_types: Vec<Type> = Vec::new();

        for (param_type, param_name) in method_info.parameters {
            let param_type: Type = Type::from(&param_type);
            method_analyzer
                .scope
                .add_variable(param_name.clone(), param_type.clone(), loc)?;
            method_analyzer
                .scope
                .assign_variable(&param_name, &param_type, loc)?;
            param_types.push(param_type);
        }

        for statement in method_info.body {
            method_analyzer.statement(statement, false)?;
        }

        let method: Function = Function {
            parameters: param_types,
            return_type,
            is_static: false,
        };

        // FIXME: fix this warning later
        #[allow(clippy::map_entry)]
        if methods.contains_key(&method_info.name) {
            for m in &methods[&method_info.name] {
                if m.parameters == method.parameters {
                    return Err(SemanticError {
                        error_type: SemanticErrorType::DuplicateMethod(method_info.name),
                        line: loc.0,
                        column: loc.1,
                    });
                }
            }

            methods
                .get_mut(&method_info.name)
                .expect("Checked before")
                .push(method);
        } else {
            methods.insert(method_info.name, vec![method]);
        }

        Ok(())
    }

    fn if_statement(
        &mut self,
        conditional_branches: Vec<(Expr, Vec<Stmt>)>,
        else_branch: Option<Vec<Stmt>>,
        loc: (usize, usize),
    ) -> StatementReturn {
        for (condition, body) in conditional_branches {
            let condition_type: Type = self.expression(condition)?;

            if condition_type != Type::Boolean {
                return Err(SemanticError {
                    error_type: SemanticErrorType::NonBooleanCondition((&condition_type).into()),
                    line: loc.0,
                    column: loc.1,
                });
            }

            for statement in body {
                self.statement(statement, false)?;
            }
        }

        if let Some(else_body) = else_branch {
            for statement in else_body {
                self.statement(statement, false)?;
            }
        }

        Ok(())
    }

    fn while_statement(
        &mut self,
        condition: Expr,
        body: Vec<Stmt>,
        loc: (usize, usize),
    ) -> StatementReturn {
        let condition_type: Type = self.expression(condition)?;

        if condition_type != Type::Boolean {
            return Err(SemanticError {
                error_type: SemanticErrorType::NonBooleanCondition((&condition_type).into()),
                line: loc.0,
                column: loc.1,
            });
        }

        for statement in body {
            self.statement(statement, false)?;
        }

        Ok(())
    }

    fn return_statement(&self, expr: Option<Expr>, loc: (usize, usize)) -> StatementReturn {
        let function_return: Type = match &self.function_return {
            Some(ret) => ret.clone(),
            None => {
                return Err(SemanticError {
                    error_type: SemanticErrorType::IllegalReturn,
                    line: loc.0,
                    column: loc.1,
                });
            }
        };

        let has_expr: bool = expr.is_some();
        let void_return: bool = function_return == Type::Void;

        if has_expr == void_return {
            Err(SemanticError {
                error_type: SemanticErrorType::ReturnTypeMismatch {
                    expected: (&function_return).into(),
                    found: (&self.expression(expr.expect("Checked before"))?).into(),
                },
                line: loc.0,
                column: loc.1,
            })
        } else if has_expr {
            let expr: Expr = expr.expect("Checked before");
            let expr_type: Type = self.expression(expr)?;

            if expr_type == function_return {
                Ok(())
            } else {
                Err(SemanticError {
                    error_type: SemanticErrorType::ReturnTypeMismatch {
                        expected: (&function_return).into(),
                        found: (&expr_type).into(),
                    },
                    line: loc.0,
                    column: loc.1,
                })
            }
        } else {
            Ok(())
        }
    }

    // TODO: Remove temporary allow attributes once implemented.
    #[allow(clippy::needless_pass_by_value)]
    #[allow(clippy::unused_self)]
    #[allow(unused_variables)]
    fn expression(&self, expr: Expr) -> ExpressionReturn {
        let loc: (usize, usize) = Self::get_loc(&expr.span);

        match expr.node {
            Expression::Literal(literal) => Ok(Self::literal(&literal)),
            Expression::Identifier(identifier) => self.scope.get_variable(&identifier, loc),
            Expression::Binary {
                left,
                operator,
                right,
            } => self.binary(*left, &operator, *right),
            Expression::Unary { operator, operand } => self.unary(&operator, *operand),
            Expression::Call { callee, arguments } => self.call(*callee, arguments),
            _ => todo!(),
        }
    }

    const fn literal(literal: &Literal) -> Type {
        match literal {
            Literal::Integer(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::Boolean(_) => Type::Boolean,
            Literal::String(_) => Type::String,
        }
    }

    fn binary(&self, left: Expr, operator: &BinaryOperator, right: Expr) -> ExpressionReturn {
        let lloc: (usize, usize) = Self::get_loc(&left.span);
        let rloc: (usize, usize) = Self::get_loc(&right.span);

        let ltype: Type = self.expression(left)?;
        let rtype: Type = self.expression(right)?;

        let op_name: &'static str = match operator {
            BinaryOperator::Add => "Add",
            BinaryOperator::Subtract => "Sub",
            BinaryOperator::Multiply => "Mul",
            BinaryOperator::Divide => "Div",
            BinaryOperator::Equals => "Eq",
            BinaryOperator::NotEquals => "Ne",
            BinaryOperator::LessThan => "Lt",
            BinaryOperator::GreaterThan => "Gt",
            BinaryOperator::LessThanOrEqual => "Le",
            BinaryOperator::GreaterThanOrEqual => "Ge",
            BinaryOperator::And => "And",
            BinaryOperator::Or => "Or",
        };

        let lhs_func_name: String = format!("_bop{op_name}");
        let rhs_func_name: String = format!("_bopR{op_name}");

        let lclass: Class = self.scope.get_class(&String::from(&ltype), lloc)?;
        let rclass: Class = self.scope.get_class(&String::from(&rtype), rloc)?;

        if let Ok(method) = rclass.get_method(&rhs_func_name, &[ltype], rloc) {
            Ok(method.return_type.clone())
        } else {
            Ok(lclass
                .get_method(&lhs_func_name, &[rtype], lloc)?
                .return_type
                .clone())
        }
    }

    fn unary(&self, operator: &UnaryOperator, operand: Expr) -> ExpressionReturn {
        let loc: (usize, usize) = Self::get_loc(&operand.span);

        let op_type: Type = self.expression(operand)?;

        let func_name: String = format!(
            "_uop{}",
            match operator {
                UnaryOperator::Not => "Not",
            }
        );

        let op_class: Class = self.scope.get_class(&String::from(&op_type), loc)?;

        Ok(op_class
            .get_method(&func_name, &[], loc)?
            .return_type
            .clone())
    }

    fn call(&self, callee: Expr, arguments: Vec<Expr>) -> ExpressionReturn {
        let arguments: Vec<Type> = arguments
            .into_iter()
            .map(|arg| self.expression(arg))
            .collect::<Result<_, _>>()?;

        let loc: (usize, usize) = Self::get_loc(&callee.span);

        Ok(match callee.node {
            Expression::Identifier(name) => self.scope.get_function(&name, loc),
            Expression::MemberAccess { object, member } => {
                let object_type: Type = self.expression(object.as_ref().clone())?;
                let class: Class = self.scope.get_class(&String::from(&object_type), loc)?;
                class.get_method(&member, &arguments, loc).cloned()
            }
            _ => unreachable!("Parser only allows identifiers and member accesses as callees."),
        }?
        .return_type)
    }
}

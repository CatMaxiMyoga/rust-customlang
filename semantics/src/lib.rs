//! The semantic analysis crate for the custom language's AST.

use std::collections::{HashMap, hash_map::Entry};

use parser::types::{
    BinaryOperator, Expr, Expression, Literal, Span, Statement, Stmt, UnaryOperator,
};

use crate::{
    errors::{SemanticError, SemanticErrorType},
    types::{
        Class, ExpressionReturn, Field, FieldDeclarationInfo, Function, LValue,
        MethodDeclarationBodyInfo, MethodDeclarationSignatureInfo,
        MethodDeclarationSignatureReturn, Scope, StatementReturn, Type,
    },
};

pub mod builtins;
pub mod errors;
pub mod types;

/// Analyzes the AST for semantic correctness, such as type checking and scope resolution (later on)
pub struct SemanticAnalyzer {
    function_return: Option<Type>,
    found_return: bool,
    class: Option<Type>,
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
            found_return: false,
            class: None,
        };

        for class in builtins::get_builtin_types() {
            analyzer.scope.add_class(class, (0, 0))?;
        }

        for statement in ast.statements {
            analyzer.statement(statement, true)?;
        }

        let main: Class = analyzer
            .scope
            .get_class("Main", (0, 0))
            .map_err(|_| SemanticError {
                error_type: SemanticErrorType::EntryPointMissing,
                line: 0,
                column: 0,
            })?;
        let main_method: Function = main
            .get_method("main", &[], (0, 0))
            .map_err(|_| SemanticError {
                error_type: SemanticErrorType::EntryPointMissing,
                line: 0,
                column: 0,
            })?
            .clone();

        if main_method.return_type == Type::Int {
            if main_method.is_static {
                Ok(())
            } else {
                Err(SemanticError {
                    error_type: SemanticErrorType::EntryPointMustBeStatic,
                    line: 0,
                    column: 0,
                })
            }
        } else {
            Err(SemanticError {
                error_type: SemanticErrorType::EntryPointReturnTypeMismatch(
                    (&main_method.return_type).into(),
                ),
                line: 0,
                column: 0,
            })
        }
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

        if var_type == Type::Void {
            return Err(SemanticError {
                error_type: SemanticErrorType::IllegalVoidVariable(name.to_string()),
                line: loc.0,
                column: loc.1,
            });
        }

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
                } else if matches!(object.node, Expression::Identifier(_) | Expression::Self_) {
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
            found_return: false,
            class: None,
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

        self.scope.add_function(
            name.to_string(),
            Function {
                parameters: param_types,
                return_type: return_type.clone(),
                is_static: false,
            },
            loc,
        )?;

        for statement in body {
            function_analyzer.statement(statement, false)?;
        }

        if return_type != Type::Void && !function_analyzer.found_return {
            return Err(SemanticError {
                error_type: SemanticErrorType::MissingReturn,
                line: loc.0,
                column: loc.1,
            });
        }

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
        let mut body_info: Vec<MethodDeclarationBodyInfo> = Vec::new();

        let mut found_method: bool = false;

        let class_name: String = name.to_string();

        self.class = Some(Type::Class(name.to_string()));
        for statement in body {
            let loc: (usize, usize) = Self::get_loc(&statement.span);

            match statement.node {
                Statement::FieldDeclaration {
                    type_,
                    name,
                    static_,
                    value,
                } => {
                    if found_method {
                        return Err(SemanticError {
                            error_type: SemanticErrorType::FieldAfterMethod(name),
                            line: loc.0,
                            column: loc.1,
                        });
                    }

                    self.field_declaration(
                        &mut fields,
                        &methods,
                        FieldDeclarationInfo {
                            field_type: type_,
                            name,
                            static_,
                            value,
                        },
                        loc,
                    )?;
                }
                Statement::MethodDeclaration {
                    return_type,
                    name,
                    parameters,
                    body,
                    static_,
                } => {
                    found_method = true;
                    let ret: MethodDeclarationSignatureReturn = self.method_signature(
                        &mut methods,
                        &fields,
                        MethodDeclarationSignatureInfo {
                            class_name: class_name.clone(),
                            return_type: return_type.clone(),
                            name: name.clone(),
                            parameters,
                            static_,
                        },
                        loc,
                    )?;
                    body_info.push(MethodDeclarationBodyInfo {
                        parameters: ret.0,
                        return_type: ret.1,
                        constructor: ret.2,
                        body,
                        loc,
                    });
                }
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

        for info in body_info {
            self.method_body(info)?;
        }
        self.class = None;

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

        if field_type == Type::Void {
            return Err(SemanticError {
                error_type: SemanticErrorType::IllegalVoidField(field_info.name),
                line: loc.0,
                column: loc.1,
            });
        }

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

    fn method_signature(
        &self,
        methods: &mut HashMap<String, Vec<Function>>,
        fields: &HashMap<String, Field>,
        mut method_info: MethodDeclarationSignatureInfo,
        loc: (usize, usize),
    ) -> Result<MethodDeclarationSignatureReturn, SemanticError> {
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

        let mut constructor: bool = false;

        let return_type: Type = if method_info.return_type.is_empty() {
            method_info.name = "new".into();
            constructor = true;
            Type::from(&method_info.class_name)
        } else {
            Type::from(&method_info.return_type)
        };

        let mut params: Vec<(Type, String)> = Vec::new();
        let mut param_types: Vec<Type> = Vec::new();

        if !method_info.static_ {
            params.push((
                Type::Class(method_info.class_name.clone()),
                "self".to_string(),
            ));
        }

        for (param_type, param_name) in method_info.parameters {
            let param_type: Type = if param_type == "Self" {
                Type::Class(method_info.class_name.clone())
            } else {
                Type::from(&param_type)
            };
            param_types.push(param_type.clone());
            params.push((param_type, param_name));
        }

        let method: Function = Function {
            parameters: param_types,
            return_type: return_type.clone(),
            is_static: method_info.static_,
        };

        match methods.entry(method_info.name.clone()) {
            Entry::Occupied(mut entry) => {
                for m in entry.get() {
                    if m.parameters == method.parameters {
                        return Err(SemanticError {
                            error_type: SemanticErrorType::DuplicateMethod(method_info.name),
                            line: loc.0,
                            column: loc.1,
                        });
                    }
                }

                entry.get_mut().push(method);
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![method]);
            }
        }

        Ok(MethodDeclarationSignatureReturn(
            params,
            return_type,
            constructor,
        ))
    }

    fn method_body(&self, mut method_info: MethodDeclarationBodyInfo) -> StatementReturn {
        let mut method_analyzer: Self = Self {
            scope: Scope::new(Some(Box::new(self.scope.clone()))),
            function_return: Some(if method_info.constructor {
                method_info.return_type = Type::Void;
                Type::Void
            } else {
                method_info.return_type.clone()
            }),
            found_return: false,
            class: self.class.clone(),
        };

        for (ptype, pname) in method_info.parameters {
            method_analyzer
                .scope
                .add_variable(pname.clone(), ptype.clone(), method_info.loc)?;
            method_analyzer
                .scope
                .assign_variable(&pname, &ptype, method_info.loc)?;
        }

        for statement in method_info.body {
            method_analyzer.statement(statement, false)?;
        }

        if method_info.return_type != Type::Void && !method_analyzer.found_return {
            return Err(SemanticError {
                error_type: SemanticErrorType::MissingReturn,
                line: method_info.loc.0,
                column: method_info.loc.1,
            });
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

    fn return_statement(&mut self, expr: Option<Expr>, loc: (usize, usize)) -> StatementReturn {
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
                self.found_return = true;
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
            self.found_return = true;
            Ok(())
        }
    }

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
            Expression::MemberAccess { object, member } => {
                self.member_access(*object, &member, loc)
            }
            Expression::Self_ => self
                .class
                .as_ref()
                .ok_or_else(|| unreachable!("Should be caught by parser"))
                .cloned(),
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
            Expression::Identifier(name) => {
                let func: Function = self.scope.get_function(&name, loc)?;
                if func.parameters == arguments {
                    func.return_type
                } else {
                    return Err(SemanticError {
                        error_type: SemanticErrorType::ArgumentTypeMismatch {
                            func: name,
                            expected: func.parameters.iter().map(Into::into).collect(),
                            found: arguments.iter().map(Into::into).collect(),
                        },
                        line: loc.0,
                        column: loc.1,
                    });
                }
            }
            Expression::MemberAccess { object, member } => {
                let object_type: Type = match &object.node {
                    Expression::Identifier(ident) => {
                        if self.scope.get_class(ident, loc).is_ok() {
                            Type::Class(ident.clone())
                        } else {
                            self.expression(*object)?
                        }
                    }
                    _ => self.expression(object.as_ref().clone())?,
                };
                let class: Class = self.scope.get_class(&String::from(&object_type), loc)?;
                class
                    .get_method(&member, &arguments, loc)?
                    .clone()
                    .return_type
            }
            _ => unreachable!("Parser only allows identifiers and member accesses as callees."),
        })
    }

    fn member_access(&self, object: Expr, member: &str, loc: (usize, usize)) -> ExpressionReturn {
        let object_type: Type = match &object.node {
            Expression::Identifier(ident) => {
                if self.scope.get_class(ident, loc).is_ok() {
                    Type::Class(ident.clone())
                } else {
                    self.expression(object)?
                }
            }
            _ => self.expression(object)?,
        };
        let class: Class = self.scope.get_class(&String::from(&object_type), loc)?;

        // Cannot be method, since method calls are handled in `call` method
        if class.fields.contains_key(member) {
            Ok(class.fields[member].field_type.clone())
        } else {
            Err(SemanticError {
                error_type: SemanticErrorType::FieldNotFound {
                    class: class.name,
                    field: member.to_string(),
                },
                line: loc.0,
                column: loc.1,
            })
        }
    }
}

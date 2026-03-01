//! Contains the types used in the semantic analysis of the language.

use std::collections::HashMap;

use crate::errors::{SemanticError, SemanticErrorType};

/// Represents the result of analyzing a statement, which does not have a type.
pub type StatementReturn = Result<(), SemanticError>;

/// Represents the result of analyzing an expression, which has a type which will be returned.
pub type ExpressionReturn = Result<Type, SemanticError>;

/// Represents expressions which can be used as lvalues in assignments.
pub enum LValue {
    /// Represents a variable, a.k.a. an identifier to a non-function, non-class, locally declared
    /// variable.
    Variable(String),
    /// Represents a field of a class.
    Field {
        /// The type of the base expression.
        base: Type,
        /// The name of the field.
        field_name: String,
    },
    /// Represents a static field of a class.
    StaticField {
        /// The type of the class containing the static field.
        class: Type,
        /// The name of the static field.
        field_name: String,
    },
}

/// Represents a variable's state and type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    /// The variable's type
    pub var_type: Type,
    /// Whether or not the variable has been initialized
    pub initialized: bool,
}

/// Represents a function
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    /// The types of the function's parameters, in order
    pub parameters: Vec<Type>,
    /// The return type of the function
    pub return_type: Type,
    /// Whether or not the function is a static method
    pub is_static: bool,
}

/// Represents a user-defined class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
    /// The name of the class
    pub name: String,
    /// Fields defined in the class, storing type and whether or not they're static
    pub fields: HashMap<String, Field>,
    /// Methods defined in the class
    pub methods: HashMap<String, Function>,
}

/// Represents a field in a class, storing the field's type, whether or not it is static, and
/// whether or not it has been initialized (always true for static fields).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    /// The type of the field
    pub field_type: Type,
    /// Whether or not the field is static
    pub is_static: bool,
}

/// Represents a type in the language
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// Represents an integer, like `42`
    Int,
    /// Represents a floating-point number, like `3.14`
    Float,
    /// Represents a boolean value `true` or `false`
    Boolean,
    /// Represents a string, like `"Hello, world!"`
    String,
    /// Represents the absence of a value, used for functions that don't return anything
    Void,
    /// Represents a user-defined class, like `class MyClass { ... }`
    Class(String),
    /// Represents the current class' type inside the class
    SelfType,
}

impl From<&Type> for String {
    fn from(val: &Type) -> Self {
        match val {
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::Boolean => "bool".to_string(),
            Type::String => "string".to_string(),
            Type::Void => "void".to_string(),
            Type::Class(class_name) => class_name.clone(),
            Type::SelfType => "self".to_string(),
        }
    }
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value {
            "int" => Self::Int,
            "float" => Self::Float,
            "bool" => Self::Boolean,
            "string" => Self::String,
            "void" => Self::Void,
            "Self" => Self::SelfType,
            class_name => Self::Class(class_name.to_string()),
        }
    }
}

impl From<&String> for Type {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

/// Represents a scope containing all variables and functions defined in it as well as the parent
/// scope (if any)
#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    parent: Option<Box<Scope>>,
    variables: HashMap<String, Variable>,
    functions: HashMap<String, Function>,
    classes: HashMap<String, Class>,
}

impl Scope {
    /// Creates a new scope instance.
    ///
    /// # Parameters
    /// - `parent`: An optional parent scope to allow for nested scopes.
    #[must_use]
    pub fn new(parent: Option<Box<Self>>) -> Self {
        Self {
            parent,
            variables: HashMap::new(),
            functions: HashMap::new(),
            classes: HashMap::new(),
        }
    }

    /// Tries to add a new variable to the current scope.
    ///
    /// # Parameters
    /// - `name`: The name of the variable to add.
    /// - `var_type`: The type of the variable to add.
    /// - `loc`: Location in the source code, used for errors.
    ///
    /// # Errors
    /// - `SemanticErrorType::ShadowingFunction`: If a function with the same name already exists in
    ///   the current scope.
    pub fn add_variable(
        &mut self,
        name: String,
        var_type: Type,
        loc: (usize, usize),
    ) -> Result<(), SemanticError> {
        self.check_shadowing(&name, ShadowingCheck::Variable, loc)?;
        self.variables.insert(
            name,
            Variable {
                var_type,
                initialized: false,
            },
        );
        Ok(())
    }

    /// Check if the assigned value's type matches the variable's type and mark the variable as
    /// initialized if not already.
    ///
    /// # Parameters
    /// - `name`: The name of the variable being assigned to.
    /// - `value_type`: The type of the value being assigned to the variable.
    /// - `loc`: Location in the source code, used for errors.
    ///
    /// # Errors
    /// - `SemanticErrorType::TypeMismatch`: If the type of the value being assigned does not match the
    ///   variable's type.
    /// - `SemanticErrorType::VariableNotFound`: If the variable is not found in the current scope or
    ///   any parent scope.
    /// - `SemanticErrorType::VariableUninitialized`: If the variable is found but hasn't been
    ///   initialized yet.
    pub fn assign_variable(
        &mut self,
        name: &str,
        value_type: &Type,
        loc: (usize, usize),
    ) -> Result<(), SemanticError> {
        let var_type: Type = self.get_variable(name, loc)?;

        if var_type == *value_type {
            Ok(())
        } else {
            Err(SemanticError {
                error_type: SemanticErrorType::VariableAssignmentTypeMismatch {
                    expected: (&var_type).into(),
                    found: value_type.into(),
                },
                line: loc.0,
                column: loc.1,
            })
        }
    }

    /// Get the type of a variable by its name, searching through parent scopes if necessary.
    ///
    /// # Parameters
    /// - `name`: The name of the variable to look up.
    /// - `loc`: Location in the source code, used for errors.
    ///
    /// # Errors
    /// - `SemanticErrorType::VariableNotFound`: If the variable is not found in the current scope or
    ///   any parent scope.
    /// - `SemanticErrorType::VariableUninitialized`: If the variable is found but hasn't been
    ///   initialized yet.
    pub fn get_variable(&self, name: &str, loc: (usize, usize)) -> Result<Type, SemanticError> {
        self.variables
            .get(name)
            .map_or_else(
                || {
                    self.parent.as_ref().map_or_else(
                        || Err(SemanticErrorType::VariableNotFound(name.to_string())),
                        |parent_scope| {
                            parent_scope
                                .get_variable(name, loc)
                                .map_err(|e| e.error_type)
                        },
                    )
                },
                |var| {
                    if var.initialized {
                        Err(SemanticErrorType::VariableUninitialized(name.to_string()))
                    } else {
                        Ok(var.var_type.clone())
                    }
                },
            )
            .map_err(|e| SemanticError {
                error_type: e,
                line: loc.0,
                column: loc.1,
            })
    }

    /// Get the type of a variable in the current scope by its name.
    ///
    /// # Parameters
    /// - `name`: The name of the variable to look up.
    /// - `loc`: Location in the source code, used for errors.
    ///
    /// # Errors
    /// - `SemanticErrorType::VariableNotFound`: If the variable is not found in the current scope
    pub fn get_local_variable(
        &self,
        name: &str,
        loc: (usize, usize),
    ) -> Result<Type, SemanticError> {
        self.variables.get(name).map_or_else(
            || {
                Err(SemanticError {
                    error_type: SemanticErrorType::VariableNotFound(name.to_string()),
                    line: loc.0,
                    column: loc.1,
                })
            },
            |var| Ok(var.var_type.clone()),
        )
    }

    /// Add a function to the current scope.
    ///
    /// # Parameters
    /// - `name`: The name of the function to add.
    /// - `function`: The function to add to the current scope.
    /// - `loc`: Location in the source code, used for errors.
    ///
    /// # Errors
    /// - `SemanticErrorType::ShadowingFunction`: If a function with the same name already exists in
    ///   the current scope.
    /// - `SemanticErrorType::ShadowingClass`: If a class with the same name as the function already
    ///   exists in the current scope.
    pub fn add_function(
        &mut self,
        name: String,
        function: Function,
        loc: (usize, usize),
    ) -> Result<(), SemanticError> {
        self.check_shadowing(&name, ShadowingCheck::Function, loc)?;
        self.functions.insert(name, function);
        Ok(())
    }

    /// Gets a function by its name, searching through parent scopes if necessary.
    ///
    /// # Parameters
    /// - `name`: The name of the function to look up.
    /// - `loc`: Location in the source code, used for errors.
    ///
    /// # Errors
    /// - `SemanticErrorType::FunctionNotFound`: If the function is not found in the current scope or
    ///   any parent scope
    pub fn get_function(&self, name: &str, loc: (usize, usize)) -> Result<Function, SemanticError> {
        self.functions
            .get(name)
            .map_or_else(
                || {
                    self.parent.as_ref().map_or_else(
                        || Err(SemanticErrorType::FunctionNotFound(name.to_string())),
                        |parent_scope| {
                            parent_scope
                                .get_function(name, loc)
                                .map_err(|e| e.error_type)
                        },
                    )
                },
                |function| Ok(function.clone()),
            )
            .map_err(|e| SemanticError {
                error_type: e,
                line: loc.0,
                column: loc.1,
            })
    }

    /// Add a class to the current scope.
    ///
    /// # Parameters
    /// - `class`: The class to add to the current scope.
    /// - `loc`: Location in the source code, used for errors.
    ///
    /// # Errors
    /// - `SemanticErrorType::ShadowingFunction`: If a function with the same name as the class already
    ///   exists in the current scope.
    /// - `SemanticErrorType::ShadowingClass`: If a class with the same name already exists in the
    ///   current scope.
    pub fn add_class(&mut self, class: Class, loc: (usize, usize)) -> Result<(), SemanticError> {
        self.check_shadowing(&class.name, ShadowingCheck::Class, loc)?;
        self.classes.insert(class.name.clone(), class);
        Ok(())
    }

    /// Gets a class by its name, searching through parent scopes if necessary.
    ///
    /// # Parameters
    /// - `name`: The name of the class to look up.
    /// - `loc`: Location in the source code, used for errors.
    ///
    /// # Errors
    /// - `SemanticErrorType::ClassNotFound`: If the class is not found in the current scope or any
    ///   parent scope.
    pub fn get_class(&self, name: &str, loc: (usize, usize)) -> Result<Class, SemanticError> {
        self.classes
            .get(name)
            .map_or_else(
                || {
                    self.parent.as_ref().map_or_else(
                        || Err(SemanticErrorType::ClassNotFound(name.to_string())),
                        |parent| parent.get_class(name, loc).map_err(|e| e.error_type),
                    )
                },
                |class| Ok(class.clone()),
            )
            .map_err(|e| SemanticError {
                error_type: e,
                line: loc.0,
                column: loc.1,
            })
    }

    /// Gets a class field by its name, searching through parent scopes if necessary.
    ///
    /// # Parameters
    /// - `class_name`: The name of the class to look up.
    /// - `field_name`: The name of the field in class `class_name` to look up.
    /// - `loc`: Location in the source code, used for errors.
    ///
    /// # Returns
    /// The type of the field and whether or not the field is static (i.e. if it belongs to the
    /// class instead of an instance of the class).
    ///
    /// # Errors
    /// - `SemanticErrorType::ClassNotFound`: If the class is not found in the current scope or any
    ///   parent scope.
    /// - `SemanticErrorType::FieldNotFound`: If the field is not found in the class definition.
    pub fn get_class_field(
        &self,
        class_name: &str,
        field_name: &str,
        loc: (usize, usize),
    ) -> Result<Field, SemanticError> {
        let class: Class = self.get_class(class_name, loc)?;

        class
            .fields
            .get(field_name)
            .cloned()
            .ok_or_else(|| SemanticError {
                error_type: SemanticErrorType::FieldNotFound {
                    class: class_name.to_string(),
                    field: field_name.to_string(),
                },
                line: loc.0,
                column: loc.1,
            })
    }

    /// Gets a class method by its name, searching through parent scopes if necessary.
    ///
    /// # Parameters
    /// - `class_name`: The name of the class to look up.
    /// - `method_name`: The name of the method in class `class_name` to look up.
    /// - `loc`: Location in the source code, used for errors.
    ///
    /// # Errors
    /// - `SemanticErrorType::ClassNotFound`: If the class is not found in the current scope or any
    ///   parent scope.
    /// - `SemanticErrorType::MethodNotFound`: If the method is not found in the class definition.
    pub fn get_class_method(
        &self,
        class_name: &str,
        method_name: &str,
        loc: (usize, usize),
    ) -> Result<Function, SemanticError> {
        let class: Class = self.get_class(class_name, loc)?;

        class
            .methods
            .get(method_name)
            .cloned()
            .ok_or_else(|| SemanticError {
                error_type: SemanticErrorType::MethodNotFound {
                    class: class_name.to_string(),
                    method: method_name.to_string(),
                },
                line: loc.0,
                column: loc.1,
            })
    }

    /// Check if the assigned value's type matches the field's type.
    ///
    /// # Parameters
    /// - `class`: The name of the class containing the field being assigned to.
    /// - `field_name`: The name of the field being assigned to.
    /// - `value_type`: The type of the value being assigned to the field.
    /// - `loc`: Location in the source code, used for errors.
    ///
    /// # Errors
    /// - `SemanticErrorType::TypeMismatch`: If the type of the value being assigned to the field
    ///   does not match the field's type.
    /// - `SemanticErrorType::FieldNotFound`: If the field is not found in the class definition.
    pub fn assign_field(
        &mut self,
        class: &str,
        field_name: &str,
        value_type: &Type,
        loc: (usize, usize),
    ) -> Result<(), SemanticError> {
        let field: Field = self.get_class_field(class, field_name, loc)?;

        if field.field_type == *value_type {
            Ok(())
        } else {
            Err(SemanticError {
                error_type: SemanticErrorType::VariableAssignmentTypeMismatch {
                    expected: (&field.field_type).into(),
                    found: value_type.into(),
                },
                line: loc.0,
                column: loc.1,
            })
        }
    }

    fn check_shadowing(
        &self,
        name: &str,
        check_type: ShadowingCheck,
        loc: (usize, usize),
    ) -> Result<(), SemanticError> {
        (if check_type != ShadowingCheck::Variable && self.variables.contains_key(name) {
            Err(SemanticErrorType::ShadowingVariable(name.to_string()))
        } else if check_type != ShadowingCheck::Function && self.functions.contains_key(name) {
            Err(SemanticErrorType::ShadowingFunction(name.to_string()))
        } else if check_type != ShadowingCheck::Class && self.classes.contains_key(name) {
            Err(SemanticErrorType::ShadowingClass(name.to_string()))
        } else {
            Ok(())
        })
        .map_err(|e| SemanticError {
            error_type: e,
            line: loc.0,
            column: loc.1,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShadowingCheck {
    Variable,
    Function,
    Class,
}

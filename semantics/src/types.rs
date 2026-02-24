//! Contains the types used in the semantic analysis of the language.

use std::collections::HashMap;

use crate::errors::SemanticError;

/// Represents the result of analyzing a statement, which does not have a type.
pub type StatementReturn = Result<(), SemanticError>;

/// Represents the result of analyzing an expression, which has a type which will be returned.
pub type ExpressionReturn = Result<Type, SemanticError>;

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
    /// Fields defined in the class
    pub fields: HashMap<String, Type>,
    /// Methods defined in the class
    pub methods: HashMap<String, Function>,
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

impl From<Type> for String {
    fn from(val: Type) -> Self {
        match val {
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::Boolean => "bool".to_string(),
            Type::String => "string".to_string(),
            Type::Void => "void".to_string(),
            Type::Class(class_name) => class_name,
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
            class_name => Self::Class(class_name.to_string()),
        }
    }
}

impl From<&String> for Type {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<String> for Type {
    fn from(value: String) -> Self {
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
    ///
    /// # Errors
    /// - `SemanticError::ShadowingFunction`: If a function with the same name already exists in
    ///   the current scope.
    pub fn add_variable(&mut self, name: String, var_type: Type) -> Result<(), SemanticError> {
        self.check_shadowing(&name, ShadowingCheck::Variable)?;
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
    ///
    /// # Errors
    /// - `SemanticError::TypeMismatch`: If the type of the value being assigned does not match the
    ///   variable's type.
    /// - `SemanticError::VariableNotFound`: If the variable is not found in the current scope or
    ///   any parent scope.
    /// - `SemanticError::VariableUninitialized`: If the variable is found but hasn't been
    ///   initialized yet.
    pub fn assign_variable(&mut self, name: &str, value_type: &Type) -> Result<(), SemanticError> {
        let var_type: Type = self.get_variable(name)?;

        if var_type == *value_type {
            Ok(())
        } else {
            Err(SemanticError::VariableAssignmentTypeMismatch {
                expected: var_type.into(),
                found: value_type.clone().into(),
            })
        }
    }

    /// Get the type of a variable by its name, searching through parent scopes if necessary.
    ///
    /// # Parameters
    /// - `name`: The name of the variable to look up.
    ///
    /// # Errors
    /// - `SemanticError::VariableNotFound`: If the variable is not found in the current scope or
    ///   any parent scope.
    /// - `SemanticError::VariableUninitialized`: If the variable is found but hasn't been
    ///   initialized yet.
    pub fn get_variable(&self, name: &str) -> Result<Type, SemanticError> {
        self.variables.get(name).map_or_else(
            || {
                self.parent.as_ref().map_or_else(
                    || Err(SemanticError::VariableNotFound(name.to_string())),
                    |parent_scope| parent_scope.get_variable(name),
                )
            },
            |var| {
                if var.initialized {
                    Err(SemanticError::VariableUninitialized(name.to_string()))
                } else {
                    Ok(var.var_type.clone())
                }
            },
        )
    }

    /// Add a function to the current scope.
    ///
    /// # Parameters
    /// - `name`: The name of the function to add.
    /// - `function`: The function to add to the current scope.
    ///
    /// # Errors
    /// - `SemanticError::ShadowingFunction`: If a function with the same name already exists in
    ///   the current scope.
    /// - `SemanticError::ShadowingClass`: If a class with the same name as the function already
    ///   exists in the current scope.
    pub fn add_function(&mut self, name: String, function: Function) -> Result<(), SemanticError> {
        self.check_shadowing(&name, ShadowingCheck::Function)?;
        self.functions.insert(name, function);
        Ok(())
    }

    /// Gets a function by its name, searching through parent scopes if necessary.
    ///
    /// # Parameters
    /// - `name`: The name of the function to look up.
    ///
    /// # Errors
    /// - `SemanticError::FunctionNotFound`: If the function is not found in the current scope or
    ///   any parent scope
    pub fn get_function(&self, name: &str) -> Result<Function, SemanticError> {
        self.functions.get(name).map_or_else(
            || {
                self.parent.as_ref().map_or_else(
                    || Err(SemanticError::FunctionNotFound(name.to_string())),
                    |parent_scope| parent_scope.get_function(name),
                )
            },
            |function| Ok(function.clone()),
        )
    }

    /// Add a class to the current scope.
    ///
    /// # Parameters
    /// - `class`: The class to add to the current scope.
    ///
    /// # Errors
    /// - `SemanticError::ShadowingFunction`: If a function with the same name as the class already
    ///   exists in the current scope.
    /// - `SemanticError::ShadowingClass`: If a class with the same name already exists in the
    ///   current scope.
    pub fn add_class(&mut self, class: Class) -> Result<(), SemanticError> {
        self.check_shadowing(&class.name, ShadowingCheck::Class)?;
        self.classes.insert(class.name.clone(), class);
        Ok(())
    }

    /// Gets a class by its name, searching through parent scopes if necessary.
    ///
    /// # Parameters
    /// - `name`: The name of the class to look up.
    ///
    /// # Errors
    /// - `SemanticError::ClassNotFound`: If the class is not found in the current scope or any
    ///   parent scope.
    pub fn get_class(&self, name: &str) -> Result<Class, SemanticError> {
        self.classes.get(name).map_or_else(
            || {
                self.parent.as_ref().map_or_else(
                    || Err(SemanticError::ClassNotFound(name.to_string())),
                    |parent| parent.get_class(name),
                )
            },
            |class| Ok(class.clone()),
        )
    }

    /// Gets a class field by its name, searching through parent scopes if necessary.
    ///
    /// # Parameters
    /// - `class_name`: The name of the class to look up.
    /// - `field_name`: The name of the field in class `class_name` to look up.
    ///
    /// # Errors
    /// - `SemanticError::ClassNotFound`: If the class is not found in the current scope or any
    ///   parent scope.
    /// - `SemanticError::FieldNotFound`: If the field is not found in the class definition.
    pub fn get_class_field(
        &self,
        class_name: &str,
        field_name: &str,
    ) -> Result<Type, SemanticError> {
        let class: Class = self.get_class(class_name)?;

        class
            .fields
            .get(field_name)
            .cloned()
            .ok_or_else(|| SemanticError::FieldNotFound {
                class: class_name.to_string(),
                field: field_name.to_string(),
            })
    }

    /// Gets a class method by its name, searching through parent scopes if necessary.
    ///
    /// # Parameters
    /// - `class_name`: The name of the class to look up.
    /// - `method_name`: The name of the method in class `class_name` to look up.
    ///
    /// # Errors
    /// - `SemanticError::ClassNotFound`: If the class is not found in the current scope or any
    ///   parent scope.
    /// - `SemanticError::MethodNotFound`: If the method is not found in the class definition.
    pub fn get_class_method(
        &self,
        class_name: &str,
        method_name: &str,
    ) -> Result<Function, SemanticError> {
        let class: Class = self.get_class(class_name)?;

        class
            .methods
            .get(method_name)
            .cloned()
            .ok_or_else(|| SemanticError::MethodNotFound {
                class: class_name.to_string(),
                method: method_name.to_string(),
            })
    }

    fn check_shadowing(&self, name: &str, check_type: ShadowingCheck) -> Result<(), SemanticError> {
        if check_type != ShadowingCheck::Variable && self.variables.contains_key(name) {
            return Err(SemanticError::ShadowingVariable(name.to_string()));
        }
        if check_type != ShadowingCheck::Function && self.functions.contains_key(name) {
            return Err(SemanticError::ShadowingFunction(name.to_string()));
        }
        if check_type != ShadowingCheck::Class && self.classes.contains_key(name) {
            return Err(SemanticError::ShadowingClass(name.to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShadowingCheck {
    Variable,
    Function,
    Class,
}

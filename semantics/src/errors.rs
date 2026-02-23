//! Contains the different errors that can come up during semantic analysis

/// Represents an error that can occur during semantic analysis, such as type errors or scope
/// resolution
pub enum SemanticError {
    /// User tried to create a function or class with the same name as an existing variable in the
    /// current scope.
    ShadowingVariable(String),
    /// User tried to create a variable or class with the same name as an existing function in the
    /// current scope.
    ShadowingFunction(String),
    /// User tried to create a variable or function with the same name as an existing class in the
    /// current scope.
    ShadowingClass(String),
    /// User tried to access a variable that doesn't exist in the current scope or any parent
    /// scope.
    VariableNotFound(String),
    /// User tried to access a function that doesn't exist in the current scope or any parent
    /// scope.
    FunctionNotFound(String),
    /// User tried to access a class that doesn't exist in the current scope or any parent scope.
    ClassNotFound(String),
    /// User tried to access a class' field that doesn't exist in the class definition.
    FieldNotFound {
        /// The name of the class that was being accessed.
        class: String,
        /// The name of the field that was being accessed inside `class`.
        field: String,
    },
    /// User tried to access a class' method that doesn't exist in the class definition.
    MethodNotFound {
        /// The name of the class that was being accessed.
        class: String,
        /// The name of the method that was being accessed inside `class`.
        method: String,
    },
}

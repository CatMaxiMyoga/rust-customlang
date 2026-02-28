//! Contains the different errors that can come up during semantic analysis

/// Represents an error that can occur during semantic analysis, including the type of error and
/// the location in the source code where the error occurred.
pub struct SemanticError {
    /// The type of semantic error that occurred.
    pub error_type: SemanticErrorType,
    /// The line number in the source code where the error occurred.
    pub line: usize,
    /// The column number in the source code where the error occurred.
    pub column: usize,
}

impl SemanticError {
    pub fn error_message(&self) -> String {
        let mut message: String = String::new();

        message.push_str("SemanticAnalysisError: ");
        message.push_str(self.error_type.error_name());
        message.push_str("' at [");
        message.push_str(&self.line.to_string());
        message.push_str(":");
        message.push_str(&self.column.to_string());
        message.push_str("]: ");
        message.push_str(&self.error_type.message());

        message
    }
}

/// Represents an error that can occur during semantic analysis, such as type errors or scope
/// resolution
pub enum SemanticErrorType {
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
    /// User tried to access a variable that exists but hasn't been initialized yet.
    VariableUninitialized(String),
    /// User tried to assign a value of one type to a variable of a different type.
    VariableAssignmentTypeMismatch {
        /// The actually expected type of the variable being assigned to.
        expected: String,
        /// The type of the value that was being assigned to the variable.
        found: String,
    },
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

impl SemanticErrorType {
    /// Returns a human-readable error message describing the semantic error.
    pub fn message(&self) -> String {
        match self {
            Self::ShadowingVariable(var) => Self::one_var_message(
                "Cannot declare function or class",
                var,
                "because a variable with that name already exists in the current scope",
            ),
            Self::ShadowingFunction(func) => Self::one_var_message(
                "Cannot declare variable or class",
                func,
                "because a function with the same name already exists in the current scope",
            ),
            Self::ShadowingClass(class) => Self::one_var_message(
                "Cannot declare variable or function",
                class,
                "because a class with the same name already exists in the current scope",
            ),
            Self::VariableNotFound(var) => Self::one_var_message(
                "Tried to access variable",
                var,
                "which does not exist in the current or any parent scope",
            ),
            Self::VariableUninitialized(var) => Self::one_var_message(
                "Tried to access variable",
                var,
                "which exists but has not been assigned a value yet",
            ),
            Self::VariableAssignmentTypeMismatch { expected, found } => Self::two_var_message(
                "Tried to assign a value of type",
                found,
                "to a variable of type ",
                expected,
                "",
            ),
            Self::FunctionNotFound(func) => Self::one_var_message(
                "Tried to access function",
                func,
                "which does not exist in the current or any parent scope",
            ),
            Self::ClassNotFound(class) => Self::one_var_message(
                "Tried to access class",
                class,
                "which does not exist in the current or any parent scope",
            ),
            Self::FieldNotFound { class, field } => Self::two_var_message(
                "Tried to access field",
                field,
                "which does not exist in class ",
                class,
                "",
            ),
            Self::MethodNotFound { class, method } => Self::two_var_message(
                "Tried to access method",
                method,
                "which does not exist in class ",
                class,
                "",
            ),
        }
    }

    fn one_var_message(part1: &str, var: &str, part2: &str) -> String {
        format!("{} '{}' {}", part1, var, part2)
    }

    fn two_var_message(part1: &str, var1: &str, part2: &str, var2: &str, part3: &str) -> String {
        if part3.is_empty() {
            format!("{} '{}' {} '{}'", part1, var1, part2, var2)
        } else {
            format!("{} '{}' {} '{}' {}", part1, var1, part2, var2, part3)
        }
    }

    /// Returns the name of the error type as a string.
    pub const fn error_name(&self) -> &'static str {
        match self {
            Self::ShadowingVariable(_) => "ShadowingVariable",
            Self::ShadowingFunction(_) => "ShadowingFunction",
            Self::ShadowingClass(_) => "ShadowingClass",
            Self::VariableNotFound(_) => "VariableNotFound",
            Self::VariableUninitialized(_) => "VariableUninitialized",
            Self::VariableAssignmentTypeMismatch { .. } => "VariableAssignmentTypeMismatch",
            Self::FunctionNotFound(_) => "FunctionNotFound",
            Self::ClassNotFound(_) => "ClassNotFound",
            Self::FieldNotFound { .. } => "FieldNotFound",
            Self::MethodNotFound { .. } => "MethodNotFound",
        }
    }
}

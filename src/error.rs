use std::fmt;

/// Unified error type for all Catalyst operations
#[derive(Debug)]
pub enum CatalystError {
    FileError(String),
    CommandError(crate::engine::commands::CommandError),
    HttpError(String),
    JsonError(String),
    ValidationError(String),
    ConfigError(String),
    VariableError(String),
    TestError(String),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
}

impl fmt::Display for CatalystError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CatalystError::FileError(msg) => write!(f, "File operation failed: {msg}"),
            CatalystError::CommandError(err) => write!(f, "Command execution failed: {err}"),
            CatalystError::HttpError(msg) => write!(f, "HTTP request failed: {msg}"),
            CatalystError::JsonError(msg) => write!(f, "JSON processing failed: {msg}"),
            CatalystError::ValidationError(msg) => write!(f, "Validation failed: {msg}"),
            CatalystError::ConfigError(msg) => write!(f, "Configuration error: {msg}"),
            CatalystError::VariableError(msg) => write!(f, "Variable substitution failed: {msg}"),
            CatalystError::TestError(msg) => write!(f, "Test execution failed: {msg}"),
            CatalystError::IoError(err) => write!(f, "IO operation failed: {err}"),
            CatalystError::SerdeError(err) => write!(f, "JSON serialization/deserialization failed: {err}"),
        }
    }
}

impl std::error::Error for CatalystError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CatalystError::CommandError(err) => Some(err),
            CatalystError::IoError(err) => Some(err),
            CatalystError::SerdeError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<crate::engine::commands::CommandError> for CatalystError {
    fn from(err: crate::engine::commands::CommandError) -> Self {
        CatalystError::CommandError(err)
    }
}

impl From<std::io::Error> for CatalystError {
    fn from(err: std::io::Error) -> Self {
        CatalystError::IoError(err)
    }
}

impl From<serde_json::Error> for CatalystError {
    fn from(err: serde_json::Error) -> Self {
        CatalystError::SerdeError(err)
    }
}

/// Result type alias for Catalyst operations
pub type CatalystResult<T> = Result<T, CatalystError>;

impl CatalystError {
    /// Create a file error with context
    pub fn file_error(msg: impl Into<String>) -> Self {
        CatalystError::FileError(msg.into())
    }
    
    /// Create an HTTP error with context  
    pub fn http_error(msg: impl Into<String>) -> Self {
        CatalystError::HttpError(msg.into())
    }
    
    /// Create a JSON error with context
    pub fn json_error(msg: impl Into<String>) -> Self {
        CatalystError::JsonError(msg.into())
    }
    
    /// Create a validation error with context
    pub fn validation_error(msg: impl Into<String>) -> Self {
        CatalystError::ValidationError(msg.into())
    }
    
    /// Create a configuration error with context
    pub fn config_error(msg: impl Into<String>) -> Self {
        CatalystError::ConfigError(msg.into())
    }
    
    /// Create a variable substitution error with context
    pub fn variable_error(msg: impl Into<String>) -> Self {
        CatalystError::VariableError(msg.into())
    }
    
    /// Create a test execution error with context
    pub fn test_error(msg: impl Into<String>) -> Self {
        CatalystError::TestError(msg.into())
    }
}

/// Extension trait to convert string results to CatalystError
pub trait StringResultExt<T> {
    fn map_file_err(self) -> CatalystResult<T>;
    fn map_http_err(self) -> CatalystResult<T>;
    fn map_json_err(self) -> CatalystResult<T>;
    fn map_validation_err(self) -> CatalystResult<T>;
    fn map_config_err(self) -> CatalystResult<T>;
    fn map_variable_err(self) -> CatalystResult<T>;
    fn map_test_err(self) -> CatalystResult<T>;
}

impl<T> StringResultExt<T> for Result<T, String> {
    fn map_file_err(self) -> CatalystResult<T> {
        self.map_err(CatalystError::file_error)
    }
    
    fn map_http_err(self) -> CatalystResult<T> {
        self.map_err(CatalystError::http_error)
    }
    
    fn map_json_err(self) -> CatalystResult<T> {
        self.map_err(CatalystError::json_error)
    }
    
    fn map_validation_err(self) -> CatalystResult<T> {
        self.map_err(CatalystError::validation_error)
    }
    
    fn map_config_err(self) -> CatalystResult<T> {
        self.map_err(CatalystError::config_error)
    }
    
    fn map_variable_err(self) -> CatalystResult<T> {
        self.map_err(CatalystError::variable_error)
    }
    
    fn map_test_err(self) -> CatalystResult<T> {
        self.map_err(CatalystError::test_error)
    }
}
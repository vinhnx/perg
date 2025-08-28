use std::fmt;

/// Custom error type for perg operations
#[derive(Debug)]
pub enum PergError {
    /// File I/O errors
    Io(std::io::Error),
    /// Regex compilation errors
    Regex(regex::Error),
    /// File not found or permission denied
    FileNotFound(String),
    /// Invalid pattern
    InvalidPattern(String),
}

impl fmt::Display for PergError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PergError::Io(err) => write!(f, "I/O error: {}", err),
            PergError::Regex(err) => write!(f, "Regex error: {}", err),
            PergError::FileNotFound(path) => write!(f, "File not found: {}", path),
            PergError::InvalidPattern(pattern) => write!(f, "Invalid pattern: {}", pattern),
        }
    }
}

impl std::error::Error for PergError {}

impl From<std::io::Error> for PergError {
    fn from(err: std::io::Error) -> Self {
        PergError::Io(err)
    }
}

impl From<regex::Error> for PergError {
    fn from(err: regex::Error) -> Self {
        PergError::Regex(err)
    }
}

/// Result type alias for perg operations
pub type Result<T> = std::result::Result<T, PergError>;

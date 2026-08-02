use std::fmt;

/// Custom error types for Slugify operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlugifyError {
    InvalidRegexPattern(String),
    InvalidReplacementFormat(String),
}

impl fmt::Display for SlugifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlugifyError::InvalidRegexPattern(msg) => write!(f, "Invalid regex pattern: {}", msg),
            SlugifyError::InvalidReplacementFormat(msg) => write!(f, "Invalid replacement format: {}", msg),
        }
    }
}

impl std::error::Error for SlugifyError {}

pub type Result<T> = std::result::Result<T, SlugifyError>;

//! Parser error types

use std::fmt;

/// Error type for parser operations
#[derive(Debug, Clone)]
pub enum ParseError {
    /// HTML parsing failed
    HtmlParseError(String),

    /// Invalid HTML structure
    InvalidStructure(String),

    /// Unknown error
    Unknown(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HtmlParseError(msg) => write!(f, "HTML parse error: {}", msg),
            Self::InvalidStructure(msg) => write!(f, "Invalid structure: {}", msg),
            Self::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

/// Result type for parser operations
pub type ParseResult<T> = Result<T, ParseError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ParseError::HtmlParseError("Invalid tag".to_string());
        assert_eq!(format!("{}", err), "HTML parse error: Invalid tag");
    }
}

//! Repository error types

use std::fmt;

/// Error type for repository operations
#[derive(Debug, Clone)]
pub enum RepositoryError {
    /// Network request failed
    NetworkError(String),

    /// API returned an error response
    ApiError(String),

    /// Parsing error
    ParseError(String),

    /// Not found
    NotFound(String),

    /// Authentication failed
    AuthenticationError(String),

    /// Rate limited
    RateLimited,

    /// Unknown error
    Unknown(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Self::ApiError(msg) => write!(f, "API error: {}", msg),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            Self::RateLimited => write!(f, "Rate limited"),
            Self::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

/// Result type for repository operations
pub type RepositoryResult<T> = Result<T, RepositoryError>;

// ============================================================================
// CONVERSIONS FROM OTHER ERROR TYPES
// ============================================================================

impl From<Box<dyn std::error::Error + Send + Sync>> for RepositoryError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::NetworkError(err.to_string())
    }
}

impl From<reqwest::Error> for RepositoryError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Self::NetworkError("Request timeout".to_string())
        } else if err.is_connect() {
            Self::NetworkError("Connection failed".to_string())
        } else if err.is_request() {
            Self::NetworkError(format!("Request error: {}", err))
        } else {
            Self::Unknown(err.to_string())
        }
    }
}

impl From<serde_json::Error> for RepositoryError {
    fn from(err: serde_json::Error) -> Self {
        Self::ParseError(format!("JSON parse error: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = RepositoryError::NetworkError("Connection failed".to_string());
        assert_eq!(format!("{}", err), "Network error: Connection failed");
    }

    #[test]
    fn test_not_found() {
        let err = RepositoryError::NotFound("Thread 123".to_string());
        assert_eq!(format!("{}", err), "Not found: Thread 123");
    }

    #[test]
    fn test_rate_limited() {
        let err = RepositoryError::RateLimited;
        assert_eq!(format!("{}", err), "Rate limited");
    }
}

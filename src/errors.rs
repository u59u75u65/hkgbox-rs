//! Domain-specific error types
//!
//! This module provides comprehensive error types for different domains of the application,
//! using the `thiserror` crate for better error handling and reporting.

use thiserror::Error;

/// API-related errors
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    HttpRequestFailed(#[from] reqwest::Error),

    #[error("API returned error response: {message}")]
    ApiResponse { message: String },

    #[error("Failed to parse JSON response: {0}")]
    JsonParseFailed(#[from] serde_json::Error),

    #[error("Invalid API endpoint: {endpoint}")]
    InvalidEndpoint { endpoint: String },

    #[error("API timeout after {seconds}s")]
    Timeout { seconds: u64 },

    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

/// Cache-related errors
#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Cache read failed: {0}")]
    ReadFailed(String),

    #[error("Cache write failed: {0}")]
    WriteFailed(String),

    #[error("Cache entry not found: {key}")]
    NotFound { key: String },

    #[error("Cache directory error: {0}")]
    DirectoryError(String),

    #[error("Cache size limit exceeded: {size} bytes (max: {max} bytes)")]
    SizeLimitExceeded { size: usize, max: usize },

    #[error("Cache entry expired")]
    Expired,
}

/// UI/Terminal-related errors
#[derive(Debug, Error)]
pub enum UIError {
    #[error("Terminal error: {0}")]
    TerminalError(String),

    #[error("Failed to get terminal size")]
    TerminalSizeError,

    #[error("Screen render failed: {0}")]
    RenderFailed(String),

    #[error("Invalid key input: {0}")]
    InvalidInput(String),

    #[error("Screen buffer overflow")]
    BufferOverflow,
}

/// Data model/parsing errors
#[derive(Debug, Error)]
pub enum DataError {
    #[error("Failed to parse HTML: {0}")]
    HtmlParseFailed(String),

    #[error("Invalid data format: {0}")]
    InvalidFormat(String),

    #[error("Missing required field: {field}")]
    MissingField { field: String },

    #[error("Data validation failed: {0}")]
    ValidationFailed(String),
}

/// Configuration errors
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Configuration file not found: {path}")]
    FileNotFound { path: String },

    #[error("Failed to parse configuration: {0}")]
    ParseError(String),

    #[error("Invalid configuration value: {field} = {value}")]
    InvalidValue { field: String, value: String },

    #[error("Missing required configuration: {field}")]
    MissingField { field: String },
}

/// Application error combining all domain errors
#[derive(Debug, Error)]
pub enum AppError {
    #[error("API error: {0}")]
    Api(#[from] ApiError),

    #[error("Cache error: {0}")]
    Cache(#[from] CacheError),

    #[error("UI error: {0}")]
    UI(#[from] UIError),

    #[error("Data error: {0}")]
    Data(#[from] DataError),

    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type alias for API operations
pub type ApiResult<T> = Result<T, ApiError>;

/// Result type alias for cache operations
pub type CacheResult<T> = Result<T, CacheError>;

/// Result type alias for UI operations
pub type UIResult<T> = Result<T, UIError>;

/// Result type alias for data operations
pub type DataResult<T> = Result<T, DataError>;

/// Result type alias for configuration operations
pub type ConfigResult<T> = Result<T, ConfigError>;

/// Result type alias for general application operations
pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_display() {
        let err = ApiError::InvalidEndpoint {
            endpoint: "/invalid".to_string(),
        };
        assert_eq!(err.to_string(), "Invalid API endpoint: /invalid");
    }

    #[test]
    fn test_cache_error_display() {
        let err = CacheError::NotFound {
            key: "test_key".to_string(),
        };
        assert_eq!(err.to_string(), "Cache entry not found: test_key");
    }

    #[test]
    fn test_error_conversion() {
        let api_err = ApiError::RateLimitExceeded;
        let app_err: AppError = api_err.into();
        assert_eq!(app_err.to_string(), "API error: Rate limit exceeded");
    }

    #[test]
    fn test_result_types() {
        fn api_function() -> ApiResult<String> {
            Ok("success".to_string())
        }

        fn cache_function() -> CacheResult<Vec<u8>> {
            Err(CacheError::NotFound {
                key: "test".to_string(),
            })
        }

        assert!(api_function().is_ok());
        assert!(cache_function().is_err());
    }
}

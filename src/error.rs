//! Error types for the HKGolden TUI application
use std::io;
use reqwest::Error as ReqwestError;

/// Main error type for the HKGolden application
#[derive(Debug, thiserror::Error)]
pub enum HkgError {
    /// HTML parsing errors
    #[error("Failed to parse HTML: {0}")]
    HtmlParse(String),

    /// HTTP request errors
    #[error("HTTP request failed: {0}")]
    HttpRequest(#[from] ReqwestError),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// URL parsing errors
    #[error("Failed to parse URL: {0}")]
    UrlParse(String),

    /// Cache errors
    #[error("Cache error: {0}")]
    Cache(String),

    /// JSON parsing errors
    #[error("Failed to parse JSON: {0}")]
    JsonParse(#[from] serde_json::Error),

    /// Channel communication errors
    #[error("Channel communication error: {0}")]
    Channel(String),

    /// Terminal initialization errors
    #[error("Failed to initialize terminal: {0}")]
    Terminal(String),

    /// Input parsing errors
    #[error("Failed to parse input: {0}")]
    InputParse(String),

    /// Image loading errors
    #[error("Failed to load image: {0}")]
    ImageLoad(String),

    /// API errors
    #[error("API error: {0}")]
    Api(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, HkgError>;

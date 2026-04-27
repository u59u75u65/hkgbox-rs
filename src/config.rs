//! Application configuration management
//!
//! This module provides centralized configuration management for the HKG application,
//! including API settings, cache configuration, and UI preferences.

use std::path::PathBuf;
use std::time::Duration;

/// Main application configuration
///
/// # Configuration Categories
/// - **API Settings**: API endpoint, timeout, request limits
/// - **Cache Settings**: Cache directory, size limits, expiration
/// - **UI Settings**: Terminal settings, display preferences
/// - **Performance Settings**: Concurrent requests, buffer sizes
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// API base URL
    pub api_base_url: String,

    /// API request timeout in seconds
    pub api_timeout_secs: u64,

    /// Maximum concurrent API requests
    pub max_concurrent_requests: usize,

    /// Maximum number of topics to fetch per API page
    pub api_topics_per_page: usize,

    /// Cache directory path
    pub cache_dir: PathBuf,

    /// Maximum cache size in bytes (0 = unlimited)
    pub max_cache_size_bytes: usize,

    /// Cache entry TTL in seconds (0 = no expiration)
    pub cache_ttl_secs: u64,

    /// Image cache directory path
    pub image_cache_dir: PathBuf,

    /// Maximum image cache size in bytes
    pub max_image_cache_size_bytes: usize,

    /// Terminal refresh rate in milliseconds
    pub terminal_refresh_rate_ms: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_base_url: "https://api.hkgolden.com/v1".to_string(),
            api_timeout_secs: 10,
            max_concurrent_requests: 5,
            api_topics_per_page: 30,
            cache_dir: PathBuf::from("data/cache"),
            max_cache_size_bytes: 100 * 1024 * 1024, // 100 MB
            cache_ttl_secs: 3600, // 1 hour
            image_cache_dir: PathBuf::from("data/cache/images"),
            max_image_cache_size_bytes: 500 * 1024 * 1024, // 500 MB
            terminal_refresh_rate_ms: 50,
        }
    }
}

impl AppConfig {
    /// Create a new configuration with default values
    ///
    /// # Examples
    /// ```
    /// use hkg::config::AppConfig;
    ///
    /// let config = AppConfig::new();
    /// assert_eq!(config.api_timeout_secs, 10);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Get API request timeout as Duration
    pub fn api_timeout(&self) -> Duration {
        Duration::from_secs(self.api_timeout_secs)
    }

    /// Get cache TTL as Duration
    pub fn cache_ttl(&self) -> Duration {
        Duration::from_secs(self.cache_ttl_secs)
    }

    /// Get terminal refresh rate as Duration
    pub fn refresh_rate(&self) -> Duration {
        Duration::from_millis(self.terminal_refresh_rate_ms)
    }

    /// Get full API URL for a specific endpoint
    ///
    /// # Arguments
    /// * `endpoint` - API endpoint path (e.g., "/topics/BW/1")
    ///
    /// # Returns
    /// Complete URL string
    pub fn api_url(&self, endpoint: &str) -> String {
        format!("{}{}", self.api_base_url, endpoint)
    }

    /// Validate configuration settings
    ///
    /// # Returns
    /// `Ok(())` if configuration is valid, `Err(String)` with error message otherwise
    pub fn validate(&self) -> Result<(), String> {
        if self.api_timeout_secs == 0 {
            return Err("API timeout must be greater than 0".to_string());
        }

        if self.max_concurrent_requests == 0 {
            return Err("Max concurrent requests must be greater than 0".to_string());
        }

        if self.api_topics_per_page == 0 {
            return Err("API topics per page must be greater than 0".to_string());
        }

        if self.terminal_refresh_rate_ms == 0 {
            return Err("Terminal refresh rate must be greater than 0".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.api_timeout_secs, 10);
        assert_eq!(config.max_concurrent_requests, 5);
        assert_eq!(config.api_topics_per_page, 30);
    }

    #[test]
    fn test_api_url_construction() {
        let config = AppConfig::new();
        let url = config.api_url("/topics/BW/1");
        assert_eq!(url, "https://api.hkgolden.com/v1/topics/BW/1");
    }

    #[test]
    fn test_duration_conversions() {
        let config = AppConfig::new();
        assert_eq!(config.api_timeout(), Duration::from_secs(10));
        assert_eq!(config.cache_ttl(), Duration::from_secs(3600));
        assert_eq!(config.refresh_rate(), Duration::from_millis(50));
    }

    #[test]
    fn test_validation() {
        let mut config = AppConfig::new();
        assert!(config.validate().is_ok());

        config.api_timeout_secs = 0;
        assert!(config.validate().is_err());
    }
}

//! Image service for business logic related to images
//!
//! This service handles image-related operations including validation,
/// URL processing, and cache management.

use crate::errors::{AppResult, UIError};
use log::{info, debug};
use std::path::PathBuf;

/// Service for image-related business logic
///
/// The `ImageService` encapsulates all business logic related to images,
/// providing validation, URL processing, and cache management.
pub struct ImageService {
    cache_dir: PathBuf,
    max_cache_size: usize,
}

impl ImageService {
    /// Create a new image service
    ///
    /// # Arguments
    /// * `cache_dir` - Directory for image cache
    /// * `max_cache_size` - Maximum cache size in bytes
    ///
    /// # Examples
    /// ```
    /// use hkg::services::ImageService;
    /// use std::path::PathBuf;
    ///
    /// let cache_dir = PathBuf::from("data/cache/images");
    /// let service = ImageService::new(cache_dir, 500 * 1024 * 1024);
    /// ```
    #[must_use]
    pub fn new(cache_dir: PathBuf, max_cache_size: usize) -> Self {
        Self { cache_dir, max_cache_size }
    }

    /// Validate image URL
    ///
    /// # Arguments
    /// * `url` - Image URL to validate
    ///
    /// # Returns
    /// `true` if URL is valid, `false` otherwise
    ///
    /// # Examples
    /// ```
    /// use hkg::services::ImageService;
    /// use std::path::PathBuf;
    ///
    /// let service = ImageService::new(PathBuf::new(), 0);
    /// assert!(service.is_valid_image_url("https://example.com/image.jpg"));
    /// assert!(!service.is_valid_image_url("not-a-url"));
    /// ```
    #[must_use]
    pub fn is_valid_image_url(&self, url: &str) -> bool {
        // Check if it starts with http/https
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return false;
        }

        // Check for common image extensions
        let valid_extensions = [".jpg", ".jpeg", ".png", ".gif", ".webp", ".bmp"];
        let url_lower = url.to_lowercase();

        valid_extensions.iter().any(|ext| url_lower.ends_with(ext))
    }

    /// Extract image ID from URL
    ///
    /// # Arguments
    /// * `url` - Image URL
    ///
    /// # Returns
    /// Image ID derived from URL or error if invalid
    ///
    /// # Examples
    /// ```
    /// use hkg::services::ImageService;
    /// use std::path::PathBuf;
    ///
    /// let service = ImageService::new(PathBuf::new(), 0);
    /// let url = "https://example.com/images/abc123.jpg";
    /// let id = service.extract_image_id(url).unwrap();
    /// ```
    pub fn extract_image_id(&self, url: &str) -> AppResult<String> {
        if !self.is_valid_image_url(url) {
            return Err(UIError::InvalidInput(format!("Invalid image URL: {}", url)).into());
        }

        // Extract filename from URL
        let parts: Vec<&str> = url.rsplit('/').collect();
        if let Some(filename) = parts.first() {
            // Remove extension
            let id = filename.rsplit('.').nth(1).unwrap_or(filename);
            Ok(id.to_string())
        } else {
            Err(UIError::InvalidInput("Cannot extract image ID from URL".to_string()).into())
        }
    }

    /// Generate cache file path for image
    ///
    /// # Arguments
    /// * `url` - Image URL
    ///
    /// # Returns
    /// Path where the image should be cached
    ///
    /// # Examples
    /// ```
    /// use hkg::services::ImageService;
    /// use std::path::PathBuf;
    ///
    /// let service = ImageService::new(PathBuf::from("cache"), 0);
    /// let path = service.get_cache_path("https://example.com/image.jpg");
    /// assert!(path.is_ok());
    /// ```
    pub fn get_cache_path(&self, url: &str) -> AppResult<PathBuf> {
        // Validate URL first
        let _id = self.extract_image_id(url)?;

        // Use a hash of the URL as filename to avoid conflicts
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        url.hash(&mut hasher);
        let hash = hasher.finish();

        let filename = format!("{:x}.jpg", hash);
        Ok(self.cache_dir.join(filename))
    }

    /// Calculate optimal cache size for terminal
    ///
    /// # Arguments
    /// * `terminal_width` - Terminal width in characters
    /// * `terminal_height` - Terminal height in characters
    /// * `image_count` - Number of images to cache
    ///
    /// # Returns
    /// Recommended cache size in bytes
    ///
    /// # Examples
    /// ```
    /// use hkg::services::ImageService;
    ///
    /// let size = ImageService::calculate_cache_size(80, 24, 10);
    /// assert!(size > 0);
    /// ```
    #[must_use]
    pub const fn calculate_cache_size(
        terminal_width: usize,
        terminal_height: usize,
        image_count: usize,
    ) -> usize {
        // Estimate image size based on terminal dimensions
        // Assume 2 bytes per pixel (RGB565) and some padding
        let pixel_count = terminal_width * terminal_height;
        let estimated_image_size = pixel_count * 2 + 1024; // +1KB for metadata

        estimated_image_size * image_count
    }

    /// Check if cache should be cleared
    ///
    /// # Returns
    /// `true` if cache size exceeds limit, `false` otherwise
    #[must_use]
    pub fn should_clear_cache(&self, current_size: usize) -> bool {
        current_size > self.max_cache_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_image_url() {
        let service = ImageService::new(PathBuf::new(), 0);
        assert!(service.is_valid_image_url("https://example.com/image.jpg"));
        assert!(service.is_valid_image_url("http://example.com/image.png"));
        assert!(!service.is_valid_image_url("ftp://example.com/image.jpg"));
        assert!(!service.is_valid_image_url("not-a-url"));
    }

    #[test]
    fn test_extract_image_id() {
        let service = ImageService::new(PathBuf::new(), 0);
        let url = "https://example.com/images/abc123.jpg";
        let id = service.extract_image_id(url);
        assert!(id.is_ok());
        assert_eq!(id.unwrap(), "abc123");

        let invalid = service.extract_image_id("invalid-url");
        assert!(invalid.is_err());
    }

    #[test]
    fn test_calculate_cache_size() {
        let size = ImageService::calculate_cache_size(80, 24, 10);
        assert!(size > 0);

        let larger = ImageService::calculate_cache_size(100, 30, 20);
        assert!(larger > size);
    }

    #[test]
    fn test_should_clear_cache() {
        let service = ImageService::new(PathBuf::new(), 1000);
        assert!(!service.should_clear_cache(500));
        assert!(service.should_clear_cache(1500));
    }
}

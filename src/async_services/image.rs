//! Async Image service for business logic related to images
//!
//! This async service handles image-related operations including validation,
//! URL processing, and cache management.

use std::path::PathBuf;

/// Async service for image-related business logic
///
/// The `AsyncImageService` provides validation, URL processing, and cache
/// management for images.
pub struct AsyncImageService {
    cache_dir: PathBuf,
    max_cache_size: usize,
}

impl AsyncImageService {
    /// Create a new AsyncImageService
    ///
    /// # Arguments
    /// * `cache_dir` - Directory for image cache
    /// * `max_cache_size` - Maximum number of cached images
    ///
    /// # Returns
    /// A new image service instance
    ///
    /// # Examples
    /// ```
    /// use hkg::async_services::AsyncImageService;
    /// use std::path::PathBuf;
    ///
    /// let cache_dir = PathBuf::from("data/cache/img");
    /// let service = AsyncImageService::new(cache_dir, 1000);
    /// ```
    pub fn new(cache_dir: PathBuf, max_cache_size: usize) -> Self {
        Self {
            cache_dir,
            max_cache_size,
        }
    }

    /// Validate if a URL is a valid image URL
    ///
    /// # Arguments
    /// * `url` - URL to validate
    ///
    /// # Returns
    /// true if the URL appears to be an image URL
    ///
    /// # Examples
    /// ```
    /// use hkg::async_services::AsyncImageService;
    /// use std::path::PathBuf;
    ///
    /// let service = AsyncImageService::new(PathBuf::from("cache"), 1000);
    /// assert!(service.is_valid_image_url("https://example.com/image.jpg"));
    /// assert!(service.is_valid_image_url("https://example.com/image.png"));
    /// assert!(!service.is_valid_image_url("https://example.com/file.pdf"));
    /// ```
    pub fn is_valid_image_url(&self, url: &str) -> bool {
        let url_lower = url.to_lowercase();

        // Check for common image extensions
        let extensions = [".jpg", ".jpeg", ".png", ".gif", ".webp", ".bmp"];

        // Check if URL ends with image extension
        for ext in &extensions {
            if url_lower.ends_with(ext) {
                return true;
            }
        }

        // Check if URL contains image extension (e.g., from CDN)
        for ext in &extensions {
            if url_lower.contains(ext) {
                return true;
            }
        }

        false
    }

    /// Extract image ID from URL
    ///
    /// # Arguments
    /// * `url` - Image URL
    ///
    /// # Returns
    /// Extracted image ID or empty string if not found
    ///
    /// # Examples
    /// ```
    /// use hkg::async_services::AsyncImageService;
    /// use std::path::PathBuf;
    ///
    /// let service = AsyncImageService::new(PathBuf::from("cache"), 1000);
    ///
    /// // Extract from cache URL
    /// let url = "https://cache.hkgolden.media/compress/12345.jpg";
    /// let id = service.extract_image_id(url);
    /// assert_eq!(id, "12345");
    /// ```
    pub fn extract_image_id(&self, url: &str) -> String {
        // Remove cache URL prefix
        let url = url.replace("https://cache.hkgolden.media/compress/", "");

        // Extract ID (filename without extension)
        if let Some(pos) = url.find('.') {
            url[..pos].to_string()
        } else {
            url.to_string()
        }
    }

    /// Calculate current cache size
    ///
    /// # Returns
    /// Number of files in cache directory
    ///
    /// # Examples
    /// ```no_run
    /// use hkg::async_services::AsyncImageService;
    /// use std::path::PathBuf;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let service = AsyncImageService::new(PathBuf::from("data/cache/img"), 1000);
    /// let size = service.calculate_cache_size().await?;
    /// println!("Cache size: {}", size);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn calculate_cache_size(&self) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        use tokio::fs;

        if !self.cache_dir.exists() {
            return Ok(0);
        }

        let mut entries = fs::read_dir(&self.cache_dir).await?;
        let mut count = 0;

        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;
            if metadata.is_file() {
                count += 1;
            }
        }

        Ok(count)
    }

    /// Check if cache should be cleared
    ///
    /// # Returns
    /// true if cache size exceeds maximum
    ///
    /// # Examples
    /// ```no_run
    /// use hkg::async_services::AsyncImageService;
    /// use std::path::PathBuf;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let service = AsyncImageService::new(PathBuf::from("data/cache/img"), 1000);
    /// if service.should_clear_cache().await? {
    ///     println!("Cache needs clearing");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn should_clear_cache(&self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let current_size = self.calculate_cache_size().await?;
        Ok(current_size > self.max_cache_size)
    }

    /// Get cache directory
    ///
    /// # Returns
    /// Path to cache directory
    pub fn cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }

    /// Get maximum cache size
    ///
    /// # Returns
    /// Maximum number of cached images
    pub fn max_cache_size(&self) -> usize {
        self.max_cache_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_image_url() {
        let service = AsyncImageService::new(PathBuf::from("cache"), 1000);

        // Valid image URLs
        assert!(service.is_valid_image_url("https://example.com/image.jpg"));
        assert!(service.is_valid_image_url("https://example.com/image.jpeg"));
        assert!(service.is_valid_image_url("https://example.com/image.png"));
        assert!(service.is_valid_image_url("https://example.com/image.gif"));
        assert!(service.is_valid_image_url("https://example.com/image.webp"));

        // Invalid URLs
        assert!(!service.is_valid_image_url("https://example.com/file.pdf"));
        assert!(!service.is_valid_image_url("https://example.com/document.html"));
        assert!(!service.is_valid_image_url("https://example.com/data.json"));
    }

    #[test]
    fn test_extract_image_id() {
        let service = AsyncImageService::new(PathBuf::from("cache"), 1000);

        // Extract from cache URL
        let url = "https://cache.hkgolden.media/compress/12345.jpg";
        assert_eq!(service.extract_image_id(url), "12345");

        // Extract without extension
        let url = "https://cache.hkgolden.media/compress/67890";
        assert_eq!(service.extract_image_id(url), "67890");

        // Extract complex filename
        let url = "https://cache.hkgolden.media/compress/abc123def456.jpg";
        assert_eq!(service.extract_image_id(url), "abc123def456");
    }

    #[test]
    fn test_cache_dir() {
        let cache_dir = PathBuf::from("test/cache");
        let service = AsyncImageService::new(cache_dir.clone(), 1000);
        assert_eq!(service.cache_dir(), &cache_dir);
    }

    #[test]
    fn test_max_cache_size() {
        let service = AsyncImageService::new(PathBuf::from("cache"), 1000);
        assert_eq!(service.max_cache_size(), 1000);
    }

    #[tokio::test]
    async fn test_calculate_cache_size_empty() {
        let service = AsyncImageService::new(PathBuf::from("/nonexistent/path"), 1000);
        let size = service.calculate_cache_size().await.unwrap();
        assert_eq!(size, 0);
    }
}

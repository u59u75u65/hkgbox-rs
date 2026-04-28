//! Image display and caching utilities
//!
//! This module provides functions for displaying images in the terminal
//! using iTerm2's imgcat protocol, with caching support.

use std::io::Read;
use base64::{Engine as _, engine::general_purpose};
use log::{info, error};

/// Display image data using iTerm2 imgcat protocol
///
/// # Arguments
/// * `data` - Raw image data (JPEG, PNG, etc.)
/// * `width` - Display width in characters
///
/// # Returns
/// ANSI escape sequence to display the image
///
/// # Examples
/// ```
/// use hkg::image_utils;
///
/// // In reality you'd have actual image data
/// let data = b"fake_image_data";
/// let escape_seq = image_utils::imgcat_from_data(data, 80);
/// ```
#[must_use]
pub fn imgcat_from_data(data: &[u8], width: usize) -> String {
    let encoded = general_purpose::STANDARD.encode(data);
    format!("\x1b]1337;File=inline=1;width={};content:{};\x07", width, encoded)
}

/// Fetch and display image from URL with caching
///
/// # Arguments
/// * `url` - Image URL to fetch
/// * `width` - Display width in characters
///
/// # Returns
/// ANSI escape sequence to display the image
///
/// # Errors
/// Returns error if HTTP request fails or data cannot be processed
///
/// # Examples
/// ```no_run
/// use hkg::image_utils;
///
/// let result = image_utils::imgcat_from_url("https://example.com/image.jpg", 80);
/// ```
pub fn imgcat_from_url(url: &str, width: usize) -> Result<String, Box<dyn std::error::Error>> {
    info!("[imgcat] Fetching image from URL: {}", url);

    // Generate cache filename from URL
    let cache_key = general_purpose::URL_SAFE.encode(url.as_bytes());
    let cache_path = format!("data/cache/img/{}", cache_key);

    // Check if image is already cached
    if let Some(cached_image) = display_from_cache(&cache_path) {
        info!("[imgcat] Using cached image: {}", cache_path);
        return Ok(cached_image);
    }

    // Fetch the image
    info!("[imgcat] Fetching from network...");

    // Check if this is a LIHKG image URL and add required headers
    let is_lihkg_image = url.contains("lihkg.com") || url.contains("lih.kg");

    let response = if is_lihkg_image {
        // Use LIHKG headers for LIHKG images
        info!("[imgcat] Using LIHKG headers for LIHKG image");
        let device_id = generate_lihkg_device_id();
        let load_time = calculate_lihkg_load_time();

        reqwest::blocking::Client::new()
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
            .header("Referer", "https://lihkg.com/")
            .header("x-li-device-type", "browser")
            .header("x-li-device", &device_id)
            .header("x-li-load-time", &load_time.to_string())
            .send()?
    } else {
        // Basic fetch for other images
        reqwest::blocking::get(url)?
    };

    if !response.status().is_success() {
        error!("[imgcat] HTTP error {}", response.status());
        return Err(format!("HTTP {}", response.status()).into());
    }

    let image_data = response.bytes()?.to_vec();
    info!("[imgcat] Downloaded {} bytes", image_data.len());

    // Save to cache
    if let Some(parent) = std::path::Path::new(&cache_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(&cache_path, &image_data)?;
    info!("[imgcat] Cached to {}", cache_path);

    // Display using imgcat
    Ok(imgcat_from_data(&image_data, width))
}

/// Try to load and display image from cache
///
/// # Arguments
/// * `path` - Path to cached image file
///
/// # Returns
/// `Some(escape_sequence)` if cache hit, `None` otherwise
fn display_from_cache(path: &str) -> Option<String> {
    info!("[imgcat] Loading cached image from {}", path);

    let mut file = std::fs::File::open(path).ok()?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).ok()?;

    info!("[imgcat] Loaded {} bytes from cache", buffer.len());
    Some(imgcat_from_data(&buffer, 80))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imgcat_from_data() {
        let data = b"test_image_data";
        let result = imgcat_from_data(data, 80);
        assert!(result.contains("\x1b]1337;File=inline=1;width=80;"));
        assert!(result.contains("dGVzdF9pbWFnZV9kYXRh")); // base64 of "test_image_data"
    }

    #[test]
    fn test_imgcat_from_data_different_width() {
        let data = b"test";
        let result = imgcat_from_data(data, 40);
        assert!(result.contains("width=40;"));
    }
}

/// Generate device ID for LIHKG image requests
///
/// Creates a device identifier as LIHKG expects
fn generate_lihkg_device_id() -> String {
    // Hardcoded working device ID from Playwright testing
    // This matches the format that successfully bypasses LIHKG rate limiting
    "89ee2a48214807d7762894d2ae9d07500e339171".to_string()
}

/// Calculate load time for LIHKG image requests
fn calculate_lihkg_load_time() -> f64 {
    // Hardcoded working load time from Playwright testing
    3.516740
}

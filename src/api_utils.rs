//! API utility functions (re-exports for backward compatibility)
//!
//! This module re-exports utility functions from their respective modules
//! for backward compatibility. New code should import directly from:
//! - `image_utils` for image display functions
//! - `html_parser` for HTML parsing functions
//! - `time_utils` for time conversion functions

// Re-export image utilities
pub use crate::image_utils::{imgcat_from_data, imgcat_from_url};

// Re-export HTML parsing utilities
pub use crate::html_parser::{extract_image_urls, parse_html_content};

// Re-export time utilities
pub use crate::time_utils::timestamp_to_strings;

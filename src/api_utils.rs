//! API utility functions
//!
//! This module re-exports functions from specialized modules for backwards compatibility.
//! The actual implementations are in:
//! - `image_utils` - Image display and caching
//! - `html_parser` - HTML parsing and content extraction
//! - `time_utils` - Time conversion utilities

pub use crate::image_utils::{imgcat_from_data, imgcat_from_url};
pub use crate::html_parser::{parse_html_content, extract_image_urls};
pub use crate::time_utils::timestamp_to_strings;

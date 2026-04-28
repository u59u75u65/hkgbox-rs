//! Parser abstraction layer
//!
//! This module defines trait-based parser interfaces for content parsing.
//! These abstractions allow decoupling business logic from specific parser implementations.

pub mod html_parser;
pub mod error;

pub use error::{ParseError, ParseResult};
pub use html_parser::HtmlContentParser;

use crate::domain::ContentNode;

/// Trait for parsing HTML content into structured nodes
///
/// This trait abstracts the HTML parser implementation, allowing different
/// parsing strategies or libraries to be used.
pub trait ContentParser: Send + Sync {
    /// Parse HTML content into structured nodes
    ///
    /// # Arguments
    /// * `html` - HTML string to parse
    ///
    /// # Returns
    /// Vector of content nodes
    ///
    /// # Errors
    /// Returns `ParseError` if parsing fails
    fn parse_content(&self, html: &str) -> ParseResult<Vec<ContentNode>>;
}

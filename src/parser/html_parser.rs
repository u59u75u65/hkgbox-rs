//! HTML content parser implementation
//!
//! This module provides an implementation of ContentParser using the existing
//! HTML parsing logic from html_parser.rs.

use crate::html_parser;
use crate::reply_model::NodeType;
use crate::domain::ContentNode;
use super::{ContentParser, ParseResult};

/// HTML content parser
///
/// Implements `ContentParser` using the existing HTML parsing logic.
/// This converts the existing `NodeType` enum to the generic `ContentNode` enum.
pub struct HtmlContentParser;

impl HtmlContentParser {
    /// Create a new HTML content parser
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for HtmlContentParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ContentParser for HtmlContentParser {
    fn parse_content(&self, html: &str) -> ParseResult<Vec<ContentNode>> {
        let nodes = html_parser::parse_html_content(html);

        let content_nodes: Vec<ContentNode> = nodes
            .into_iter()
            .map(|node_type| Self::convert_node_type(node_type))
            .collect();

        Ok(content_nodes)
    }
}

impl HtmlContentParser {
    /// Convert NodeType to ContentNode
    fn convert_node_type(node_type: NodeType) -> ContentNode {
        match node_type {
            NodeType::Text(text_node) => {
                ContentNode::Text(crate::domain::TextNode {
                    data: text_node.data,
                })
            }
            NodeType::Image(image_node) => {
                ContentNode::Image(crate::domain::ImageNode {
                    data: image_node.data,
                    alt: image_node.alt,
                })
            }
            NodeType::BlockQuote(blockquote_node) => {
                let data = blockquote_node.data
                    .into_iter()
                    .map(Self::convert_node_type)
                    .collect();
                ContentNode::BlockQuote(crate::domain::BlockQuoteNode { data })
            }
            NodeType::Br(_) => ContentNode::Br(crate::domain::BrNode {}),
            NodeType::Link(link_node) => {
                ContentNode::Link(crate::domain::LinkNode {
                    url: link_node.url,
                    text: link_node.text,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_html() {
        let parser = HtmlContentParser::new();
        let html = "<p>Hello world</p>";
        let result = parser.parse_content(html);

        assert!(result.is_ok());
        let nodes = result.unwrap();
        assert!(!nodes.is_empty());
    }

    #[test]
    fn test_parse_html_with_link() {
        let parser = HtmlContentParser::new();
        let html = r#"<a href="http://example.com">Link</a>"#;
        let result = parser.parse_content(html);

        assert!(result.is_ok());
        let nodes = result.unwrap();
        assert_eq!(nodes.len(), 1);

        if let ContentNode::Link(link) = &nodes[0] {
            assert_eq!(link.url, "http://example.com");
            assert_eq!(link.text, "Link");
        } else {
            panic!("Expected Link node");
        }
    }

    #[test]
    fn test_parse_html_with_image() {
        let parser = HtmlContentParser::new();
        let html = r#"<img src="https://example.com/image.jpg" alt="Example" />"#;
        let result = parser.parse_content(html);

        assert!(result.is_ok());
        let nodes = result.unwrap();
        assert_eq!(nodes.len(), 1);

        if let ContentNode::Image(img) = &nodes[0] {
            assert_eq!(img.data, "https://example.com/image.jpg");
            assert_eq!(img.alt, "Example");
        } else {
            panic!("Expected Image node");
        }
    }

    #[test]
    fn test_parse_html_with_blockquote() {
        let parser = HtmlContentParser::new();
        let html = r#"<blockquote><p>Quote text</p></blockquote>"#;
        let result = parser.parse_content(html);

        assert!(result.is_ok());
        let nodes = result.unwrap();
        assert_eq!(nodes.len(), 1);

        if let ContentNode::BlockQuote(blockquote) = &nodes[0] {
            assert!(!blockquote.data.is_empty());
        } else {
            panic!("Expected BlockQuote node");
        }
    }

    #[test]
    fn test_parse_html_with_br() {
        let parser = HtmlContentParser::new();
        let html = r#"<br />"#;
        let result = parser.parse_content(html);

        assert!(result.is_ok());
        let nodes = result.unwrap();
        assert_eq!(nodes.len(), 1);

        assert!(matches!(nodes[0], ContentNode::Br(_)));
    }

    #[test]
    fn test_parse_complex_html() {
        let parser = HtmlContentParser::new();
        let html = r#"<p>Hello <strong>world</strong>!</p><br /><a href="http://example.com">Link</a>"#;
        let result = parser.parse_content(html);

        assert!(result.is_ok());
        let nodes = result.unwrap();
        assert!(nodes.len() >= 3);
    }
}

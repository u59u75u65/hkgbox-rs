//! Content node domain models
//!
//! Represents parsed HTML content in a generic way, independent of any specific parser.

use serde::{Serialize, Deserialize};

/// A node in parsed content
///
/// This enum represents different types of content that can be parsed from HTML.
/// It is generic and can be produced by different parser implementations.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContentNode {
    /// Plain text content
    Text(TextNode),

    /// Image content
    Image(ImageNode),

    /// Block quote content
    BlockQuote(BlockQuoteNode),

    /// Line break
    Br(BrNode),

    /// Hyperlink
    Link(LinkNode),
}

impl ContentNode {
    /// Check if this node is text
    #[must_use]
    pub const fn is_text(&self) -> bool {
        matches!(self, Self::Text(_))
    }

    /// Check if this node is an image
    #[must_use]
    pub const fn is_image(&self) -> bool {
        matches!(self, Self::Image(_))
    }

    /// Check if this node is a link
    #[must_use]
    pub const fn is_link(&self) -> bool {
        matches!(self, Self::Link(_))
    }

    /// Get text content if this is a text node
    #[must_use]
    pub fn as_text(&self) -> Option<&str> {
        if let Self::Text(node) = self {
            Some(&node.data)
        } else {
            None
        }
    }

    /// Get image data if this is an image node
    #[must_use]
    pub fn as_image(&self) -> Option<(&str, &str)> {
        if let Self::Image(node) = self {
            Some((&node.data, &node.alt))
        } else {
            None
        }
    }

    /// Get link data if this is a link node
    #[must_use]
    pub fn as_link(&self) -> Option<(&str, &str)> {
        if let Self::Link(node) = self {
            Some((&node.url, &node.text))
        } else {
            None
        }
    }
}

/// Plain text node
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TextNode {
    /// Text content
    pub data: String,
}

impl TextNode {
    /// Create a new text node
    #[must_use]
    pub fn new(data: String) -> Self {
        Self { data }
    }

    /// Create a text node from a string slice
    #[must_use]
    pub fn from_str(data: &str) -> Self {
        Self {
            data: data.to_string(),
        }
    }
}

/// Image node
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ImageNode {
    /// Image URL
    pub data: String,

    /// Alt text
    pub alt: String,
}

impl ImageNode {
    /// Create a new image node
    #[must_use]
    pub fn new(data: String, alt: String) -> Self {
        Self { data, alt }
    }

    /// Create an image node from string slices
    #[must_use]
    pub fn from_str(data: &str, alt: &str) -> Self {
        Self {
            data: data.to_string(),
            alt: alt.to_string(),
        }
    }
}

/// Block quote node containing nested content
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct BlockQuoteNode {
    /// Nested content nodes
    pub data: Vec<ContentNode>,
}

impl BlockQuoteNode {
    /// Create a new block quote node
    #[must_use]
    pub fn new(data: Vec<ContentNode>) -> Self {
        Self { data }
    }

    /// Create an empty block quote node
    #[must_use]
    pub fn empty() -> Self {
        Self {
            data: Vec::new(),
        }
    }
}

/// Line break node
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct BrNode {}

impl BrNode {
    /// Create a new line break node
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

/// Link node
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct LinkNode {
    /// Link URL
    pub url: String,

    /// Link text
    pub text: String,
}

impl LinkNode {
    /// Create a new link node
    #[must_use]
    pub fn new(url: String, text: String) -> Self {
        Self { url, text }
    }

    /// Create a link node from string slices
    #[must_use]
    pub fn from_str(url: &str, text: &str) -> Self {
        Self {
            url: url.to_string(),
            text: text.to_string(),
        }
    }
}

// ============================================================================
// CONVERSION: From reply_model::NodeType to ContentNode
// ============================================================================

impl From<crate::reply_model::NodeType> for ContentNode {
    fn from(node_type: crate::reply_model::NodeType) -> Self {
        match node_type {
            crate::reply_model::NodeType::Text(text_node) => {
                Self::Text(TextNode { data: text_node.data })
            }
            crate::reply_model::NodeType::Image(image_node) => {
                Self::Image(ImageNode {
                    data: image_node.data,
                    alt: image_node.alt,
                })
            }
            crate::reply_model::NodeType::BlockQuote(blockquote_node) => {
                let data = blockquote_node.data
                    .into_iter()
                    .map(ContentNode::from)
                    .collect();
                Self::BlockQuote(BlockQuoteNode { data })
            }
            crate::reply_model::NodeType::Br(_) => Self::Br(BrNode {}),
            crate::reply_model::NodeType::Link(link_node) => {
                Self::Link(LinkNode {
                    url: link_node.url,
                    text: link_node.text,
                })
            }
        }
    }
}

// ============================================================================
// CONVERSION: From ContentNode to reply_model::NodeType
// ============================================================================

impl From<ContentNode> for crate::reply_model::NodeType {
    fn from(content_node: ContentNode) -> Self {
        match content_node {
            ContentNode::Text(text_node) => {
                Self::Text(crate::reply_model::TextNode { data: text_node.data })
            }
            ContentNode::Image(image_node) => {
                Self::Image(crate::reply_model::ImageNode {
                    data: image_node.data,
                    alt: image_node.alt,
                })
            }
            ContentNode::BlockQuote(blockquote_node) => {
                let data = blockquote_node.data
                    .into_iter()
                    .map(crate::reply_model::NodeType::from)
                    .collect();
                Self::BlockQuote(crate::reply_model::BlockQuoteNode { data })
            }
            ContentNode::Br(_) => Self::Br(crate::reply_model::BrNode {}),
            ContentNode::Link(link_node) => {
                Self::Link(crate::reply_model::LinkNode {
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
    fn test_text_node() {
        let node = TextNode::new("Hello world".to_string());
        assert_eq!(node.data, "Hello world");
    }

    #[test]
    fn test_image_node() {
        let node = ImageNode::new("url.jpg".to_string(), "Alt text".to_string());
        assert_eq!(node.data, "url.jpg");
        assert_eq!(node.alt, "Alt text");
    }

    #[test]
    fn test_blockquote_node() {
        let children = vec![
            ContentNode::Text(TextNode::new("Quote text".to_string())),
        ];
        let node = BlockQuoteNode::new(children);
        assert_eq!(node.data.len(), 1);
    }

    #[test]
    fn test_link_node() {
        let node = LinkNode::new("http://example.com".to_string(), "Click here".to_string());
        assert_eq!(node.url, "http://example.com");
        assert_eq!(node.text, "Click here");
    }

    #[test]
    fn test_content_node_is_text() {
        let node = ContentNode::Text(TextNode::new("Hello".to_string()));
        assert!(node.is_text());
        assert!(!node.is_image());
        assert!(!node.is_link());
    }

    #[test]
    fn test_content_node_as_text() {
        let node = ContentNode::Text(TextNode::new("Hello".to_string()));
        assert_eq!(node.as_text(), Some("Hello"));
        assert_eq!(node.as_image(), None);
        assert_eq!(node.as_link(), None);
    }

    #[test]
    fn test_content_node_is_image() {
        let node = ContentNode::Image(ImageNode::new("url.jpg".to_string(), "Alt".to_string()));
        assert!(!node.is_text());
        assert!(node.is_image());
        assert!(!node.is_link());
    }

    #[test]
    fn test_content_node_as_image() {
        let node = ContentNode::Image(ImageNode::new("url.jpg".to_string(), "Alt".to_string()));
        assert_eq!(node.as_text(), None);
        assert_eq!(node.as_image(), Some(("url.jpg", "Alt")));
        assert_eq!(node.as_link(), None);
    }

    #[test]
    fn test_content_node_is_link() {
        let node = ContentNode::Link(LinkNode::new("http://example.com".to_string(), "Click".to_string()));
        assert!(!node.is_text());
        assert!(!node.is_image());
        assert!(node.is_link());
    }

    #[test]
    fn test_content_node_as_link() {
        let node = ContentNode::Link(LinkNode::new("http://example.com".to_string(), "Click".to_string()));
        assert_eq!(node.as_text(), None);
        assert_eq!(node.as_image(), None);
        assert_eq!(node.as_link(), Some(("http://example.com", "Click")));
    }

    #[test]
    fn test_conversion_from_node_type() {
        let old_node = crate::reply_model::NodeType::Text(
            crate::reply_model::TextNode { data: "Hello".to_string() }
        );
        let new_node: ContentNode = old_node.into();
        assert_eq!(new_node.as_text(), Some("Hello"));
    }

    #[test]
    fn test_conversion_to_node_type() {
        let new_node = ContentNode::Text(TextNode::new("Hello".to_string()));
        let old_node: crate::reply_model::NodeType = new_node.into();
        if let crate::reply_model::NodeType::Text(text_node) = old_node {
            assert_eq!(text_node.data, "Hello");
        } else {
            panic!("Expected Text node");
        }
    }
}

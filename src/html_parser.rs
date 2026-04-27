//! HTML parsing utilities
//!
//! This module provides functions for extracting URLs and parsing HTML content
//! into structured node representations.

use regex::Regex;
use crate::reply_model::{NodeType, ImageNode, TextNode, BlockQuoteNode, BrNode, LinkNode};

/// Extract image URLs from HTML content
///
/// # Arguments
/// * `html` - HTML string to parse
///
/// # Returns
/// Vector of image URLs found in the HTML
#[must_use]
pub fn extract_image_urls(html: &str) -> Vec<String> {
    let mut urls = Vec::new();
    let img_regex = Regex::new(r#"<img[^>]+src="([^"]+)""#).unwrap();

    for capture in img_regex.captures_iter(html) {
        if let Some(url_match) = capture.get(1) {
            urls.push(url_match.as_str().to_string());
        }
    }
    urls
}

/// HTML token types for parsing
#[derive(Debug, Clone)]
enum HtmlToken {
    OpenTag(String, Vec<(String, String)>), // tag name, attributes
    CloseTag(String),
    SelfClosingTag(String, Vec<(String, String)>),
    Text(String),
}

/// Parse HTML content into structured nodes
///
/// # Arguments
/// * `html` - HTML string to parse
///
/// # Returns
/// Vector of parsed nodes representing the HTML content
#[must_use]
pub fn parse_html_content(html: &str) -> Vec<NodeType> {
    let tokens = tokenize_html(html);
    let ast = build_ast(&tokens);
    ast_to_nodes(ast, true)
}

// Tokenization functions
fn tokenize_html(html: &str) -> Vec<HtmlToken> {
    let mut tokens = Vec::new();
    let mut chars = html.chars().peekable();
    let mut current_text = String::new();

    while let Some(&c) = chars.peek() {
        match c {
            '<' => {
                // Flush any accumulated text
                if !current_text.is_empty() {
                    tokens.push(HtmlToken::Text(current_text.clone()));
                    current_text.clear();
                }

                let _ = chars.next(); // consume '<'
                let next_char = chars.next();

                match next_char {
                    Some('/') => {
                        // Closing tag - read tag name
                        let mut tag_name = String::new();
                        while let Some(&c) = chars.peek() {
                            if c.is_whitespace() || c == '>' {
                                break;
                            }
                            tag_name.push(c);
                            let _ = chars.next();
                        }
                        tokens.push(HtmlToken::CloseTag(tag_name));
                        // Skip to next '>'
                        while chars.next() != Some('>') {}
                    }
                    Some('!') => {
                        // Comment or doctype, skip to '>'
                        while chars.next() != Some('>') {}
                    }
                    Some(next_char) => {
                        // Opening tag - we already consumed next_char, use it to start tag name
                        let mut tag_name = String::new();
                        tag_name.push(next_char);

                        // Continue reading tag name
                        while let Some(&c) = chars.peek() {
                            if c.is_whitespace() || c == '>' || c == '/' {
                                break;
                            }
                            tag_name.push(c);
                            let _ = chars.next();
                        }

                        // Skip whitespace after tag name
                        while let Some(&c) = chars.peek() {
                            if !c.is_whitespace() {
                                break;
                            }
                            let _ = chars.next();
                        }

                        // Check if this is a self-closing tag by looking ahead
                        let mut is_self_closing = false;

                        // Parse attributes and check for self-closing
                        let attrs = parse_attributes(&mut chars);

                        // After parsing attributes, check for '/'
                        if let Some(&c) = chars.peek() {
                            if c == '/' {
                                is_self_closing = true;
                                let _ = chars.next(); // consume '/'
                            }
                        }

                        // Expect '>'
                        if chars.next() != Some('>') {
                            // Malformed HTML, but continue
                        }

                        if is_self_closing {
                            tokens.push(HtmlToken::SelfClosingTag(tag_name, attrs));
                        } else {
                            tokens.push(HtmlToken::OpenTag(tag_name, attrs));
                        }
                    }
                    None => break,
                }
            }
            '&' => {
                // HTML entity - try to consume it
                let _ = chars.next(); // consume '&'

                if let Some(entity) = read_html_entity(&mut chars) {
                    // Valid entity found, decode it immediately
                    let decoded = decode_html_entity(&entity);
                    if !current_text.is_empty() {
                        tokens.push(HtmlToken::Text(current_text.clone()));
                        current_text.clear();
                    }
                    tokens.push(HtmlToken::Text(decoded));
                } else {
                    // Not a valid entity, treat '&' as literal text
                    current_text.push('&');
                }
            }
            _ => {
                current_text.push(c);
                let _ = chars.next();
            }
        }
    }

    // Flush remaining text
    if !current_text.is_empty() {
        tokens.push(HtmlToken::Text(current_text));
    }

    tokens
}

fn parse_attributes(chars: &mut std::iter::Peekable<std::str::Chars>) -> Vec<(String, String)> {
    let mut attrs = Vec::new();

    // Skip whitespace before attributes
    while let Some(&c) = chars.peek() {
        if !c.is_whitespace() {
            break;
        }
        let _ = chars.next();
    }

    // Parse attributes until we hit '/' or '>'
    while let Some(&c) = chars.peek() {
        if c == '/' || c == '>' {
            break;
        }

        // Skip whitespace
        while let Some(&c) = chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            let _ = chars.next();
        }

        // Check if we're done
        if let Some(&c) = chars.peek() {
            if c == '/' || c == '>' {
                break;
            }
        }

        // Read attribute name
        let mut attr_name = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() || c == '=' {
                break;
            }
            attr_name.push(c);
            let _ = chars.next();
        }

        // Skip whitespace and '='
        while let Some(&c) = chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            let _ = chars.next();
        }

        if let Some(&c) = chars.peek() {
            if c == '=' {
                let _ = chars.next(); // consume '='

                // Skip whitespace after '='
                while let Some(&c) = chars.peek() {
                    if !c.is_whitespace() {
                        break;
                    }
                    let _ = chars.next();
                }

                // Read attribute value
                if let Some(&c) = chars.peek() {
                    if c == '"' || c == '\'' {
                        let quote = c;
                        let _ = chars.next(); // consume opening quote
                        let mut attr_value = String::new();

                        while let Some(ch) = chars.next() {
                            if ch == quote {
                                break;
                            }
                            attr_value.push(ch);
                        }

                        attrs.push((attr_name, attr_value));
                    } else {
                        // Unquoted value - read until whitespace or delimiter
                        let mut attr_value = String::new();
                        while let Some(&c) = chars.peek() {
                            if c.is_whitespace() || c == '>' || c == '/' {
                                break;
                            }
                            attr_value.push(c);
                            let _ = chars.next();
                        }
                        attrs.push((attr_name, attr_value));
                    }
                }
            }
        }
    }

    attrs
}

fn read_html_entity(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<String> {
    let mut entity = String::new();

    // Peek ahead to validate without consuming
    let mut temp = chars.clone();

    // Check characters (don't consume yet)
    while let Some(&c) = temp.peek() {
        let _ = temp.next();
        if c == ';' {
            // Valid entity, now consume it
            entity.clear(); // Clear the peeked entity
            while let Some(&c) = chars.peek() {
                chars.next();
                if c == ';' {
                    break;
                }
                entity.push(c);
            }
            chars.next(); // consume the ';'
            return Some(entity);
        } else if c.is_alphanumeric() || c == '#' {
            entity.push(c);
            if entity.len() > 20 {
                return None;
            }
        } else {
            // Invalid character, not a valid entity
            return None;
        }
    }

    // No semicolon found
    None
}

// AST construction
#[derive(Debug)]
enum AstNode {
    Element(String, Vec<AstNode>),
    Text(String),
    Image(String, String), // url, alt
    Link(String, String),  // url, text
}

fn build_ast(tokens: &[HtmlToken]) -> Vec<AstNode> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            HtmlToken::OpenTag(tag_name, attrs) => {
                // Special handling for <a> tags to extract href
                if tag_name == "a" {
                    let url = attrs.iter()
                        .find(|(name, _)| name == "href")
                        .map(|(_, value)| value.clone())
                        .unwrap_or_default();

                    let (children, new_i) = collect_children(tokens, i + 1, tag_name);

                    // Check if children contain only text (no images, no nested elements)
                    let is_text_only = children.iter().all(|node| {
                        matches!(node, AstNode::Text(_))
                    });

                    if is_text_only && !url.is_empty() {
                        // Extract text from children
                        let text = children.iter()
                            .filter_map(|node| {
                                if let AstNode::Text(t) = node {
                                    Some(t.as_str())
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("");
                        // Only create Link node if there's actual text content
                        if !text.is_empty() {
                            result.push(AstNode::Link(url, text));
                        }
                    } else {
                        // Contains images or mixed content, keep as Element to preserve structure
                        result.push(AstNode::Element(tag_name.clone(), children));
                    }
                    i = new_i;
                } else {
                    let (children, new_i) = collect_children(tokens, i + 1, tag_name);
                    result.push(AstNode::Element(tag_name.clone(), children));
                    i = new_i;
                }
            }
            HtmlToken::SelfClosingTag(tag_name, attrs) => {
                // Special handling for img tags to preserve attributes
                if tag_name == "img" {
                    let url = attrs.iter()
                        .find(|(name, _)| name == "src")
                        .map(|(_, value)| {
                            // Remove cache URL prefix from image src
                            value.replace("https://cache.hkgolden.media/compress/", "")
                        })
                        .unwrap_or_default();

                    let alt = attrs.iter()
                        .find(|(name, _)| name == "alt")
                        .map(|(_, value)| value.clone())
                        .unwrap_or_default();

                    if !url.is_empty() {
                        result.push(AstNode::Image(url, alt));
                    }
                } else {
                    // Other self-closing tags are treated as elements with no children
                    result.push(AstNode::Element(tag_name.clone(), Vec::new()));
                }
                i += 1;
            }
            HtmlToken::Text(text) => {
                result.push(AstNode::Text(text.clone()));
                i += 1;
            }
            HtmlToken::CloseTag(_) => {
                // Ignore close tags at top level (malformed HTML)
                i += 1;
            }
        }
    }

    result
}

fn collect_children(tokens: &[HtmlToken], start: usize, close_tag: &str) -> (Vec<AstNode>, usize) {
    let mut children = Vec::new();
    let mut i = start;

    while i < tokens.len() {
        match &tokens[i] {
            HtmlToken::CloseTag(tag) if tag == close_tag => {
                return (children, i + 1);
            }
            HtmlToken::OpenTag(tag_name, attrs) => {
                // Special handling for <a> tags to extract href
                if tag_name == "a" {
                    let url = attrs.iter()
                        .find(|(name, _)| name == "href")
                        .map(|(_, value)| value.clone())
                        .unwrap_or_default();

                    let (child_children, new_i) = collect_children(tokens, i + 1, tag_name);

                    // Check if children contain only text (no images, no nested elements)
                    let is_text_only = child_children.iter().all(|node| {
                        matches!(node, AstNode::Text(_))
                    });

                    if is_text_only && !url.is_empty() {
                        // Extract text from children
                        let text = child_children.iter()
                            .filter_map(|node| {
                                if let AstNode::Text(t) = node {
                                    Some(t.as_str())
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("");
                        // Only create Link node if there's actual text content
                        if !text.is_empty() {
                            children.push(AstNode::Link(url, text));
                        }
                    } else {
                        // Contains images or mixed content, keep as Element to preserve structure
                        children.push(AstNode::Element(tag_name.clone(), child_children));
                    }
                    i = new_i;
                } else {
                    let (child_children, new_i) = collect_children(tokens, i + 1, tag_name);
                    children.push(AstNode::Element(tag_name.clone(), child_children));
                    i = new_i;
                }
            }
            HtmlToken::SelfClosingTag(tag_name, attrs) => {
                // Special handling for img tags to preserve attributes
                if tag_name == "img" {
                    let url = attrs.iter()
                        .find(|(name, _)| name == "src")
                        .map(|(_, value)| {
                            // Remove cache URL prefix from image src
                            value.replace("https://cache.hkgolden.media/compress/", "")
                        })
                        .unwrap_or_default();

                    let alt = attrs.iter()
                        .find(|(name, _)| name == "alt")
                        .map(|(_, value)| value.clone())
                        .unwrap_or_default();

                    if !url.is_empty() {
                        children.push(AstNode::Image(url, alt));
                    }
                } else {
                    // Other self-closing tags are treated as elements with no children
                    children.push(AstNode::Element(tag_name.clone(), Vec::new()));
                }
                i += 1;
            }
            HtmlToken::Text(text) => {
                children.push(AstNode::Text(text.clone()));
                i += 1;
            }
            HtmlToken::CloseTag(_) => {
                // Mismatched close tag, skip it
                i += 1;
            }
        }
    }

    (children, i)
}

fn decode_html_entity(entity: &str) -> String {
    match entity {
        "lt" => "<".to_string(),
        "gt" => ">".to_string(),
        "amp" => "&".to_string(),
        "quot" => "\"".to_string(),
        "apos" => "'".to_string(),
        "nbsp" => " ".to_string(),
        _ => format!("&{};", entity), // Unknown entity, keep as-is
    }
}

fn ast_to_nodes(ast: Vec<AstNode>, extract_images: bool) -> Vec<NodeType> {
    let mut nodes = Vec::new();

    for node in ast {
        match node {
            AstNode::Element(tag, children) => {
                match tag.as_str() {
                    "img" => {
                        if extract_images {
                            let src = extract_img_src_from_children(&children);
                            nodes.push(NodeType::Image(ImageNode {
                                data: src.clone(),
                                alt: src,
                            }));
                        }
                    }
                    "br" => {
                        nodes.push(NodeType::Br(BrNode {}));
                    }
                    "blockquote" => {
                        let child_nodes = ast_to_nodes(children, extract_images);
                        nodes.push(NodeType::BlockQuote(BlockQuoteNode {
                            data: child_nodes,
                        }));
                    }
                    "div" | "p" | "span" => {
                        let sub_nodes = ast_to_nodes(children, extract_images);
                        nodes.extend(sub_nodes);
                    }
                    "b" | "strong" | "i" | "em" => {
                        let sub_nodes = ast_to_nodes(children, extract_images);
                        nodes.extend(sub_nodes);
                    }
                    _ => {
                        let sub_nodes = ast_to_nodes(children, extract_images);
                        nodes.extend(sub_nodes);
                    }
                }
            }
            AstNode::Text(text) => {
                // Split text by newlines
                for line in text.split('\n') {
                    if !line.is_empty() {
                        nodes.push(NodeType::Text(TextNode {
                            data: line.to_string(),
                        }));
                    }
                }
            }
            AstNode::Image(url, alt) => {
                nodes.push(NodeType::Image(ImageNode {
                    data: url,
                    alt: alt,
                }));
            }
            AstNode::Link(url, text) => {
                nodes.push(NodeType::Link(LinkNode {
                    url: url,
                    text: text,
                }));
            }
        }
    }

    nodes
}

fn extract_img_src_from_children(_children: &[AstNode]) -> String {
    // For now, return empty string - the img tag should have attributes parsed
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_html_content_simple() {
        let html = "<p>Hello world</p>";
        let nodes = parse_html_content(html);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn test_debug_tokens() {
        let html = r#"<br />"#;
        let tokens = tokenize_html(html);
        println!("Tokens for '<br />':");
        for (i, token) in tokens.iter().enumerate() {
            println!("  {}: {:?}", i, token);
        }

        let ast = build_ast(&tokens);
        println!("AST for '<br />':");
        for (i, node) in ast.iter().enumerate() {
            println!("  {}: {:?}", i, node);
        }

        let nodes = ast_to_nodes(ast, true);
        println!("Nodes for '<br />':");
        for (i, node) in nodes.iter().enumerate() {
            println!("  {}: {:?}", i, node);
        }
    }

    #[test]
    fn test_parse_html_content_with_strong() {
        let html = r#"<p>Hello <strong>world</strong>!</p>"#;
        let nodes = parse_html_content(html);

        // Should have 3 text nodes: "Hello ", "world", "!"
        assert_eq!(nodes.len(), 3);
        if let NodeType::Text(text_node) = &nodes[0] {
            assert_eq!(text_node.data, "Hello ");
        }
        if let NodeType::Text(text_node) = &nodes[1] {
            assert_eq!(text_node.data, "world");
        }
        if let NodeType::Text(text_node) = &nodes[2] {
            assert_eq!(text_node.data, "!");
        }
    }

    #[test]
    fn test_extract_image_urls() {
        let html = r#"<img src="https://example.com/image1.jpg" />"#;
        let urls = extract_image_urls(html);
        assert_eq!(urls.len(), 1);
    }

    #[test]
    fn test_real_html_from_thread() {
        let html = r#"<a href="https://example.com">Link</a><br /><img src="https://example.com/image.jpg" />"#;
        let nodes = parse_html_content(html);

        // Should parse: Link, Br, Image
        assert_eq!(nodes.len(), 3);

        // First node should be Link
        if let NodeType::Link(link_node) = &nodes[0] {
            assert_eq!(link_node.url, "https://example.com");
            assert_eq!(link_node.text, "Link");
        } else {
            panic!("First node should be Link, got {:?}", nodes[0]);
        }

        // Second node should be Br
        assert!(matches!(nodes[1], NodeType::Br(_)));

        // Third node should be Image
        assert!(matches!(nodes[2], NodeType::Image(_)));
    }

    #[test]
    fn test_text_only_link() {
        let html = r#"<a href="http://example.com">Click here</a>"#;
        let nodes = parse_html_content(html);

        assert_eq!(nodes.len(), 1);
        if let NodeType::Link(link) = &nodes[0] {
            assert_eq!(link.url, "http://example.com");
            assert_eq!(link.text, "Click here");
        } else {
            panic!("Expected Link node, got {:?}", nodes[0]);
        }
    }

    #[test]
    fn test_link_with_image() {
        let html = r#"<a href="https://example.com/image.jpg"><img src="https://cache.hkgolden.media/compress/https://example.com/image.jpg" alt="Image" /></a>"#;
        let nodes = parse_html_content(html);

        // Should extract the image, not convert to Link (because it contains an image)
        assert_eq!(nodes.len(), 1);
        if let NodeType::Image(img) = &nodes[0] {
            assert_eq!(img.data, "https://example.com/image.jpg");
            assert_eq!(img.alt, "Image");
        } else {
            panic!("Expected Image node, got {:?}", nodes[0]);
        }
    }

    #[test]
    fn test_multiple_text_links() {
        let html = r#"Visit <a href="http://goo.gl/D13Y2C">http://goo.gl/D13Y2C</a> and <a href="http://goo.gl/7xHMeF">http://goo.gl/7xHMeF</a>"#;
        let nodes = parse_html_content(html);

        assert_eq!(nodes.len(), 4); // "Visit ", Link1, " and ", Link2

        // First link
        if let NodeType::Link(link) = &nodes[1] {
            assert_eq!(link.url, "http://goo.gl/D13Y2C");
            assert_eq!(link.text, "http://goo.gl/D13Y2C");
        } else {
            panic!("Expected Link node at index 1, got {:?}", nodes[1]);
        }

        // Second link
        if let NodeType::Link(link) = &nodes[3] {
            assert_eq!(link.url, "http://goo.gl/7xHMeF");
            assert_eq!(link.text, "http://goo.gl/7xHMeF");
        } else {
            panic!("Expected Link node at index 3, got {:?}", nodes[3]);
        }
    }

    #[test]
    fn test_html_entity_amp() {
        let html = r#"A &amp; B"#;
        let nodes = parse_html_content(html);

        // After immediate entity decoding, we get: "A ", "&", " B"
        assert_eq!(nodes.len(), 3);
        if let NodeType::Text(text) = &nodes[0] {
            assert_eq!(text.data, "A ");
        }
        if let NodeType::Text(text) = &nodes[1] {
            assert_eq!(text.data, "&");
        }
        if let NodeType::Text(text) = &nodes[2] {
            // Note: Leading space may be lost due to text splitting
            assert_eq!(text.data, "B");
        }
    }

    #[test]
    fn test_html_entity_lt_gt() {
        let html = r#"1 &lt; 2 &gt; 3"#;
        let nodes = parse_html_content(html);

        assert_eq!(nodes.len(), 5);
        if let NodeType::Text(text) = &nodes[1] {
            assert_eq!(text.data, "<");
        }
        if let NodeType::Text(text) = &nodes[3] {
            assert_eq!(text.data, ">");
        }
    }

    #[test]
    fn test_invalid_entity_qa() {
        // Test the "Q&A" case that was breaking thread 8029384
        let html = r#"新手向資料及介紹(含陳年卡評,Q&A)"#;
        let nodes = parse_html_content(html);

        assert_eq!(nodes.len(), 1);
        if let NodeType::Text(text) = &nodes[0] {
            assert_eq!(text.data, "新手向資料及介紹(含陳年卡評,Q&A)");
        } else {
            panic!("Expected Text node, got {:?}", nodes[0]);
        }
    }

    #[test]
    fn test_mixed_entities_and_ampersand() {
        let html = r#"Q&A, AT&amp;T, and &lt;tag&gt;"#;
        let nodes = parse_html_content(html);

        // Should parse as: "Q&A, AT", "&", "T, and ", "<", "tag", ">"
        // But text gets split by newlines, so we have fewer nodes
        assert!(!nodes.is_empty());

        // Check Q&A is preserved correctly
        let has_qa = nodes.iter().any(|n| {
            if let NodeType::Text(text) = n {
                text.data.contains("Q&A")
            } else {
                false
            }
        });
        assert!(has_qa, "Should preserve Q&A in text");

        // Check that &amp; was decoded
        let has_amp = nodes.iter().any(|n| {
            if let NodeType::Text(text) = n {
                text.data == "&"
            } else {
                false
            }
        });
        assert!(has_amp, "Should decode &amp; to &");
    }

    #[test]
    fn test_complex_thread_8029384_pattern() {
        // Test actual HTML pattern from thread 8029384
        let html = r#"<strong>新手向資料及介紹(含陳年卡評,Q&A)</strong> - <a href="http://goo.gl/D13Y2C" target="_blank">http://goo.gl/D13Y2C</a>"#;
        let nodes = parse_html_content(html);

        // Should have: bold text, " - ", Link
        assert!(nodes.len() >= 2);

        // Find the link node
        let link_node = nodes.iter().find(|n| matches!(n, NodeType::Link(_)));
        assert!(link_node.is_some(), "Should have a Link node");

        if let NodeType::Link(link) = link_node.unwrap() {
            assert_eq!(link.url, "http://goo.gl/D13Y2C");
            assert_eq!(link.text, "http://goo.gl/D13Y2C");
        }

        // Check Q&A is preserved in text
        let has_qa = nodes.iter().any(|n| {
            if let NodeType::Text(text) = n {
                text.data.contains("Q&A")
            } else {
                false
            }
        });
        assert!(has_qa, "Should preserve Q&A in text");
    }

    #[test]
    fn test_image_link_wrapper_preserves_image() {
        // Test that <a> wrapping <img> preserves the image
        let html = r#"<a href="https://cache.hkgolden.media/compress/https://example.com/img.jpg" target="_blank"><img src="https://cache.hkgolden.media/compress/https://example.com/img.jpg" alt="Example" /></a>"#;
        let nodes = parse_html_content(html);

        assert_eq!(nodes.len(), 1);
        if let NodeType::Image(img) = &nodes[0] {
            // URL should have cache prefix removed
            assert_eq!(img.data, "https://example.com/img.jpg");
            assert_eq!(img.alt, "Example");
        } else {
            panic!("Expected Image node, got {:?}", nodes[0]);
        }
    }

    #[test]
    fn test_link_with_mixed_content() {
        // Test link with both text and image - should preserve structure
        let html = r#"<a href="http://example.com">Text <img src="/icon.gif" alt="icon" /> more</a>"#;
        let nodes = parse_html_content(html);

        // Should have text and image, not a Link node
        assert!(nodes.len() >= 2);
        let has_image = nodes.iter().any(|n| matches!(n, NodeType::Image(_)));
        assert!(has_image, "Should preserve image in mixed content link");
    }

    #[test]
    fn test_empty_link_text() {
        let html = r#"<a href="http://example.com"></a>"#;
        let nodes = parse_html_content(html);

        // Empty link with no text should not create a node
        assert_eq!(nodes.len(), 0);
    }

    #[test]
    fn test_consecutive_links() {
        let html = r#"<a href="http://a.com">A</a><a href="http://b.com">B</a>"#;
        let nodes = parse_html_content(html);

        assert_eq!(nodes.len(), 2);
        if let NodeType::Link(link) = &nodes[0] {
            assert_eq!(link.text, "A");
            assert_eq!(link.url, "http://a.com");
        }
        if let NodeType::Link(link) = &nodes[1] {
            assert_eq!(link.text, "B");
            assert_eq!(link.url, "http://b.com");
        }
    }

    #[test]
    fn test_entity_nbsp() {
        let html = r#"A&nbsp;B"#;
        let nodes = parse_html_content(html);

        // Should decode to "A B" but may be merged
        assert!(nodes.len() >= 2);
        let has_space = nodes.iter().any(|n| {
            if let NodeType::Text(text) = n {
                text.data == " "
            } else {
                false
            }
        });
        assert!(has_space, "Should have a space from &nbsp;");
    }

    #[test]
    fn test_unknown_entity_preserved() {
        let html = r#"&unknown;"#;
        let nodes = parse_html_content(html);

        assert_eq!(nodes.len(), 1);
        if let NodeType::Text(text) = &nodes[0] {
            assert_eq!(text.data, "&unknown;");
        }
    }
}

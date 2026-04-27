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
    Entity(String),
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
                    // Valid entity found
                    if !current_text.is_empty() {
                        tokens.push(HtmlToken::Text(current_text.clone()));
                        current_text.clear();
                    }
                    tokens.push(HtmlToken::Entity(entity));
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
    let mut has_semicolon = false;

    // Peek ahead first to check if there's a semicolon
    let mut temp = chars.clone();
    let _ = temp.next(); // Skip '&'

    while let Some(&c) = temp.peek() {
        let _ = temp.next();
        if c == ';' {
            has_semicolon = true;
            break;
        }
        entity.push(c);
        if entity.len() > 20 {
            // Entity too long, probably not valid
            return None;
        }
    }

    if !has_semicolon {
        return None;
    }

    // Now actually consume the characters
    while let Some(&c) = chars.peek() {
        chars.next();
        if c == ';' {
            break;
        }
    }

    Some(entity)
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
                        result.push(AstNode::Link(url, text));
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
            HtmlToken::Entity(entity) => {
                let decoded = decode_html_entity(entity);
                result.push(AstNode::Text(decoded));
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
                        children.push(AstNode::Link(url, text));
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
            HtmlToken::Entity(entity) => {
                let decoded = decode_html_entity(entity);
                children.push(AstNode::Text(decoded));
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

        // Should parse: Link text, Br, Image
        assert_eq!(nodes.len(), 3);

        // First node should be the link text "Link"
        if let NodeType::Text(text_node) = &nodes[0] {
            assert_eq!(text_node.data, "Link");
        }

        // Second node should be Br
        assert!(matches!(nodes[1], NodeType::Br(_)));

        // Third node should be Image
        assert!(matches!(nodes[2], NodeType::Image(_)));
    }
}

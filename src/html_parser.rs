//! HTML parsing utilities
//!
//! This module provides functions for extracting URLs and parsing HTML content
//! into structured node representations.

use regex::Regex;
use crate::reply_model::{NodeType, ImageNode, TextNode, BlockQuoteNode, BrNode};

/// Extract image URLs from HTML content
///
/// # Arguments
/// * `html` - HTML string to parse
///
/// # Returns
/// Vector of image URLs found in the HTML
///
/// # Examples
/// ```
/// use hkg::html_parser;
///
/// let html = r#"<img src="https://example.com/image.jpg" />"#;
/// let urls = html_parser::extract_image_urls(html);
/// assert_eq!(urls.len(), 1);
/// ```
#[must_use]
pub fn extract_image_urls(html: &str) -> Vec<String> {
    let mut urls = Vec::new();

    // Pattern to match <img src="..." />
    let img_regex = Regex::new(r#"<img[^>]+src="([^"]+)""#).unwrap();

    for capture in img_regex.captures_iter(html) {
        if let Some(url_match) = capture.get(1) {
            let url = url_match.as_str().to_string();
            urls.push(url);
        }
    }

    urls
}

/// HTML token types for parsing
#[derive(Debug, Clone)]
enum HtmlToken {
    OpenTag(String),
    CloseTag(String),
    SelfClosingTag(String, Vec<(String, String)>), // tag name, attributes
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
///
/// # Examples
/// ```
/// use hkg::html_parser;
///
/// let html = "<p>Hello world</p>";
/// let nodes = html_parser::parse_html_content(html);
/// # // nodes will contain TextNode with "Hello world"
/// ```
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

                chars.next(); // consume '<'
                match chars.next() {
                    Some('/') => {
                        // Closing tag
                        let tag_name = read_tag_name(&mut chars);
                        tokens.push(HtmlToken::CloseTag(tag_name));
                    }
                    Some('!') => {
                        // Comment or DOCTYPE - skip until '>'
                        while let Some(&c) = chars.peek() {
                            chars.next();
                            if c == '>' {
                                break;
                            }
                        }
                    }
                    Some('?') => {
                        // XML processing instruction - skip until '?>'
                        let mut prev = '\0';
                        while let Some(&c) = chars.peek() {
                            chars.next();
                            if c == '>' && prev == '?' {
                                break;
                            }
                            prev = c;
                        }
                    }
                    _ => {
                        // Opening tag or self-closing tag
                        let tag_content = read_until(&mut chars, '>');
                        let tag_content = tag_content.trim();

                        if tag_content.ends_with('/') {
                            // Self-closing tag
                            let (tag_name, attributes) = parse_tag_content(&tag_content[..tag_content.len() - 1]);
                            tokens.push(HtmlToken::SelfClosingTag(tag_name, attributes));
                        } else {
                            // Opening tag
                            let (tag_name, attributes) = parse_tag_content(tag_content);
                            tokens.push(HtmlToken::OpenTag(tag_name));
                        }
                    }
                }
            }
            '&' => {
                // HTML entity
                if !current_text.is_empty() {
                    tokens.push(HtmlToken::Text(current_text.clone()));
                    current_text.clear();
                }

                chars.next(); // consume '&'
                let entity = read_html_entity(&mut chars);
                tokens.push(HtmlToken::Entity(entity));
            }
            _ => {
                // Regular text
                chars.next();
                current_text.push(c);
            }
        }
    }

    // Flush remaining text
    if !current_text.is_empty() {
        tokens.push(HtmlToken::Text(current_text));
    }

    tokens
}

fn read_tag_name(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut tag_name = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() || c == '>' {
            break;
        }
        chars.next();
        tag_name.push(c);
    }

    tag_name.to_lowercase()
}

fn read_until(chars: &mut std::iter::Peekable<std::str::Chars>, delimiter: char) -> String {
    let mut result = String::new();

    while let Some(&c) = chars.peek() {
        chars.next();
        if c == delimiter {
            break;
        }
        result.push(c);
    }

    result
}

fn parse_tag_content(content: &str) -> (String, Vec<(String, String)>) {
    let parts: Vec<&str> = content.split_whitespace().collect();
    let tag_name = parts.first().map_or(String::new(), |s| s.to_lowercase());

    let mut attributes = Vec::new();
    if parts.len() > 1 {
        for part in &parts[1..] {
            if let Some((key, value)) = part.split_once('=') {
                let value = value.trim_matches('"').trim_matches('\'').to_string();
                attributes.push((key.to_lowercase(), value));
            }
        }
    }

    (tag_name, attributes)
}

fn read_html_entity(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut entity = String::new();

    while let Some(&c) = chars.peek() {
        chars.next();
        if c == ';' {
            break;
        }
        entity.push(c);
    }

    entity
}

// AST construction
#[derive(Debug)]
enum AstNode {
    Element(String, Vec<AstNode>),
    Text(String),
}

fn build_ast(tokens: &[HtmlToken]) -> Vec<AstNode> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            HtmlToken::OpenTag(tag_name) => {
                let (children, new_i) = collect_children(tokens, i + 1, tag_name);
                result.push(AstNode::Element(tag_name.clone(), children));
                i = new_i;
            }
            HtmlToken::Text(text) => {
                result.push(AstNode::Text(text.clone()));
                i += 1;
            }
            HtmlToken::Entity(entity) => {
                // Convert basic HTML entities
                let decoded = decode_html_entity(entity);
                result.push(AstNode::Text(decoded));
                i += 1;
            }
            _ => {
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
            HtmlToken::OpenTag(tag_name) => {
                let (child_children, new_i) = collect_children(tokens, i + 1, tag_name);
                children.push(AstNode::Element(tag_name.clone(), child_children));
                i = new_i;
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
            _ => {
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
                            // Extract image src attribute
                            let src = extract_img_src(&children);
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
                    _ => {
                        // Recursively process other elements
                        let child_nodes = ast_to_nodes(children, extract_images);
                        nodes.extend(child_nodes);
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
        }
    }

    nodes
}

fn extract_img_src(_children: &[AstNode]) -> String {
    // In a real implementation, you'd extract the src attribute
    // For now, return a placeholder
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_image_urls() {
        let html = r#"<img src="https://example.com/image1.jpg" /><img src="https://example.com/image2.png">"#;
        let urls = extract_image_urls(html);
        assert_eq!(urls.len(), 2);
        assert_eq!(urls[0], "https://example.com/image1.jpg");
        assert_eq!(urls[1], "https://example.com/image2.png");
    }

    #[test]
    fn test_extract_image_urls_no_images() {
        let html = "<p>No images here</p>";
        let urls = extract_image_urls(html);
        assert!(urls.is_empty());
    }

    #[test]
    fn test_parse_html_content() {
        let html = "<p>Hello world</p>";
        let nodes = parse_html_content(html);
        assert!(!nodes.is_empty());
    }
}

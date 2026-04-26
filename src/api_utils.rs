use std::io::Read;

use base64::{Engine as _, engine::general_purpose};
use crate::reply_model::{NodeType, ImageNode, TextNode, BlockQuoteNode, BrNode};

pub fn imgcat_from_data(data: &[u8], width: usize) -> String {
    let encoded = general_purpose::STANDARD.encode(data);
    format!("\x1b]1337;File=inline=1;width={};content:{};\x07", width, encoded)
}

pub fn imgcat_from_url(url: &str, width: usize) -> Result<String, Box<dyn std::error::Error>> {
    use log::{info, error};
    use base64::{Engine as _, engine::general_purpose};

    info!("imgcat_from_url: Fetching image from URL: {}", url);

    // Generate cache filename from URL
    let cache_key = general_purpose::URL_SAFE.encode(url.as_bytes());
    let cache_path = format!("data/cache/img/{}", cache_key);

    // Check if image is already cached
    if let Some(cached_image) = display_from_cache(&cache_path) {
        info!("imgcat_from_url: Using cached image: {}", cache_path);
        return Ok(cached_image);
    }

    // Fetch the image
    info!("imgcat_from_url: Fetching from network...");
    let response = reqwest::blocking::get(url)?;

    if !response.status().is_success() {
        error!("imgcat_from_url: HTTP error {}", response.status());
        return Err(format!("HTTP {}", response.status()).into());
    }

    let image_data = response.bytes()?.to_vec();
    info!("imgcat_from_url: Downloaded {} bytes", image_data.len());

    // Save to cache
    if let Some(parent) = std::path::Path::new(&cache_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(&cache_path, &image_data)?;
    info!("imgcat_from_url: Cached to {}", cache_path);

    // Display using imgcat
    Ok(imgcat_from_data(&image_data, width))
}

fn display_from_cache(path: &str) -> Option<String> {
    use log::info;
    info!("display_from_cache: Loading cached image from {}", path);

    let mut file = std::fs::File::open(path).ok()?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).ok()?;

    info!("display_from_cache: Loaded {} bytes from cache", buffer.len());
    Some(imgcat_from_data(&buffer, 80))
}

pub fn extract_image_urls(html: &str) -> Vec<String> {
    use regex::Regex;

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

#[derive(Debug, Clone)]
enum HtmlToken {
    OpenTag(String),
    CloseTag(String),
    SelfClosingTag(String, Vec<(String, String)>), // tag name, attributes
    Text(String),
    Entity(String),
}

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
                        if chars.next() != Some('>') {
                            // Malformed HTML, skip to next '>'
                        }
                    }
                    Some('!') => {
                        // Comment or doctype, skip to '>'
                        while chars.next() != Some('>') && chars.next() != Some('>') {}
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
                            // Malformed HTML
                        }

                        if is_self_closing {
                            tokens.push(HtmlToken::SelfClosingTag(tag_name, attrs));
                        } else {
                            tokens.push(HtmlToken::OpenTag(tag_name));
                        }
                    }
                    None => break,
                }
            }
            '&' => {
                // HTML entity
                let _ = chars.next(); // consume '&'
                let entity = read_html_entity(&mut chars);
                if !current_text.is_empty() {
                    tokens.push(HtmlToken::Text(current_text.clone()));
                    current_text.clear();
                }
                tokens.push(HtmlToken::Entity(entity));
            }
            _ => {
                current_text.push(c);
                let _ = chars.next();
            }
        }
    }

    // Don't forget the last text
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
            if !c.is_whitespace() && c != '=' {
                break;
            }
            let _ = chars.next();
        }

        // Read attribute value
        let mut attr_value = String::new();
        if let Some(&c) = chars.peek() {
            if c == '"' || c == '\'' {
                let quote = c;
                let _ = chars.next(); // consume opening quote
                while let Some(&c) = chars.peek() {
                    let _ = chars.next();
                    if c == quote {
                        break;
                    }
                    attr_value.push(c);
                }
            } else {
                // Unquoted value
                while let Some(&c) = chars.peek() {
                    if c.is_whitespace() || c == '/' || c == '>' {
                        break;
                    }
                    attr_value.push(c);
                    let _ = chars.next();
                }
            }
        }

        if !attr_name.is_empty() {
            attrs.push((attr_name, attr_value));
        }
    }

    attrs
}

fn read_html_entity(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut entity = String::new();
    entity.push('&');

    while let Some(&c) = chars.peek() {
        entity.push(c);
        let _ = chars.next();
        if c == ';' {
            break;
        }
    }
    entity
}

/// AST node representing HTML structure
#[derive(Debug, Clone)]
enum AstNode {
    Text(String),
    Element { tag: String, children: Vec<AstNode> },
    SelfClosing { tag: String, attrs: Vec<(String, String)> },
}

/// Build AST from tokens with proper nesting
fn build_ast(tokens: &[HtmlToken]) -> Vec<AstNode> {
    let mut root = Vec::new();
    let mut stack: Vec<(String, Vec<AstNode>)> = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            HtmlToken::Text(text) => {
                if !text.trim().is_empty() && text.trim() != "&nbsp;" {
                    let node = AstNode::Text(text.trim().to_string());
                    if let Some((_, ref mut children)) = stack.last_mut() {
                        children.push(node);
                    } else {
                        root.push(node);
                    }
                }
                i += 1;
            }
            HtmlToken::OpenTag(tag) => {
                stack.push((tag.clone(), Vec::new()));
                i += 1;
            }
            HtmlToken::CloseTag(_close_tag) => {
                if let Some((open_tag, children)) = stack.pop() {
                    let node = AstNode::Element { tag: open_tag, children };
                    if let Some((_, ref mut parent_children)) = stack.last_mut() {
                        parent_children.push(node);
                    } else {
                        root.push(node);
                    }
                }
                i += 1;
            }
            HtmlToken::SelfClosingTag(tag, attrs) => {
                let node = AstNode::SelfClosing { tag: tag.clone(), attrs: attrs.clone() };
                if let Some((_, ref mut children)) = stack.last_mut() {
                    children.push(node);
                } else {
                    root.push(node);
                }
                i += 1;
            }
            HtmlToken::Entity(entity) => {
                let decoded = match entity.as_str() {
                    "&nbsp;" => " ",
                    "&lt;" => "<",
                    "&gt;" => ">",
                    "&amp;" => "&",
                    "&quot;" => "\"",
                    _ => entity.as_str(),
                };
                let node = AstNode::Text(decoded.to_string());
                if let Some((_, ref mut children)) = stack.last_mut() {
                    children.push(node);
                } else {
                    root.push(node);
                }
                i += 1;
            }
        }
    }

    // Close any remaining open tags
    while let Some((tag, children)) = stack.pop() {
        let node = AstNode::Element { tag, children };
        root.push(node);
    }

    root
}

/// Convert AST to display nodes
fn ast_to_nodes(ast: Vec<AstNode>, extract_images: bool) -> Vec<NodeType> {
    let mut nodes = Vec::new();

    for node in ast {
        match node {
            AstNode::Text(text) => {
                if !text.is_empty() {
                    nodes.push(NodeType::Text(TextNode { data: text }));
                }
            }
            AstNode::Element { tag, children } => {
                match tag.as_str() {
                    "blockquote" => {
                        let sub_nodes = ast_to_nodes(children, false);
                        nodes.push(NodeType::BlockQuote(BlockQuoteNode { data: sub_nodes }));
                    }
                    "br" => {
                        nodes.push(NodeType::Br(BrNode {}));
                    }
                    "div" | "p" | "span" => {
                        // Inline formatting - convert children and add
                        let sub_nodes = ast_to_nodes(children, extract_images);
                        nodes.extend(sub_nodes);
                    }
                    "b" | "strong" => {
                        // Bold text - preserve as text nodes (we can add formatting later)
                        let sub_nodes = ast_to_nodes(children, extract_images);
                        nodes.extend(sub_nodes);
                    }
                    "i" | "em" => {
                        // Italic text - preserve as text nodes
                        let sub_nodes = ast_to_nodes(children, extract_images);
                        nodes.extend(sub_nodes);
                    }
                    "a" => {
                        // Links - extract text content
                        let sub_nodes = ast_to_nodes(children, extract_images);
                        nodes.extend(sub_nodes);
                    }
                    _ => {
                        // Unknown tag - try to extract children
                        let sub_nodes = ast_to_nodes(children, extract_images);
                        nodes.extend(sub_nodes);
                    }
                }
            }
            AstNode::SelfClosing { tag, attrs } => {
                match tag.as_str() {
                    "br" => {
                        nodes.push(NodeType::Br(BrNode {}));
                    }
                    "img" => {
                        // Extract src attribute for image URL
                        let url = attrs.iter()
                            .find(|(name, _)| name == "src")
                            .map(|(_, value)| {
                                // Remove cache URL prefix from image src
                                value.replace("https://cache.hkgolden.media/compress/", "")
                            })
                            .unwrap_or_default();

                        // Extract alt attribute for image type detection
                        let alt = attrs.iter()
                            .find(|(name, _)| name == "alt")
                            .map(|(_, value)| value.clone())
                            .unwrap_or_default();

                        if !url.is_empty() {
                            nodes.push(NodeType::Image(ImageNode {
                                data: url,
                                alt: alt,
                            }));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    nodes
}

/// Parse HTML content to extract nodes (images, text, blockquotes, etc.)
pub fn parse_html_content(html: &str) -> Vec<NodeType> {
    let tokens = tokenize_html(html);
    let ast = build_ast(&tokens);
    ast_to_nodes(ast, true)
}

pub fn timestamp_to_strings(ms: i64) -> (String, String) {
    use time::OffsetDateTime;

    let dt = OffsetDateTime::from_unix_timestamp(ms / 1000)
        .unwrap_or_else(|_| OffsetDateTime::now_utc());

    let date = format!("{:02}/{:02}/{:04}",
        dt.day(), dt.month(), dt.year());

    let time = format!("{:02}:{:02}",
        dt.hour(), dt.minute());

    (date, time)
}

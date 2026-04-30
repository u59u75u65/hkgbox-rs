//! Test link parsing on thread 8029384

use hkg::api_utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Thread 8029384 Links ===");

    // Fetch the thread
    let url = "https://api.hkgolden.com/v1/view/8029384/1?sensormode=Y&hideblock=N";
    let response = reqwest::blocking::get(url)?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()).into());
    }

    let json = response.text()?;

    // Parse the API response
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json) {
        if let Some(data) = parsed.get("data") {
            // Get the main content
            if let Some(content) = data.get("content").and_then(|c| c.as_str()) {
                println!("\n=== Parsing Main Content ===");
                let nodes = api_utils::parse_html_content(content);

                println!("Parsed {} nodes from main content", nodes.len());

                // Count and show links
                let mut link_count = 0;
                for (i, node) in nodes.iter().enumerate() {
                    if let hkg::reply_model::NodeType::Link(link) = node {
                        link_count += 1;
                        println!("Link {}: URL='{}', Text='{}'", link_count, link.url, link.text);
                    }
                }

                println!("\nTotal links found: {}", link_count);

                // Check for any remaining raw HTML in text nodes
                println!("\n=== Checking for raw HTML in text nodes ===");
                for (i, node) in nodes.iter().enumerate() {
                    if let hkg::reply_model::NodeType::Text(text) = node {
                        if text.data.contains("<a href") || text.data.contains("</a>") {
                            println!("Node {} contains raw HTML: {}", i, text.data.chars().take(100).collect::<String>());
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

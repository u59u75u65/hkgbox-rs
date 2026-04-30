//! Debug thread 8029384 to see what's failing

use hkg::api_utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Fetching Thread 8029384 ===");

    // Fetch the thread using API
    let url = format!("https://api.hkgolden.com/v1/view/8029384/1?sensormode=Y&hideblock=N");
    println!("Fetching: {}", url);

    let response = reqwest::blocking::get(&url)?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()).into());
    }

    let json = response.text()?;

    // Parse the API response
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json) {
        if let Some(data) = parsed.get("data") {
            // Get the main content
            if let Some(content) = data.get("content").and_then(|c| c.as_str()) {
                println!("\n=== Main Content ===");
                println!("Content length: {} chars", content.len());
                println!("Content preview (first 500 chars):");
                println!("{}\n", &content.chars().take(500).collect::<String>());

                // Try to parse it
                println!("=== Parsing Main Content ===");
                let nodes = api_utils::parse_html_content(content);

                println!("Parsed {} nodes from main content", nodes.len());
                for (i, node) in nodes.iter().take(10).enumerate() {
                    println!("Node {}: {:?}", i, node);
                }
            }

            // Get the first reply
            if let Some(replies) = data.get("replies").and_then(|r| r.as_array()) {
                if let Some(first_reply) = replies.first() {
                    if let Some(content) = first_reply.get("content").and_then(|c| c.as_str()) {
                        println!("\n=== First Reply Content ===");
                        println!("Content length: {} chars", content.len());
                        println!("Content preview (first 500 chars):");
                        println!("{}\n", &content.chars().take(500).collect::<String>());

                        // Try to parse it
                        println!("=== Parsing First Reply ===");
                        let nodes = api_utils::parse_html_content(content);

                        println!("Parsed {} nodes from first reply", nodes.len());
                        for (i, node) in nodes.iter().take(10).enumerate() {
                            println!("Node {}: {:?}", i, node);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

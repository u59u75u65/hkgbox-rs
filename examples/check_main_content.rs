//! Check full main content of thread 8029384

use hkg::api_utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Checking Main Content of Thread 8029384 ===");

    // Fetch the thread using API
    let url = format!("https://api.hkgolden.com/v1/view/8029384/1?sensormode=Y&hideblock=N");
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
                println!("Full content:\n{}\n", content);
                println!("Content length: {}\n", content.len());

                // Try to parse it
                let nodes = api_utils::parse_html_content(content);

                println!("Parsed {} nodes from main content", nodes.len());
                println!("\n=== All Parsed Nodes ===");
                for (i, node) in nodes.iter().enumerate() {
                    println!("Node {}: {:?}", i, node);
                }
            }
        }
    }

    Ok(())
}

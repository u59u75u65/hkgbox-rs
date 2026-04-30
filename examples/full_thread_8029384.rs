//! Test full thread 8029384 rendering to see what fails

use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Thread 8029384 ===");

    // Fetch using API
    let url = format!("https://api.hkgolden.com/v1/view/8029384/1?sensormode=Y&hideblock=N");
    let response = reqwest::blocking::get(&url)?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()).into());
    }

    let json = response.text()?;

    // Parse the API response
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json) {
        if let Some(data) = parsed.get("data") {
            // Get all replies
            if let Some(replies) = data.get("replies").and_then(|r| r.as_array()) {
                println!("Total replies: {}\n", replies.len());

                for (i, reply) in replies.iter().take(10).enumerate() {
                    if let Some(content) = reply.get("content").and_then(|c| c.as_str()) {
                        if let Some(author) = reply.get("authorName").and_then(|a| a.as_str()) {
                            println!("--- Reply {} by {} ---", i + 1, author);
                            println!("Content: {}", content);
                            println!("Content length: {}\n", content.len());

                            // Check for specific patterns that might fail
                            if content.contains("<img") {
                                println!("Contains <img> tag");
                            }
                            if content.contains("data-icons") {
                                println!("Contains data-icons attribute");
                            }
                            if content.contains("src=\"") {
                                println!("Contains src attribute");
                            }
                            if content.contains("alt=\"") {
                                println!("Contains alt attribute");
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

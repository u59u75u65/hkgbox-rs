// Example: Simple API call using curl
//
// Run with: cargo run --example api_simple

fn main() {
    println!("=== Simple HKGolden API Test ===\n");

    // Fetch topics using curl
    let output = std::process::Command::new("curl")
        .arg("-s")
        .arg("https://api.hkgolden.com/v1/topics/BW/1?thumb=Y&sort=0&limit=5")
        .output()
        .expect("Failed to execute curl");

    if output.status.success() {
        let json_str = String::from_utf8_lossy(&output.stdout);

        // Parse JSON
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_str) {
            if json["result"].as_bool() == Some(true) {
                println!("✅ API Response:\n");

                if let Some(topics) = json["data"]["list"].as_array() {
                    for (i, topic) in topics.iter().enumerate() {
                        let title = topic["title"].as_str().unwrap_or("N/A");
                        let author = topic["authorName"].as_str().unwrap_or("N/A");
                        let replies = topic["totalReplies"].as_i64().unwrap_or(0);

                        println!("{}. {}", i + 1, title);
                        println!("   by {} ({} replies)\n", author, replies);
                    }
                }
            } else {
                println!("❌ API Error: {:?}", json);
            }
        }
    } else {
        println!("❌ Failed to fetch data");
    }
}

// Example: Test HKGolden JSON API
//
// Run with: cargo run --example api_test

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== HKGolden API Test ===\n");

    // Test 1: Fetch topics
    println!("📡 Fetching topics from BW forum...\n");

    let topics_url = "https://api.hkgolden.com/v1/topics/BW/1?thumb=Y&sort=0&sensormode=Y&filtermodeS=N&hideblock=N&limit=-1";
    let topics_response = reqwest::blocking::get(topics_url)?.text()?;

    let topics_json: serde_json::Value = serde_json::from_str(&topics_response)?;

    if topics_json["result"].as_bool() == Some(true) {
        let total = topics_json["data"]["total"].as_i64().unwrap_or(0);
        let max_page = topics_json["data"]["maxPage"].as_i64().unwrap_or(0);

        println!("✅ Got {} topics (max page: {})\n", total, max_page);

        if let Some(topics) = topics_json["data"]["list"].as_array() {
            println!("First 5 topics:\n");

            for (i, topic) in topics.iter().take(5).enumerate() {
                let title = topic["title"].as_str().unwrap_or("N/A");
                let author = topic["authorName"].as_str().unwrap_or("N/A");
                let replies = topic["totalReplies"].as_i64().unwrap_or(0);
                let rating = topic["rating"].as_i64().unwrap_or(0);

                println!("{}. {}", i + 1, title);
                println!("   👤 {}", author);
                println!("   💬 {} replies", replies);
                println!("   ⭐ Rating: {}\n", rating);
            }
        }

        // Get first topic ID
        if let Some(first_topic) = topics_json["data"]["list"].as_array().and_then(|arr| arr.first()) {
            if let Some(thread_id) = first_topic["id"].as_i64() {
                println!("=== Fetching Thread {} ===\n", thread_id);

                let thread_url = format!(
                    "https://api.hkgolden.com/v1/view/{}/1?sensormode=Y&hideblock=N",
                    thread_id
                );

                match reqwest::blocking::get(&thread_url) {
                    Ok(response) => {
                        let thread_text = response.text()?;
                        let thread_json: serde_json::Value = serde_json::from_str(&thread_text)?;

                        if thread_json["result"].as_bool() == Some(true) {
                            let title = thread_json["data"]["title"].as_str().unwrap_or("N/A");
                            let total_replies = thread_json["data"]["totalReplies"].as_i64().unwrap_or(0);

                            println!("📖 {}", title);
                            println!("💬 {} replies total\n", total_replies);

                            if let Some(replies) = thread_json["data"]["replies"].as_array() {
                                println!("First 3 replies:\n");

                                for (i, reply) in replies.iter().take(3).enumerate() {
                                    let author = reply["authorName"].as_str().unwrap_or("N/A");
                                    let content = reply["content"].as_str().unwrap_or("N/A");

                                    // Strip HTML for display
                                    let clean_content = content
                                        .replace("<br />", " ")
                                        .replace("<blockquote>", "Quote: ")
                                        .replace("</blockquote>", "")
                                        .chars()
                                        .take(100)
                                        .collect::<String>();

                                    println!("{}. 👤 {}", i + 1, author);
                                    println!("   {}", clean_content);
                                    println!();
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("❌ Failed to fetch thread: {}\n", e);
                    }
                }
            }
        }
    } else {
        println!("❌ API returned error\n");
    }

    println!("=== Test Complete ===");

    Ok(())
}

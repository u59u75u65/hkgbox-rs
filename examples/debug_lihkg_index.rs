//! Debug LIHKG Index Request
//!
//! This example demonstrates how to successfully fetch LIHKG index/topics
//! by replicating the exact headers discovered through Playwright investigation.
//!
//! Key findings from Playwright inspection:
//! - LIHKG requires specific headers: x-li-device-type, x-li-device, x-li-load-time
//! - These headers simulate a real browser request
//! - Without these headers, Cloudflare blocks the request (Error 1020)
//!
//! Usage:
//!   cargo run --example debug_lihkg_index

use reqwest::blocking::Client;
use serde_json::Value;

/// Generate device ID for LIHKG requests
///
/// Using hardcoded working device ID from Playwright investigation.
fn generate_device_id() -> String {
    // Hardcoded working device ID from Playwright testing
    // This matches the format that successfully bypasses LIHKG rate limiting
    "89ee2a48214807d7762894d2ae9d07500e339171".to_string()
}

/// Calculate page load time
///
/// LIHKG expects a realistic page load time in seconds.
/// Browsers typically send this to indicate how long the page took to load.
fn calculate_load_time() -> f64 {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // Generate realistic load time between 1.0-5.0 seconds
    let fractional = now.fract();
    (fractional * 4.0) + 1.0  // Range: 1.0 to 5.0
}

/// Fetch LIHKG index with proper headers
fn fetch_lihkg_index(cat_id: i32, page: i32) -> Result<Value, Box<dyn std::error::Error>> {
    println!("🔍 Debugging LIHKG Index Request");
    println!("═════════════════════════════════════════\n");

    let device_id = generate_device_id();
    let load_time = calculate_load_time();

    println!("📋 Request Parameters:");
    println!("   Category ID: {} ({})", cat_id, get_category_name(cat_id));
    println!("   Page: {}", page);
    println!("   Count: 60 (max per page)");
    println!("   Type: now (latest threads)\n");

    println!("🔑 Generated Headers:");
    println!("   x-li-device-type: browser");
    println!("   x-li-device: {}", device_id);
    println!("   x-li-load-time: {:.6}", load_time);
    println!();

    let url = format!(
        "https://lihkg.com/api_v2/thread/latest?cat_id={}&page={}&count=60&type=now",
        cat_id, page
    );

    println!("📡 Fetching URL:");
    println!("   {}\n", url);

    let client = Client::new();

    // ⭐ CRITICAL: These headers are discovered through Playwright investigation
    // Without these, LIHKG returns Cloudflare error 1020
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Referer", &format!("https://lihkg.com/category/{}", cat_id))
        .header("x-li-device-type", "browser")
        .header("x-li-device", &device_id)
        .header("x-li-load-time", &load_time.to_string())
        .header("sec-ch-ua", r#""Chromium";v="147", "Not.A/Brand";v="8""#)
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", r#""macOS""#)
        .send()?;

    let status = response.status();
    println!("📊 Response Status: {}\n", status);

    if !status.is_success() {
        let error_text = response.text()?;
        println!("❌ Error Response:");
        println!("   {}\n", error_text);

        if status.as_u16() == 1020 {
            println!("🔒 Cloudflare Protection Detected!");
            println!("   This usually means required headers are missing.");
            println!("   Check that x-li-device-type, x-li-device, and x-li-load-time are set.\n");
        }

        return Err(format!("HTTP {}: {}", status, error_text).into());
    }

    let json: Value = response.json()?;

    // Check LIHKG success flag
    if json["success"].as_i64() != Some(1) {
        println!("❌ LIHKG API Error:");
        println!("   {}\n", json);
        return Err("LIHKG API request failed".into());
    }

    println!("✅ Success! LIHKG API responded successfully.\n");

    Ok(json)
}

/// Get category name from ID
fn get_category_name(cat_id: i32) -> &'static str {
    match cat_id {
        1 => "吹水台",
        2 => "熱門",
        5 => "時事台",
        15 => "財經台",
        _ => "其他分類",
    }
}

/// Display response summary
fn display_response_summary(json: &Value) {
    if let Some(response) = json.get("response") {
        if let Some(items) = response.get("items").and_then(|v| v.as_array()) {
            println!("📦 Response Summary:");
            println!("   Threads received: {}", items.len());
            println!("   Category: {}", response["category"]["name"].as_str().unwrap_or("Unknown"));
            println!("   Pagination: {}", response["is_pagination"].as_bool().unwrap_or(false));
            println!();

            if !items.is_empty() {
                println!("📝 Sample Threads (first 5):\n");

                for (i, item) in items.iter().take(5).enumerate() {
                    let title = item["title"].as_str().unwrap_or("(No title)");
                    let author = item["user_nickname"].as_str().unwrap_or("(Unknown)");
                    let replies = item["no_of_reply"].as_i64().unwrap_or(0);
                    let likes = item["like_count"].as_i64().unwrap_or(0);
                    let dislikes = item["dislike_count"].as_i64().unwrap_or(0);
                    let thread_id = item["thread_id"].as_i64().unwrap_or(0);
                    let total_page = item["total_page"].as_i64().unwrap_or(1);

                    println!("   {}. {}", i + 1, title);
                    println!("      👤 {} | 💬 {} 回覆 | 👍 {} 👎 {} | 📄 {} 頁",
                        author, replies, likes, dislikes, total_page);
                    println!("      🆔 Thread ID: {}", thread_id);
                    println!();
                }

                if items.len() > 5 {
                    println!("   ... and {} more threads\n", items.len() - 5);
                }
            }
        }
    }

    println!("═════════════════════════════════════════");
    println!("✅ LIHKG Index Request Debug Complete!");
}

/// Show comparison: with vs without proper headers
fn show_header_comparison() {
    println!("\n📚 Header Comparison:\n");
    println!("❌ WITHOUT proper headers (Cloudflare blocks):");
    println!("   GET /api_v2/thread/latest?cat_id=1&page=1");
    println!("   User-Agent: reqwest/0.11.0");
    println!("   → Error 1020 (Cloudflare)\n");

    println!("✅ WITH proper headers (discovered via Playwright):");
    println!("   GET /api_v2/thread/latest?cat_id=1&page=1");
    println!("   User-Agent: Mozilla/5.0 ...");
    println!("   Referer: https://lihkg.com/category/1");
    println!("   x-li-device-type: browser");
    println!("   x-li-device: [32-char-hex]");
    println!("   x-li-load-time: 3.141592");
    println!("   → Success 200 with real data! 🎉\n");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 LIHKG Index Request Debug Example\n");

    // Test with category 1 (吹水台)
    let cat_id = 1;
    let page = 1;

    match fetch_lihkg_index(cat_id, page) {
        Ok(json) => {
            display_response_summary(&json);
            show_header_comparison();

            println!("\n💡 Key Takeaways:");
            println!("   1. Playwright investigation revealed critical headers");
            println!("   2. x-li-device-type, x-li-device, x-li-load-time are required");
            println!("   3. Device ID must be a 32-character hex string");
            println!("   4. Load time should be realistic (1-5 seconds)");
            println!("   5. Referer must point to lihkg.com domain");
            println!("\n🚀 Now you can use --service lihkg in the main app!\n");

            Ok(())
        }
        Err(e) => {
            println!("❌ Debug failed: {}\n", e);
            println!("🔧 Troubleshooting:");
            println!("   1. Check internet connection");
            println!("   2. Verify LIHKG website is accessible");
            println!("   3. Try again in a few minutes");
            println!("   4. Check if LIHKG API structure has changed\n");
            Err(e)
        }
    }
}

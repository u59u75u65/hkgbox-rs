//! Simple example to render a single post and test image rendering
//!
//! Usage: cargo run --example render_post

use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== HKGolden Post Renderer ===");
    println!("Fetching thread 8045633...\n");

    // Fetch the thread using API
    let api_response = fetch_thread(8045633, 1)?;

    println!("Thread: {}", api_response.thread.title);
    println!("Author: {}", api_response.thread.author_name);
    println!("Replies: {}\n", api_response.thread.total_replies);

    // Render main content
    println!("=== Main Content ===");
    render_content(&api_response.thread.content)?;

    // Render replies
    println!("\n=== Replies ===");
    for (i, reply) in api_response.replies.iter().take(5).enumerate() {
        println!("\n--- Reply {} ({} @ {}) ---",
            reply.index,
            reply.author_name,
            format_timestamp(reply.reply_date)
        );
        render_content(&reply.content)?;
    }

    Ok(())
}

fn render_content(html: &str) -> Result<(), Box<dyn std::error::Error>> {
    use hkg::api_utils;

    // Parse HTML content to extract nodes
    let nodes = api_utils::parse_html_content(html);

    let mut image_count = 0;

    for node in &nodes {
        match node {
            hkg::reply_model::NodeType::Text(text) => {
                print!("{}", text.data);
                io::stdout().flush()?;
            }
            hkg::reply_model::NodeType::Br(_) => {
                println!();
            }
            hkg::reply_model::NodeType::Image(img) => {
                image_count += 1;
                println!("\n[Image {}]", image_count);
                println!("URL: {}", img.data);

                // Try to render the image using imgcat
                match api_utils::imgcat_from_url(&img.data, 80) {
                    Ok(imgcat_string) => {
                        // Print the imgcat escape sequence
                        print!("{}", imgcat_string);
                        io::stdout().flush()?;
                        println!("\n[Image rendered successfully]");
                    }
                    Err(e) => {
                        println!("[Failed to render image: {}]", e);
                    }
                }
            }
            hkg::reply_model::NodeType::BlockQuote(bq) => {
                println!("\n[Quote]");
                for sub_node in &bq.data {
                    if let hkg::reply_model::NodeType::Text(text) = sub_node {
                        print!("  {}", text.data);
                    }
                }
                println!("\n[End Quote]");
            }
            _ => {}
        }
    }

    println!();
    Ok(())
}

fn fetch_thread(thread_id: i32, page: i32) -> Result<ThreadData, Box<dyn std::error::Error>> {
    use reqwest::blocking::Client;

    let url = format!("https://api.hkgolden.com/v1/view/{}/{}?sensormode=Y&hideblock=N", thread_id, page);
    println!("Fetching: {}", url);

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let response = client.get(&url).send()?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()).into());
    }

    let json = response.text()?;

    // Parse the API response manually for this example
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json) {
        return Ok(parse_thread_response(&parsed));
    }

    Err("Failed to parse response".into())
}

fn parse_thread_response(json: &serde_json::Value) -> ThreadData {
    // Simplified parsing for the example
    let thread = &json["data"];

    ThreadData {
        thread: ThreadInfo {
            id: thread["id"].as_i64().unwrap_or(0) as i32,
            title: thread["title"].as_str().unwrap_or("Unknown").to_string(),
            author_name: thread["authorName"].as_str().unwrap_or("Unknown").to_string(),
            content: thread["content"].as_str().unwrap_or("").to_string(),
            total_replies: thread["totalReplies"].as_i64().unwrap_or(0) as i32,
        },
        replies: parse_replies(&json["data"]["replies"]),
    }
}

fn parse_replies(replies_json: &serde_json::Value) -> Vec<ReplyInfo> {
    let mut replies = Vec::new();

    if let Some(array) = replies_json.as_array() {
        for reply_json in array {
            replies.push(ReplyInfo {
                index: reply_json["index"].as_i64().unwrap_or(0) as i32,
                author_name: reply_json["authorName"].as_str().unwrap_or("Unknown").to_string(),
                content: reply_json["content"].as_str().unwrap_or("").to_string(),
                reply_date: reply_json["replyDate"].as_i64().unwrap_or(0) as i64,
            });
        }
    }

    replies
}

fn format_timestamp(ms: i64) -> String {
    use chrono::{Local, Utc, TimeZone};

    let timestamp = ms / 1000;
    let dt_utc = Utc.timestamp_opt(timestamp, 0).unwrap();
    let dt_local = dt_utc.with_timezone(&Local);

    dt_local.format("%d/%m %H:%M").to_string()
}

#[derive(Debug)]
struct ThreadData {
    thread: ThreadInfo,
    replies: Vec<ReplyInfo>,
}

#[derive(Debug)]
struct ThreadInfo {
    id: i32,
    title: String,
    author_name: String,
    content: String,
    total_replies: i32,
}

#[derive(Debug)]
struct ReplyInfo {
    index: i32,
    author_name: String,
    content: String,
    reply_date: i64,
}

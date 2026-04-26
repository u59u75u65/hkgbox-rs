// Debug parse_html_content for any thread ID
use hkg::api_client::HkgApiClient;
use std::env;
use chrono;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let thread_id = if args.len() > 1 {
        args[1].parse::<i32>()?
    } else {
        8045371
    };

    println!("=== Debug HTML Parsing for Thread {} ===\n", thread_id);

    let client = HkgApiClient::new()?;

    println!("Fetching: https://api.hkgolden.com/v1/view/{}/1?sensormode=Y&hideblock=N\n", thread_id);

    let response = client.fetch_thread(thread_id, 1)?;

    println!("Thread: {}", response.data.title);
    println!("Total replies: {}\n", response.data.replies.len());

    // Show main thread content if exists
    if !response.data.content.is_empty() {
            println!("┌─────────────────────────────────────────────────────────────");
            println!("│ MAIN THREAD CONTENT");
            println!("├─────────────────────────────────────────────────────────────");
            println!("│ Raw HTML ({} chars):", response.data.content.len());
            println!("│ {}", response.data.content);
            println!("├─────────────────────────────────────────────────────────────");
            println!("│ Parsed Content:");

            use hkg::api_utils::parse_html_content;
            let nodes = parse_html_content(&response.data.content);

            if nodes.is_empty() {
                println!("│ (No content)");
            } else {
                for (j, node) in nodes.iter().enumerate() {
                    print!("│   [{}] ", j + 1);
                    match node {
                        hkg::reply_model::NodeType::Image(img) => {
                            println!("📷 IMAGE");
                            println!("│       URL: {}", img.data);
                            println!("│       Alt: {}", img.alt);
                        }
                        hkg::reply_model::NodeType::Text(text) => {
                            println!("📝 TEXT");
                            println!("│       {}", escape_debug(&text.data));
                        }
                        hkg::reply_model::NodeType::BlockQuote(bq) => {
                            println!("💬 BLOCKQUOTE");
                            println!("│       Nested items: {}", bq.data.len());
                            for (k, child) in bq.data.iter().enumerate() {
                                if let hkg::reply_model::NodeType::Text(t) = child {
                                    println!("│         [{}] {}", k + 1, escape_debug(&t.data));
                                }
                            }
                        }
                        hkg::reply_model::NodeType::Br(_) => {
                            println!("🔃 LINE BREAK");
                        }
                    }
                }
            }
            println!("└─────────────────────────────────────────────────────────────");
            println!();
    }

    // Show all replies
    for (i, reply) in response.data.replies.iter().enumerate() {
        println!("┌─────────────────────────────────────────────────────────────");
        println!("│ Reply {} - Author: {}", i + 1, reply.author_name);
        println!("├─────────────────────────────────────────────────────────────");
        println!("│ Raw reply_date: {} ms ({} since epoch)", reply.reply_date, reply.reply_date);
        println!("│ Current time: {} ms ({} since epoch)",
            chrono::Utc::now().timestamp_millis(),
            chrono::Utc::now().timestamp()
        );
        println!("│ Time difference: {} ms", chrono::Utc::now().timestamp_millis() - reply.reply_date);

        use hkg::api_utils::timestamp_to_strings;
        let (date, time) = timestamp_to_strings(reply.reply_date);
        println!("│ Computed local time: {} {}", date, time);

        println!("├─────────────────────────────────────────────────────────────");
        println!("│ Raw HTML ({} chars):", reply.content.len());
        println!("│ {}", reply.content);
        println!("├─────────────────────────────────────────────────────────────");
        println!("│ Parsed Content:");

        use hkg::api_utils::parse_html_content;
        let nodes = parse_html_content(&reply.content);

        if nodes.is_empty() {
            println!("│ (No content)");
        } else {
            for (j, node) in nodes.iter().enumerate() {
                print!("│   [{}] ", j + 1);
                match node {
                    hkg::reply_model::NodeType::Image(img) => {
                        println!("📷 IMAGE");
                        println!("│       URL: {}", img.data);
                        println!("│       Alt: {}", img.alt);
                    }
                    hkg::reply_model::NodeType::Text(text) => {
                        println!("📝 TEXT");
                        println!("│       {}", escape_debug(&text.data));
                    }
                    hkg::reply_model::NodeType::BlockQuote(bq) => {
                        println!("💬 BLOCKQUOTE");
                        println!("│       Nested items: {}", bq.data.len());
                        for (k, child) in bq.data.iter().enumerate() {
                            if let hkg::reply_model::NodeType::Text(t) = child {
                                println!("│         [{}] {}", k + 1, escape_debug(&t.data));
                            }
                        }
                    }
                    hkg::reply_model::NodeType::Br(_) => {
                        println!("🔃 LINE BREAK");
                    }
                }
            }
        }
        println!("└─────────────────────────────────────────────────────────────");
        println!();
    }

    Ok(())
}

fn escape_debug(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            '\t' => "\\t".to_string(),
            c if c.is_control() => format!("\\x{:02x}", c as u8),
            c => c.to_string(),
        })
        .collect::<String>()
}
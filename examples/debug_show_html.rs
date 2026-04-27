// Test to see what actual HTML content we get from show screen API
use hkg::api_client::HkgApiClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Debug Show Screen HTML Content ===\n");

    let client = HkgApiClient::new()?;

    // Test with the thread that was mentioned earlier
    println!("Fetching thread 8045388...");
    let response = client.fetch_thread(8045388, 1)?;

    println!("Thread title: {}", response.data.title);
    println!("Total replies: {}\n", response.data.replies.len());

    // Show first 3 replies with their HTML content
    for (i, reply) in response.data.replies.iter().take(3).enumerate() {
        println!("--- Reply {} (User: {}) ---", i + 1, reply.author_name);
        println!("HTML length: {} chars", reply.content.len());

        // Show first 200 chars of HTML
        let preview = if reply.content.len() > 200 {
            format!("{}...", &reply.content[..200])
        } else {
            reply.content.clone()
        };
        println!("HTML preview:\n{}\n", preview);

        // Parse it
        use hkg::api_utils::parse_html_content;
        let nodes = parse_html_content(&reply.content);

        println!("Parsed {} nodes:", nodes.len());
        for (j, node) in nodes.iter().take(10).enumerate() {
            match node {
                hkg::reply_model::NodeType::Image(img) => {
                    println!("  {}. Image: {}", j + 1, truncate_url(&img.data));
                }
                hkg::reply_model::NodeType::Text(text) => {
                    let preview = if text.data.len() > 50 {
                        format!("{}...", &text.data[..50])
                    } else {
                        text.data.clone()
                    };
                    println!("  {}. Text: '{}'", j + 1, preview);
                }
                hkg::reply_model::NodeType::BlockQuote(_) => {
                    println!("  {}. BlockQuote (nested)", j + 1);
                }
                hkg::reply_model::NodeType::Br(_) => {
                    println!("  {}. LineBreak", j + 1);
                }
                hkg::reply_model::NodeType::Link(link) => {
                    let url_preview = if link.url.len() > 50 {
                        format!("{}...", &link.url[..50])
                    } else {
                        link.url.clone()
                    };
                    println!("  {}. Link: '{}' -> '{}'", j + 1, link.text, url_preview);
                }
            }
        }

        if nodes.len() > 10 {
            println!("  ... and {} more nodes", nodes.len() - 10);
        }

        println!();
    }

    Ok(())
}

fn truncate_url(url: &str) -> String {
    if url.len() > 60 {
        format!("{}...", &url[..60])
    } else {
        url.to_string()
    }
}
//! Test link parsing

use hkg::api_utils;

fn main() {
    // Test HTML with links
    let html = r#"Test <a href="http://goo.gl/D13Y2C" target="_blank">http://goo.gl/D13Y2C</a> and <a href="http://goo.gl/7xHMeF">http://goo.gl/7xHMeF</a> end."#;

    println!("Testing HTML with links:");
    println!("HTML: {}\n", html);

    let nodes = api_utils::parse_html_content(html);

    println!("Parsed {} nodes:", nodes.len());
    for (i, node) in nodes.iter().enumerate() {
        println!("Node {}: {:?}", i, node);
        match node {
            hkg::reply_model::NodeType::Link(link) => {
                println!("  -> URL: '{}', Text: '{}'", link.url, link.text);
            }
            _ => {}
        }
    }
}

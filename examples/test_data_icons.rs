//! Test specific HTML pattern from thread 8029384

use hkg::api_utils;

fn main() {
    // Test HTML with data-icons pattern
    let html = r#"舊po去到1007<img data-icons=":Olm" src="/faces/lomore/oh.gif" alt=":Olm" />"#;

    println!("Testing HTML with data-icons pattern:");
    println!("HTML: {}", html);
    println!();

    let nodes = api_utils::parse_html_content(html);

    println!("Parsed {} nodes:", nodes.len());
    for (i, node) in nodes.iter().enumerate() {
        println!("Node {}: {:?}", i, node);
        match node {
            hkg::reply_model::NodeType::Image(img) => {
                println!("  -> URL: '{}', Alt: '{}'", img.data, img.alt);
            }
            _ => {}
        }
    }

    println!();
    println!("Expected: Image with URL='/faces/lomore/oh.gif'");
}

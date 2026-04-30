use hkg::api_utils::parse_html_content;

fn main() {
    let html = r#"A &amp; B"#;
    let nodes = parse_html_content(html);

    println!("Total nodes: {}", nodes.len());
    for (i, node) in nodes.iter().enumerate() {
        if let hkg::reply_model::NodeType::Text(text) = node {
            println!("Node {}: '{}' (len={})", i, text.data, text.data.len());
        }
    }
}

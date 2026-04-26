use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodeType {
    Text(TextNode),
    Image(ImageNode),
    BlockQuote(BlockQuoteNode),
    Br(BrNode),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextNode {
    pub data: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageNode {
    pub data: String,
    pub alt: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BrNode {
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockQuoteNode {
    pub data: Vec<NodeType>,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
pub enum NodeType {
    Text(TextNode),
    Image(ImageNode),
    BlockQuote(BlockQuoteNode),
    Br(BrNode),
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
pub struct TextNode {
    pub data: String,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
pub struct ImageNode {
    pub data: String,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
pub struct BrNode {
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
pub struct BlockQuoteNode {
    pub data: Vec<NodeType>,
}

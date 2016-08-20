use reply_model::*;

#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
pub struct ListTopicTitleItem {
    pub url: String,
    pub url_query: UrlQueryItem,
    pub text: String,
    pub num_of_pages: usize
}

#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
pub struct ListTopicAuthorItem {
    pub url: String,
    pub name: String,
}

#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
pub struct ListTopicItem {
    pub title: ListTopicTitleItem,
    pub author: ListTopicAuthorItem,
    pub last_replied_date: String,
    pub last_replied_time: String,
    pub reply_count: String,
    pub rating: String,
}


#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
pub struct ShowItem {
    pub url_query: UrlQueryItem,
    pub title: String,
    pub reply_count: String,
    pub page: usize,
    pub max_page: usize,
    pub replies: Vec<ShowReplyItem>
}

#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
pub struct ShowReplyItem {
    pub userid: String,
    pub username: String,
    pub content: String,
    pub body: Vec<NodeType>,
    pub published_at: String
}

#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
pub struct UrlQueryItem {
    pub channel: String,
    pub message: String
}

#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
pub struct IconItem {
    pub src: String,
    pub alt: String
}

#[derive(RustcDecodable)]
pub struct ListTopicTitleItem {
    pub url: String,
    pub text: String,
}

#[derive(RustcDecodable)]
pub struct ListTopicAuthorItem {
    pub url: String,
    pub name: String,
}


#[derive(RustcDecodable)]
pub struct ListTopicItem {
    pub titles: Vec<ListTopicTitleItem>,
    pub author: ListTopicAuthorItem,
    pub last_replied_date: String,
    pub last_replied_time: String,
    pub reply_count: String,
    pub rating: String,
}


#[derive(RustcDecodable)]
pub struct ShowItem {
    pub url_query: ShowUrlQueryItem,
    pub title: String,
    pub reply_count: String,
    pub page: usize,
    pub max_page: usize,
    pub replies: Vec<ShowReplyItem>
}

#[derive(RustcDecodable)]
pub struct ShowReplyItem {
    pub userid: String,
    pub username: String,
    pub content: String,
    pub published_at: String
}

#[derive(RustcDecodable)]
pub struct ShowUrlQueryItem {
    // pub type: String,
    pub message: String
}

#[derive(RustcDecodable)]
pub struct TopicTitleItem {
    pub url: String,
    pub text: String,
}

#[derive(RustcDecodable)]
pub struct TopicAuthorItem {
    pub url: String,
    pub name: String,
}


#[derive(RustcDecodable)]
pub struct TopicItem {
    pub titles: Vec<TopicTitleItem>,
    pub author: TopicAuthorItem,
    pub last_replied_date: String,
    pub last_replied_time: String,
    pub reply_count: String,
    pub rating: String,
}

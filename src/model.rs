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

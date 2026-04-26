use crate::reply_model::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListTopicTitleItem {
    pub url: String,
    pub url_query: UrlQueryItem,
    pub text: String,
    pub num_of_pages: usize
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListTopicAuthorItem {
    pub url: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListTopicItem {
    pub title: ListTopicTitleItem,
    pub author: ListTopicAuthorItem,
    pub last_replied_date: String,
    pub last_replied_time: String,
    pub reply_count: String,
    pub rating: String,
}


#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ShowItem {
    pub url_query: UrlQueryItem,
    pub title: String,
    pub reply_count: String,
    pub page: usize,
    pub max_page: usize,
    pub main_content: Option<ShowReplyItem>, // Main thread content
    pub replies: Vec<ShowReplyItem>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ShowReplyItem {
    pub userid: String,
    pub username: String,
    pub content: String,
    pub body: Vec<NodeType>,
    pub published_at: String
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UrlQueryItem {
    pub channel: String,
    pub message: String
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IconItem {
    pub src: String,
    pub alt: String
}

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

/// Type-safe wrapper for channel IDs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChannelId(String);

impl ChannelId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Default for ChannelId {
    fn default() -> Self {
        Self(String::new())
    }
}

impl std::fmt::Display for ChannelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Type-safe wrapper for message IDs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(String);

impl MessageId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self(String::new())
    }
}

impl std::fmt::Display for MessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UrlQueryItem {
    pub channel: ChannelId,
    pub message: MessageId
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IconItem {
    pub src: String,
    pub alt: String
}

use serde::{Deserialize, Serialize};

// ============================================================================
// TOPIC LIST API
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTopicListResponse {
    pub result: bool,
    pub data: ApiTopicListData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTopicListData {
    pub list: Vec<ApiTopic>,
    pub total: i32,
    #[serde(rename = "maxPage")]
    pub max_page: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTopic {
    #[serde(rename = "id")]
    pub id: i32,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "forum")]
    pub forum: String,

    #[serde(rename = "authorId")]
    pub author_id: i32,

    #[serde(rename = "authorName")]
    pub author_name: String,

    #[serde(rename = "authorGender")]
    pub author_gender: i32,

    #[serde(rename = "totalReplies")]
    pub total_replies: i32,

    #[serde(rename = "rating")]
    pub rating: i32,

    #[serde(rename = "totalPage")]
    pub total_page: i32,

    #[serde(rename = "messageDate")]
    pub message_date: i64,  // milliseconds since epoch

    #[serde(rename = "lastReplyDate")]
    pub last_reply_date: i64,  // milliseconds since epoch

    #[serde(rename = "thumbnail")]
    pub thumbnail: Option<String>,

    #[serde(rename = "iconType")]
    pub icon_type: String,

    #[serde(rename = "iconPath")]
    pub icon_path: String,
}

// ============================================================================
// THREAD VIEW API
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiThreadViewResponse {
    pub result: bool,
    pub data: ApiThreadViewData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiThreadViewData {
    // Main post fields (same as ApiTopic)
    #[serde(rename = "id")]
    pub id: i32,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "content")]
    pub content: String,  // HTML content

    #[serde(rename = "authorId")]
    pub author_id: i32,

    #[serde(rename = "authorName")]
    pub author_name: String,

    // Thread metadata
    #[serde(rename = "currentPage")]
    pub current_page: i32,

    #[serde(rename = "totalPage")]
    pub total_page: i32,

    #[serde(rename = "totalReplies")]
    pub total_replies: i32,

    #[serde(rename = "messageDate")]
    pub message_date: i64,  // milliseconds since epoch

    // Replies
    #[serde(rename = "replies")]
    pub replies: Vec<ApiReply>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiReply {
    #[serde(rename = "id")]
    pub id: i32,

    #[serde(rename = "index")]
    pub index: i32,

    #[serde(rename = "authorId")]
    pub author_id: i32,

    #[serde(rename = "authorName")]
    pub author_name: String,

    #[serde(rename = "authorGender")]
    pub author_gender: i32,

    #[serde(rename = "replyDate")]
    pub reply_date: i64,  // milliseconds since epoch

    #[serde(rename = "content")]
    pub content: String,  // HTML content

    #[serde(rename = "quoted")]
    pub quoted: Vec<ApiQuotedRef>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiQuotedRef {
    #[serde(rename = "index")]
    pub index: i32,

    #[serde(rename = "id")]
    pub id: i32,
}

// ============================================================================
// CONVERSION FUNCTIONS
// ============================================================================

/// Convert HKGolden API topic to domain Topic
///
/// This function converts a topic from the HKGolden API format
/// to the generic domain Topic model.
#[must_use]
pub fn api_topic_to_domain(api_topic: ApiTopic) -> crate::domain::Topic {
    crate::domain::Topic {
        id: api_topic.id,
        title: api_topic.title,
        forum: api_topic.forum,
        author_id: api_topic.author_id,
        author_name: api_topic.author_name,
        author_gender: Some(api_topic.author_gender),
        total_replies: api_topic.total_replies,
        rating: Some(api_topic.rating),
        total_page: api_topic.total_page,
        message_date: api_topic.message_date,
        last_reply_date: Some(api_topic.last_reply_date),
        thumbnail: None,
        icon_type: None,
        icon_path: None,
    }
}

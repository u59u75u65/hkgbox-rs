//! Topic domain model
//!
//! Represents a forum topic/thread in a generic way, independent of any specific API.

use serde::{Serialize, Deserialize};

/// A forum topic/thread
///
/// This domain model represents a topic in a generic way that can be populated
/// from different API sources (HKGolden, LIHKG, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Topic {
    /// Unique topic ID
    pub id: i32,

    /// Topic title
    pub title: String,

    /// Forum/channel this topic belongs to
    pub forum: String,

    /// Author ID
    pub author_id: i32,

    /// Author name
    pub author_name: String,

    /// Author gender (if available)
    pub author_gender: Option<i32>,

    /// Total number of replies
    pub total_replies: i32,

    /// Topic rating (if available)
    pub rating: Option<i32>,

    /// Total number of pages
    pub total_page: i32,

    /// Message date (milliseconds since epoch)
    pub message_date: i64,

    /// Last reply date (milliseconds since epoch)
    pub last_reply_date: Option<i64>,

    /// Thumbnail URL (if available)
    pub thumbnail: Option<String>,

    /// Icon type (if available)
    pub icon_type: Option<String>,

    /// Icon path (if available)
    pub icon_path: Option<String>,
}

impl Topic {
    /// Create a new Topic
    #[must_use]
    pub const fn new(
        id: i32,
        title: String,
        forum: String,
        author_id: i32,
        author_name: String,
        total_replies: i32,
        total_page: i32,
        message_date: i64,
    ) -> Self {
        Self {
            id,
            title,
            forum,
            author_id,
            author_name,
            author_gender: None,
            total_replies,
            rating: None,
            total_page,
            message_date,
            last_reply_date: None,
            thumbnail: None,
            icon_type: None,
            icon_path: None,
        }
    }

    /// Set author gender
    #[must_use]
    pub const fn with_author_gender(mut self, author_gender: i32) -> Self {
        self.author_gender = Some(author_gender);
        self
    }

    /// Set rating
    #[must_use]
    pub const fn with_rating(mut self, rating: i32) -> Self {
        self.rating = Some(rating);
        self
    }

    /// Set last reply date
    #[must_use]
    pub const fn with_last_reply_date(mut self, last_reply_date: i64) -> Self {
        self.last_reply_date = Some(last_reply_date);
        self
    }

    /// Set thumbnail
    #[must_use]
    pub fn with_thumbnail(mut self, thumbnail: String) -> Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    /// Set icon type
    #[must_use]
    pub fn with_icon_type(mut self, icon_type: String) -> Self {
        self.icon_type = Some(icon_type);
        self
    }

    /// Set icon path
    #[must_use]
    pub fn with_icon_path(mut self, icon_path: String) -> Self {
        self.icon_path = Some(icon_path);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic_creation() {
        let topic = Topic::new(
            123,
            "Test Topic".to_string(),
            "BW".to_string(),
            456,
            "TestUser".to_string(),
            10,
            2,
            1234567890,
        );

        assert_eq!(topic.id, 123);
        assert_eq!(topic.title, "Test Topic");
        assert_eq!(topic.forum, "BW");
        assert_eq!(topic.author_id, 456);
        assert_eq!(topic.author_name, "TestUser");
        assert_eq!(topic.total_replies, 10);
        assert_eq!(topic.total_page, 2);
        assert_eq!(topic.message_date, 1234567890);
    }

    #[test]
    fn test_topic_builder() {
        let topic = Topic::new(
            123,
            "Test Topic".to_string(),
            "BW".to_string(),
            456,
            "TestUser".to_string(),
            10,
            2,
            1234567890,
        )
        .with_author_gender(1)
        .with_rating(5)
        .with_last_reply_date(1234567900)
        .with_thumbnail("thumb.jpg".to_string())
        .with_icon_type("icon".to_string())
        .with_icon_path("/icons/icon.png".to_string());

        assert_eq!(topic.author_gender, Some(1));
        assert_eq!(topic.rating, Some(5));
        assert_eq!(topic.last_reply_date, Some(1234567900));
        assert_eq!(topic.thumbnail, Some("thumb.jpg".to_string()));
        assert_eq!(topic.icon_type, Some("icon".to_string()));
        assert_eq!(topic.icon_path, Some("/icons/icon.png".to_string()));
    }
}

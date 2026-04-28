//! Thread view domain models
//!
//! Represents a thread view with replies in a generic way, independent of any specific API.

use serde::{Serialize, Deserialize};

/// A thread view with main content and replies
///
/// This domain model represents a thread in a generic way that can be populated
/// from different API sources (HKGolden, LIHKG, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThreadView {
    /// Thread ID
    pub id: i32,

    /// Thread title
    pub title: String,

    /// Main thread content (raw HTML)
    pub content: String,

    /// Author ID
    pub author_id: i32,

    /// Author name
    pub author_name: String,

    /// Current page number
    pub current_page: i32,

    /// Total number of pages
    pub total_page: i32,

    /// Total number of replies
    pub total_replies: i32,

    /// Message date (milliseconds since epoch)
    pub message_date: i64,

    /// Replies
    pub replies: Vec<Reply>,
}

impl ThreadView {
    /// Create a new ThreadView
    #[must_use]
    pub fn new(
        id: i32,
        title: String,
        content: String,
        author_id: i32,
        author_name: String,
        current_page: i32,
        total_page: i32,
        total_replies: i32,
        message_date: i64,
    ) -> Self {
        Self {
            id,
            title,
            content,
            author_id,
            author_name,
            current_page,
            total_page,
            total_replies,
            message_date,
            replies: Vec::new(),
        }
    }

    /// Add replies to the thread
    #[must_use]
    pub fn with_replies(mut self, replies: Vec<Reply>) -> Self {
        self.replies = replies;
        self
    }
}

/// A reply in a thread
///
/// This domain model represents a reply in a generic way.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Reply {
    /// Reply ID
    pub id: i32,

    /// Reply index number
    pub index: i32,

    /// Author ID
    pub author_id: i32,

    /// Author name
    pub author_name: String,

    /// Author gender (if available)
    pub author_gender: Option<i32>,

    /// Reply date (milliseconds since epoch)
    pub reply_date: i64,

    /// Reply content (raw HTML)
    pub content: String,

    /// Quoted reply references (if available)
    pub quoted: Vec<QuotedRef>,
}

impl Reply {
    /// Create a new Reply
    #[must_use]
    pub fn new(
        id: i32,
        index: i32,
        author_id: i32,
        author_name: String,
        reply_date: i64,
        content: String,
    ) -> Self {
        Self {
            id,
            index,
            author_id,
            author_name,
            author_gender: None,
            reply_date,
            content,
            quoted: Vec::new(),
        }
    }

    /// Set author gender
    #[must_use]
    pub const fn with_author_gender(mut self, author_gender: i32) -> Self {
        self.author_gender = Some(author_gender);
        self
    }

    /// Add quoted references
    #[must_use]
    pub fn with_quoted(mut self, quoted: Vec<QuotedRef>) -> Self {
        self.quoted = quoted;
        self
    }
}

/// A quoted reply reference
///
/// Points to another reply that was quoted in this reply.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuotedRef {
    /// Reply index number
    pub index: i32,

    /// Reply ID
    pub id: i32,
}

impl QuotedRef {
    /// Create a new QuotedRef
    #[must_use]
    pub const fn new(index: i32, id: i32) -> Self {
        Self { index, id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_view_creation() {
        let thread = ThreadView::new(
            123,
            "Test Thread".to_string(),
            "<p>Test content</p>".to_string(),
            456,
            "TestUser".to_string(),
            1,
            5,
            10,
            1234567890,
        );

        assert_eq!(thread.id, 123);
        assert_eq!(thread.title, "Test Thread");
        assert_eq!(thread.content, "<p>Test content</p>");
        assert_eq!(thread.author_id, 456);
        assert_eq!(thread.author_name, "TestUser");
        assert_eq!(thread.current_page, 1);
        assert_eq!(thread.total_page, 5);
        assert_eq!(thread.total_replies, 10);
        assert_eq!(thread.message_date, 1234567890);
        assert!(thread.replies.is_empty());
    }

    #[test]
    fn test_thread_view_with_replies() {
        let reply1 = Reply::new(
            1,
            1,
            456,
            "TestUser".to_string(),
            1234567890,
            "<p>Reply 1</p>".to_string(),
        );

        let reply2 = Reply::new(
            2,
            2,
            789,
            "OtherUser".to_string(),
            1234567891,
            "<p>Reply 2</p>".to_string(),
        );

        let thread = ThreadView::new(
            123,
            "Test Thread".to_string(),
            "<p>Test content</p>".to_string(),
            456,
            "TestUser".to_string(),
            1,
            1,
            2,
            1234567890,
        )
        .with_replies(vec![reply1, reply2]);

        assert_eq!(thread.replies.len(), 2);
    }

    #[test]
    fn test_reply_creation() {
        let reply = Reply::new(
            1,
            1,
            456,
            "TestUser".to_string(),
            1234567890,
            "<p>Reply content</p>".to_string(),
        );

        assert_eq!(reply.id, 1);
        assert_eq!(reply.index, 1);
        assert_eq!(reply.author_id, 456);
        assert_eq!(reply.author_name, "TestUser");
        assert_eq!(reply.reply_date, 1234567890);
        assert_eq!(reply.content, "<p>Reply content</p>");
        assert!(reply.quoted.is_empty());
    }

    #[test]
    fn test_reply_builder() {
        let quoted = vec![QuotedRef::new(1, 100)];

        let reply = Reply::new(
            1,
            1,
            456,
            "TestUser".to_string(),
            1234567890,
            "<p>Reply content</p>".to_string(),
        )
        .with_author_gender(1)
        .with_quoted(quoted);

        assert_eq!(reply.author_gender, Some(1));
        assert_eq!(reply.quoted.len(), 1);
        assert_eq!(reply.quoted[0].index, 1);
        assert_eq!(reply.quoted[0].id, 100);
    }

    #[test]
    fn test_quoted_ref() {
        let quoted = QuotedRef::new(5, 500);
        assert_eq!(quoted.index, 5);
        assert_eq!(quoted.id, 500);
    }
}

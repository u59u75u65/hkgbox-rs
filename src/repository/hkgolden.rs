//! HKGolden API repository implementation
//!
//! This module provides repository implementations backed by the HKGolden API.

use std::sync::Arc;
use crate::api_client::HkgApiClient;
use crate::api_models::{ApiTopic, ApiReply, ApiQuotedRef};
use crate::domain::{Topic, ThreadView, Reply, QuotedRef as DomainQuotedRef};
use super::{TopicRepository, ThreadRepository, RepositoryError, RepositoryResult};

/// HKGolden API topic repository
///
/// Implements `TopicRepository` using the HKGolden API client.
pub struct HkgoldenTopicRepository {
    client: Arc<HkgApiClient>,
}

impl HkgoldenTopicRepository {
    /// Create a new HKGolden topic repository
    ///
    /// # Arguments
    /// * `client` - HKGolden API client
    #[must_use]
    pub fn new(client: Arc<HkgApiClient>) -> Self {
        Self { client }
    }
}

impl TopicRepository for HkgoldenTopicRepository {
    fn fetch_topics(
        &self,
        channel: &str,
        page: i32,
    ) -> RepositoryResult<(Vec<Topic>, i32)> {
        let response = self.client.fetch_topics(channel, page)
            .map_err(|e| RepositoryError::NetworkError(e.to_string()))?;

        // Check API response success flag
        if !response.result {
            return Err(RepositoryError::ApiError(
                "API returned unsuccessful response".to_string()
            ));
        }

        let topics: Vec<Topic> = response.data.list
            .into_iter()
            .map(|api_topic| Self::convert_topic(api_topic, channel))
            .collect();

        let max_page = response.data.max_page;

        Ok((topics, max_page))
    }
}

impl HkgoldenTopicRepository {
    /// Convert HKGolden API topic model to domain model
    fn convert_topic(api_topic: ApiTopic, forum: &str) -> Topic {
        Topic {
            id: api_topic.id,
            title: api_topic.title,
            forum: forum.to_string(),
            author_id: api_topic.author_id,
            author_name: api_topic.author_name,
            author_gender: Some(api_topic.author_gender),
            total_replies: api_topic.total_replies,
            rating: Some(api_topic.rating),
            total_page: api_topic.total_page,
            message_date: api_topic.message_date,
            last_reply_date: Some(api_topic.last_reply_date),
            thumbnail: api_topic.thumbnail,
            icon_type: Some(api_topic.icon_type),
            icon_path: Some(api_topic.icon_path),
        }
    }
}

/// HKGolden API thread repository
///
/// Implements `ThreadRepository` using the HKGolden API client.
pub struct HkgoldenThreadRepository {
    client: Arc<HkgApiClient>,
}

impl HkgoldenThreadRepository {
    /// Create a new HKGolden thread repository
    ///
    /// # Arguments
    /// * `client` - HKGolden API client
    #[must_use]
    pub fn new(client: Arc<HkgApiClient>) -> Self {
        Self { client }
    }
}

impl ThreadRepository for HkgoldenThreadRepository {
    fn fetch_thread(
        &self,
        thread_id: i32,
        page: i32,
    ) -> RepositoryResult<ThreadView> {
        let response = self.client.fetch_thread(thread_id, page)
            .map_err(|e| RepositoryError::NetworkError(e.to_string()))?;

        // Check API response success flag
        if !response.result {
            return Err(RepositoryError::ApiError(
                "API returned unsuccessful response".to_string()
            ));
        }

        let thread = Self::convert_thread_view(response, thread_id);

        Ok(thread)
    }
}

impl HkgoldenThreadRepository {
    /// Convert HKGolden API thread view model to domain model
    fn convert_thread_view(
        api_response: crate::api_models::ApiThreadViewResponse,
        _thread_id: i32,
    ) -> ThreadView {
        let data = api_response.data;

        let replies: Vec<Reply> = data.replies
            .into_iter()
            .map(Self::convert_reply)
            .collect();

        ThreadView {
            id: data.id,
            title: data.title,
            content: data.content,
            author_id: data.author_id,
            author_name: data.author_name,
            current_page: data.current_page,
            total_page: data.total_page,
            total_replies: data.total_replies,
            message_date: data.message_date,
            replies,
        }
    }

    /// Convert HKGolden API reply model to domain model
    fn convert_reply(api_reply: ApiReply) -> Reply {
        let quoted: Vec<DomainQuotedRef> = api_reply.quoted
            .into_iter()
            .map(Self::convert_quoted_ref)
            .collect();

        Reply {
            id: api_reply.id,
            index: api_reply.index,
            author_id: api_reply.author_id,
            author_name: api_reply.author_name,
            author_gender: Some(api_reply.author_gender),
            reply_date: api_reply.reply_date,
            content: api_reply.content,
            quoted,
        }
    }

    /// Convert HKGolden API quoted ref model to domain model
    fn convert_quoted_ref(api_quoted: ApiQuotedRef) -> DomainQuotedRef {
        DomainQuotedRef {
            index: api_quoted.index,
            id: api_quoted.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_topic() {
        let api_topic = ApiTopic {
            id: 123,
            title: "Test Topic".to_string(),
            forum: "BW".to_string(),
            author_id: 456,
            author_name: "TestUser".to_string(),
            author_gender: 1,
            total_replies: 10,
            rating: 5,
            total_page: 2,
            message_date: 1234567890,
            last_reply_date: 1234567900,
            thumbnail: Some("thumb.jpg".to_string()),
            icon_type: "icon".to_string(),
            icon_path: "/icons/icon.png".to_string(),
        };

        let topic = HkgoldenTopicRepository::convert_topic(api_topic, "BW");

        assert_eq!(topic.id, 123);
        assert_eq!(topic.title, "Test Topic");
        assert_eq!(topic.forum, "BW");
        assert_eq!(topic.author_id, 456);
        assert_eq!(topic.author_name, "TestUser");
        assert_eq!(topic.author_gender, Some(1));
        assert_eq!(topic.total_replies, 10);
        assert_eq!(topic.rating, Some(5));
        assert_eq!(topic.total_page, 2);
        assert_eq!(topic.message_date, 1234567890);
        assert_eq!(topic.last_reply_date, Some(1234567900));
        assert_eq!(topic.thumbnail, Some("thumb.jpg".to_string()));
        assert_eq!(topic.icon_type, Some("icon".to_string()));
        assert_eq!(topic.icon_path, Some("/icons/icon.png".to_string()));
    }

    #[test]
    fn test_convert_reply() {
        let api_reply = ApiReply {
            id: 1,
            index: 1,
            author_id: 456,
            author_name: "TestUser".to_string(),
            author_gender: 1,
            reply_date: 1234567890,
            content: "<p>Test</p>".to_string(),
            quoted: vec![ApiQuotedRef { index: 1, id: 100 }],
        };

        let reply = HkgoldenThreadRepository::convert_reply(api_reply);

        assert_eq!(reply.id, 1);
        assert_eq!(reply.index, 1);
        assert_eq!(reply.author_id, 456);
        assert_eq!(reply.author_name, "TestUser");
        assert_eq!(reply.author_gender, Some(1));
        assert_eq!(reply.reply_date, 1234567890);
        assert_eq!(reply.content, "<p>Test</p>");
        assert_eq!(reply.quoted.len(), 1);
        assert_eq!(reply.quoted[0].index, 1);
        assert_eq!(reply.quoted[0].id, 100);
    }

    #[test]
    fn test_convert_quoted_ref() {
        let api_quoted = ApiQuotedRef { index: 5, id: 500 };
        let quoted = HkgoldenThreadRepository::convert_quoted_ref(api_quoted);

        assert_eq!(quoted.index, 5);
        assert_eq!(quoted.id, 500);
    }
}

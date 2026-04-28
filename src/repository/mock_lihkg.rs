//! Mock LIHKG repository for testing and demonstration
//!
//! This module provides a mock implementation of the LIHKG repository
//! that returns sample data when the real LIHKG API is inaccessible
//! (due to Cloudflare protection).

use crate::repository::{RepositoryResult, RepositoryError, TopicRepository};
use crate::domain::Topic;
use std::sync::Arc;

/// Mock LIHKG topic repository
///
/// Returns sample data for testing and demonstration purposes when
/// the real LIHKG API is inaccessible.
pub struct MockLihkgTopicRepository {
    cat_id: i32,
}

impl MockLihkgTopicRepository {
    /// Create a new mock LIHKG topic repository
    ///
    /// # Arguments
    /// * `cat_id` - Category ID (for display purposes)
    pub fn new(cat_id: i32) -> Self {
        Self { cat_id }
    }

    /// Get mock topics for testing
    fn get_mock_topics(&self, page: i32) -> Vec<Topic> {
        let category_name = match self.cat_id {
            1 => "吹水台",
            _ => "未知分類",
        };

        let mut topics = Vec::new();

        // Generate different mock data based on page number
        let base_id = (page - 1) * 10 + 1;

        for i in 0..10 {
            let topic_id = base_id + i;
            topics.push(Topic {
                id: topic_id,
                title: format!("【LIHKG測試話題{}】這是第{}頁的模擬數據", i + 1, page),
                forum: category_name.to_string(),
                author_id: 1000 + i,
                author_name: format!("LIHKG用戶{}", i + 1),
                author_gender: Some(if i % 2 == 0 { 1 } else { 2 }),
                total_replies: (i + 1) * 5,
                rating: Some((i + 1) * 10),
                total_page: 5, // Mock 5 pages
                message_date: 1715865600000 + (i * 1000000) as i64, // Different timestamps
                last_reply_date: Some(1715865600000 + (i * 1000000 + 60000) as i64),
                thumbnail: None,
                icon_type: None,
                icon_path: None,
            });
        }

        topics
    }
}

impl TopicRepository for MockLihkgTopicRepository {
    fn fetch_topics(
        &self,
        _channel: &str,
        page: i32,
    ) -> RepositoryResult<(Vec<Topic>, i32)> {
        log::info!("[MockLihkgTopicRepository] Returning mock data for page {} (cat_id: {})", page, self.cat_id);

        // Simulate network delay
        std::thread::sleep(std::time::Duration::from_millis(100));

        let topics = self.get_mock_topics(page);
        let max_page = 5; // Mock 5 pages

        Ok((topics, max_page))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_repository() {
        let repo = MockLihkgTopicRepository::new(1); // 吹水台

        let (topics, max_page) = repo.fetch_topics("BW", 1).unwrap();

        assert_eq!(topics.len(), 10);
        assert_eq!(max_page, 5);

        let first = &topics[0];
        assert_eq!(first.id, 1);
        assert!(first.title.contains("LIHKG測試話題"));
        assert_eq!(first.forum, "吹水台");

        let last = &topics[9];
        assert_eq!(last.id, 10);
    }

    #[test]
    fn test_mock_different_pages() {
        let repo = MockLihkgTopicRepository::new(1);

        let (page1, _) = repo.fetch_topics("BW", 1).unwrap();
        let (page2, _) = repo.fetch_topics("BW", 2).unwrap();

        assert_eq!(page1[0].id, 1);
        assert_eq!(page2[0].id, 11); // Page 2 starts with ID 11
    }
}

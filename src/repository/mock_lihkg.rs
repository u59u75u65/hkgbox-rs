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

    fn get_channel_title(&self, channel_code: &str) -> String {
        // Use the same channel mappings as the real LIHKG repository
        match channel_code {
            "1" => "吹水台",
            "999" => "自選台",
            "2" => "熱　門",
            "3" => "最　新",
            "5" => "時事台",
            "33" => "政事台",
            "38" => "World",
            "15" => "財經台",
            "37" => "房屋台",
            "6" => "體育台",
            "7" => "娛樂台",
            "8" => "動漫台",
            "10" => "遊戲台",
            "11" => "影視台",
            "12" => "講故台",
            "4" => "手機台",
            "9" => "Apps台",
            "22" => "硬件台",
            "26" => "軟件台",
            "41" => "電器台",
            "21" => "音樂台",
            "23" => "攝影台",
            "24" => "玩具台",
            "25" => "寵物台",
            "20" => "汽車台",
            "31" => "創意台",
            "30" => "感情台",
            "36" => "健康台",
            "39" => "家庭台",
            "14" => "上班台",
            "16" => "飲食台",
            "17" => "旅遊台",
            "18" => "學術台",
            "19" => "校園台",
            "13" => "潮流台",
            "40" => "美容台",
            "27" => "活動台",
            "28" => "站務台",
            "29" => "成人台",
            "32" => "黑　洞",
            "34" => "直播台",
            "35" => "電訊台",
            _ => channel_code,
        }.to_string()
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

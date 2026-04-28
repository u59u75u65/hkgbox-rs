//! LIHKG API repository implementation
//!
//! This module provides repository implementations for the LIHKG (LIHKG.com) API,
//! demonstrating how the repository abstraction allows supporting multiple forum services.

use crate::repository::{RepositoryResult, RepositoryError, TopicRepository, ThreadRepository};
use crate::domain::{Topic, ThreadView, Reply, QuotedRef};

/// Generate a device ID for LIHKG API requests
///
/// Creates a device identifier as LIHKG expects
fn generate_device_id() -> String {
    // Hardcoded working device ID from Playwright testing
    // This matches the format that successfully bypasses LIHKG rate limiting
    "89ee2a48214807d7762894d2ae9d07500e339171".to_string()
}

/// Calculate a realistic page load time
///
/// Simulates browser page load time between 1-5 seconds
fn calculate_load_time() -> f64 {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // Use fractional part to create variation between 1-5 seconds
    let fractional = now.fract();
    (fractional * 4.0) + 1.0  // Range: 1.0 to 5.0 seconds
}

/// LIHKG API client
///
/// Basic HTTP client for LIHKG API endpoints.
pub struct LihkgApiClient {
    http_client: reqwest::blocking::Client,
    base_url: String,
}

impl LihkgApiClient {
    /// Create a new LIHKG API client
    ///
    /// # Errors
    /// Returns an error if HTTP client creation fails
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            http_client: reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()?,
            base_url: "https://lihkg.com/api_v2".to_string(),
        })
    }

    /// Fetch latest threads from a category
    ///
    /// # Arguments
    /// * `cat_id` - Category ID (e.g., 1 for 吹水台)
    /// * `page` - Page number (1-indexed)
    /// * `count` - Number of threads per page (default: 60)
    ///
    /// # Returns
    /// LIHKG thread list response
    ///
    /// # Errors
    /// Returns error if HTTP request fails or response is invalid
    fn fetch_latest_threads(
        &self,
        cat_id: i32,
        page: i32,
        count: i32,
    ) -> Result<LihkgThreadListResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/thread/latest?cat_id={}&page={}&count={}&type=now",
            self.base_url, cat_id, page, count
        );

        log::info!("[LihkgApiClient] Fetching from: {}", url);

        // Generate required LIHKG headers based on Playwright investigation
        let device_id = generate_device_id();
        let load_time = calculate_load_time();

        log::debug!("[LihkgApiClient] Using device_id: {}, load_time: {}", device_id, load_time);

        // Add all required headers for LIHKG API
        let response = self.http_client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .header("Accept", "application/json, text/plain, */*")
            .header("Referer", "https://lihkg.com/category/1")
            .header("x-li-device-type", "browser")
            .header("x-li-device", &device_id)
            .header("x-li-load-time", &load_time.to_string())
            .header("sec-ch-ua", r#""Chromium";v="147", "Not.A/Brand";v="8""#)
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-platform", r#""macOS""#)
            .send()?;

        let status = response.status();
        log::info!("[LihkgApiClient] Response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().unwrap_or_else(|_| "Unable to read error response".to_string());
            log::error!("[LihkgApiClient] API returned error {}: {}", status, error_text);

            if status.as_u16() == 1020 {
                return Err(
                    "LIHKG API is protected by Cloudflare. \
                    The API requires browser cookies or Cloudflare challenge completion. \
                    Please use HKGolden service or implement proper session handling.".into()
                );
            }

            return Err(format!("HTTP error: {} - {}", status, error_text).into());
        }

        let json: serde_json::Value = response.json()?;

        // Check for Cloudflare error response
        if let Some(err_code) = json["error_code"].as_i64() {
            if err_code == 1020 {
                return Err(
                    "LIHKG API is protected by Cloudflare and cannot be accessed programmatically without proper session handling.".into()
                );
            }
        }

        // Check success flag
        if json["success"].as_i64() != Some(1) {
            let error_msg = json["error_message"].as_str()
                .or(json["message"].as_str())
                .unwrap_or("Unknown error");
            return Err(format!("API request failed: {}", error_msg).into());
        }

        serde_json::from_value(json["response"].clone())
            .map_err(|e| format!("Failed to parse LIHKG response: {}", e).into())
    }

    /// Fetch a thread with replies
    ///
    /// # Arguments
    /// * `thread_id` - Thread ID to fetch
    /// * `page` - Page number (1-indexed)
    ///
    /// # Returns
    /// LIHKG thread view response
    ///
    /// # Errors
    /// Returns error if HTTP request fails or response is invalid
    fn fetch_thread(
        &self,
        thread_id: i32,
        page: i32,
    ) -> Result<LihkgThreadResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/thread/{}/page/{}?order=reply_time",
            self.base_url, thread_id, page
        );

        log::info!("[LihkgApiClient] Fetching thread from: {}", url);

        // Generate required LIHKG headers based on Playwright investigation
        let device_id = generate_device_id();
        let load_time = calculate_load_time();

        log::debug!("[LihkgApiClient] Using device_id: {}, load_time: {}", device_id, load_time);

        // Add all required headers for LIHKG API
        let response = self.http_client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .header("Accept", "application/json, text/plain, */*")
            .header("Referer", "https://lihkg.com/thread/")  // Updated referer for thread pages
            .header("x-li-device-type", "browser")
            .header("x-li-device", &device_id)
            .header("x-li-load-time", &load_time.to_string())
            .header("sec-ch-ua", r#""Chromium";v="147", "Not.A/Brand";v="8""#)
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-platform", r#""macOS""#)
            .send()?;

        let status = response.status();
        log::info!("[LihkgApiClient] Response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().unwrap_or_else(|_| "Unable to read error response".to_string());
            log::error!("[LihkgApiClient] API returned error {}: {}", status, error_text);

            if status.as_u16() == 1020 {
                return Err(
                    "LIHKG API is protected by Cloudflare. \
                    The API requires browser cookies or Cloudflare challenge completion.".into()
                );
            }

            return Err(format!("HTTP error: {} - {}", status, error_text).into());
        }

        let json: serde_json::Value = response.json()?;

        // Check for Cloudflare error response
        if let Some(err_code) = json["error_code"].as_i64() {
            if err_code == 1020 {
                return Err(
                    "LIHKG API is protected by Cloudflare and cannot be accessed programmatically.".into()
                );
            }
        }

        // Check success flag
        if json["success"].as_i64() != Some(1) {
            let error_msg = json["error_message"].as_str()
                .or(json["message"].as_str())
                .unwrap_or("Unknown error");
            return Err(format!("API request failed: {}", error_msg).into());
        }

        serde_json::from_value(json["response"].clone())
            .map_err(|e| format!("Failed to parse LIHKG thread response: {}", e).into())
    }
}

/// LIHKG thread list API response
#[derive(Debug, serde::Deserialize)]
struct LihkgThreadListResponse {
    items: Vec<LihkgThreadItem>,
    // total_page is not at response level, it's in individual thread items
    is_pagination: Option<bool>,
    category: Option<LihkgCategory>,
}

/// LIHKG thread item from list API
#[derive(Debug, serde::Deserialize)]
struct LihkgThreadItem {
    thread_id: i32,
    title: String,
    user: LihkgUser,
    category: Option<LihkgCategory>,
    cat_id: Option<i32>,  // Category ID at top level
    create_time: i64,
    no_of_reply: i32,      // LIHKG uses "no_of_reply" instead of "reply_count"
    like_count: i32,
    dislike_count: i32,
    total_page: Option<i32>, // Make optional as it might not always be present
    last_reply_time: Option<i64>, // Make optional
    user_id: Option<i32>,          // User ID at top level
    user_nickname: Option<String>, // User nickname at top level
    user_gender: Option<String>,   // User gender at top level
}

/// LIHKG user info
#[derive(Debug, serde::Deserialize)]
struct LihkgUser {
    user_id: i32,
    nickname: String,
    gender: String,
}

/// LIHKG category info
#[derive(Debug, serde::Deserialize)]
struct LihkgCategory {
    cat_id: i32,
    name: String,
}

/// LIHKG thread view API response
#[derive(Debug, serde::Deserialize)]
struct LihkgThreadResponse {
    thread: LihkgThreadDetail,
    total_page: i32,
}

/// LIHKG thread detail
#[derive(Debug, serde::Deserialize)]
struct LihkgThreadDetail {
    thread_id: i32,
    title: String,
    user: LihkgUser,
    category: LihkgCategory,
    create_time: i64,
    reply_count: i32,
    like_count: i32,
    dislike_count: i32,
    msg: String,
}

/// LIHKG reply item
#[derive(Debug, serde::Deserialize)]
struct LihkgReply {
    post_id: i32,
    user: LihkgUser,
    msg: String,
    like_count: i32,
    dislike_count: i32,
    create_time: i64,
    #[serde(default)]
    quote_reply: Option<Box<LihkgReply>>,
}

/// LIHKG topic repository
///
/// Implements `TopicRepository` trait for LIHKG API.
pub struct LihkgTopicRepository {
    client: LihkgApiClient,
    cat_id: i32,
}

impl LihkgTopicRepository {
    /// Create a new LIHKG topic repository
    ///
    /// # Arguments
    /// * `cat_id` - Category ID (e.g., 1 for 吹水台)
    ///
    /// # Examples
    /// ```no_run
    /// use hkg::repository::LihkgTopicRepository;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let repo = LihkgTopicRepository::new(1); // 吹水台
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(cat_id: i32) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            client: LihkgApiClient::new()?,
            cat_id,
        })
    }

    /// Create a new LIHKG topic repository with custom client
    ///
    /// Useful for testing with mock clients.
    pub fn with_client(cat_id: i32, client: LihkgApiClient) -> Self {
        Self {
            client,
            cat_id,
        }
    }

    /// Convert LIHKG thread item to domain Topic
    fn convert_topic(&self, item: LihkgThreadItem) -> Topic {
        // Convert Unix timestamp (seconds) to milliseconds
        let message_date_ms = item.create_time * 1000;
        let last_reply_date_ms = item.last_reply_time.unwrap_or(item.create_time) * 1000;

        // Use category from nested object or fallback to top-level cat_id
        let category_name = item.category
            .map(|c| c.name)
            .unwrap_or_else(|| {
                // Map cat_id to category name
                match item.cat_id.unwrap_or(1) {
                    1 => "吹水台",
                    2 => "熱門",
                    5 => "時事台",
                    _ => "未知分類",
                }.to_string()
            });

        // Get author info from top-level fields (primary) or nested user object (fallback)
        let author_name = item.user_nickname
            .unwrap_or_else(|| item.user.nickname.clone());

        let author_id = item.user_id
            .unwrap_or_else(|| item.user.user_id);

        Topic {
            id: item.thread_id,
            title: item.title,
            forum: category_name,
            author_id,
            author_name,
            author_gender: Some(1), // Default gender
            total_replies: item.no_of_reply, // Use correct field name
            rating: Some((item.like_count - item.dislike_count).max(0)),
            total_page: item.total_page.unwrap_or(1), // Default to 1 if not present
            message_date: message_date_ms,
            last_reply_date: Some(last_reply_date_ms),
            thumbnail: None,
            icon_type: None,
            icon_path: None,
        }
    }
}

impl TopicRepository for LihkgTopicRepository {
    fn fetch_topics(
        &self,
        _channel: &str,
        page: i32,
    ) -> RepositoryResult<(Vec<Topic>, i32)> {
        let response = self.client.fetch_latest_threads(self.cat_id, page, 60)
            .map_err(|e| RepositoryError::ApiError(e.to_string()))?;

        let topics: Vec<Topic> = response.items
            .into_iter()
            .map(|item| self.convert_topic(item))
            .collect();

        // Calculate max page - LIHKG API doesn't provide total pages at response level
        // Use a reasonable default based on typical LIHKG behavior
        // If we got less than 60 items, we might be on the last page
        let max_page = if topics.len() < 60 {
            page // Last page
        } else {
            page + 10 // Default estimate if we got a full page
        };

        Ok((topics, max_page))
    }
}

/// LIHKG thread repository
///
/// Implements `ThreadRepository` trait for LIHKG API.
pub struct LihkgThreadRepository {
    client: LihkgApiClient,
}

impl LihkgThreadRepository {
    /// Create a new LIHKG thread repository
    ///
    /// # Errors
    /// Returns an error if API client creation fails
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            client: LihkgApiClient::new()?,
        })
    }

    /// Create a new LIHKG thread repository with custom client
    ///
    /// Useful for testing with mock clients.
    pub fn with_client(client: LihkgApiClient) -> Self {
        Self { client }
    }

    /// Convert LIHKG reply to domain Reply
    ///
    /// Note: LIHKG API doesn't provide reply index, so we use 0 as placeholder.
    /// The index should be assigned based on position in the thread.
    fn convert_reply(&self, lihkg_reply: LihkgReply, index: i32) -> Reply {
        // Convert Unix timestamp (seconds) to milliseconds
        let reply_date_ms = lihkg_reply.create_time * 1000;

        // Convert quoted reply if exists
        let quoted = if let Some(qr) = lihkg_reply.quote_reply {
            vec![QuotedRef {
                index: 0, // LIHKG doesn't provide quote index
                id: qr.post_id,
            }]
        } else {
            Vec::new()
        };

        Reply {
            id: lihkg_reply.post_id,
            index,
            author_id: lihkg_reply.user.user_id,
            author_name: lihkg_reply.user.nickname,
            author_gender: None,
            reply_date: reply_date_ms,
            content: lihkg_reply.msg,
            quoted,
        }
    }
}

impl ThreadRepository for LihkgThreadRepository {
    fn fetch_thread(
        &self,
        thread_id: i32,
        page: i32,
    ) -> RepositoryResult<ThreadView> {
        let response = self.client.fetch_thread(thread_id, page)
            .map_err(|e| RepositoryError::ApiError(e.to_string()))?;

        let thread = response.thread;

        // Convert Unix timestamp (seconds) to milliseconds
        let message_date_ms = thread.create_time * 1000;

        // Note: The LIHKG thread API response structure may vary
        // For now, we construct a basic thread view
        // In a real implementation, we'd parse the reply items from the response
        Ok(ThreadView {
            id: thread.thread_id,
            title: thread.title,
            content: thread.msg,
            author_id: thread.user.user_id,
            author_name: thread.user.nickname,
            current_page: page,
            total_page: response.total_page,
            total_replies: thread.reply_count,
            message_date: message_date_ms,
            replies: Vec::new(), // Would be populated from response items
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_topic() {
        let repo = LihkgTopicRepository::new(1).unwrap();

        let lihkg_item = LihkgThreadItem {
            thread_id: 123456,
            title: "Test Thread".to_string(),
            user: LihkgUser {
                user_id: 789,
                nickname: "TestUser".to_string(),
                gender: "M".to_string(),
            },
            category: Some(LihkgCategory {
                cat_id: 1,
                name: "吹水台".to_string(),
            }),
            cat_id: Some(1),
            create_time: 1609459200, // 2021-01-01 00:00:00 UTC
            no_of_reply: 42, // Use correct field name
            like_count: 10,
            dislike_count: 2,
            total_page: Some(3),
            last_reply_time: Some(1609459260),
            user_id: Some(789),
            user_nickname: Some("TestUser".to_string()),
            user_gender: Some("M".to_string()),
        };

        let topic = repo.convert_topic(lihkg_item);

        assert_eq!(topic.id, 123456);
        assert_eq!(topic.title, "Test Thread");
        assert_eq!(topic.author_name, "TestUser");
        assert_eq!(topic.author_id, 789);
        assert_eq!(topic.rating, Some(8)); // 10 - 2
        assert_eq!(topic.total_replies, 42);
        assert_eq!(topic.forum, "吹水台");
        assert_eq!(topic.total_page, 3); // Use actual total_page from item
    }

    #[test]
    fn test_convert_reply() {
        let repo = LihkgThreadRepository::new().unwrap();

        let lihkg_reply = LihkgReply {
            post_id: 456,
            user: LihkgUser {
                user_id: 789,
                nickname: "ReplyUser".to_string(),
                gender: "F".to_string(),
            },
            msg: "Test reply content".to_string(),
            like_count: 5,
            dislike_count: 1,
            create_time: 1609459260,
            quote_reply: None,
        };

        let reply = repo.convert_reply(lihkg_reply, 1);

        assert_eq!(reply.id, 456);
        assert_eq!(reply.index, 1);
        assert_eq!(reply.author_name, "ReplyUser");
        assert_eq!(reply.author_id, 789);
        assert_eq!(reply.content, "Test reply content");
        assert!(reply.quoted.is_empty());
    }
}

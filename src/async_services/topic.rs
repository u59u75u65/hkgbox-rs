//! Async Topic service for business logic related to topics
//!
//! This async service handles topic-related operations including fetching,
//! pagination, filtering, and state management using non-blocking I/O.

use crate::async_api_client::{AsyncHkgApiClient, AsyncResult};
use crate::api_models::ApiTopicListResponse;
use std::sync::Arc;

/// Async service for topic-related business logic
///
/// The `AsyncTopicService` encapsulates all business logic related to topics,
/// providing validation, pagination calculations, and async data fetching.
pub struct AsyncTopicService {
    api_client: Arc<AsyncHkgApiClient>,
}

impl AsyncTopicService {
    /// Create a new AsyncTopicService
    ///
    /// # Arguments
    /// * `api_client` - Async API client instance
    ///
    /// # Returns
    /// A new topic service instance
    pub fn new(api_client: Arc<AsyncHkgApiClient>) -> Self {
        Self { api_client }
    }

    /// Fetch topics for a specific channel and page
    ///
    /// # Arguments
    /// * `channel` - Channel code (e.g., "BW", "HT")
    /// * `page` - Page number to fetch
    ///
    /// # Returns
    /// API response containing topic list
    ///
    /// # Examples
    /// ```no_run
    /// use hkg::async_api_client::AsyncHkgApiClient;
    /// use hkg::async_services::AsyncTopicService;
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let client = Arc::new(AsyncHkgApiClient::new().await?);
    /// let service = AsyncTopicService::new(client);
    /// let topics = service.fetch_topics("BW", 1).await?;
    /// println!("Fetched {} topics", topics.data.list.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_topics(
        &self,
        channel: &str,
        page: i32,
    ) -> AsyncResult<ApiTopicListResponse> {
        self.api_client.fetch_topics(channel, page).await
    }

    /// Fetch multiple pages concurrently
    ///
    /// # Arguments
    /// * `channel` - Channel code
    /// * `pages` - List of page numbers to fetch
    ///
    /// # Returns
    /// Vector of API responses, one per page
    ///
    /// # Examples
    /// ```no_run
    /// use hkg::async_api_client::AsyncHkgApiClient;
    /// use hkg::async_services::AsyncTopicService;
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let client = Arc::new(AsyncHkgApiClient::new().await?);
    /// let service = AsyncTopicService::new(client);
    /// let pages = vec![1, 2, 3];
    /// let results = service.fetch_multiple_pages("BW", pages).await?;
    /// println!("Fetched {} pages", results.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_multiple_pages(
        &self,
        channel: &str,
        pages: Vec<i32>,
    ) -> AsyncResult<Vec<ApiTopicListResponse>> {
        self.api_client.fetch_multiple_pages(channel, pages).await
    }

    /// Calculate the number of pages needed based on terminal height
    ///
    /// # Arguments
    /// * `body_height` - Available height for displaying topics
    /// * `api_limit` - Maximum items per API page (usually 30 or -1 for unlimited)
    ///
    /// # Returns
    /// Number of API pages needed to fill the screen
    ///
    /// # Examples
    /// ```
    /// use hkg::async_services::AsyncTopicService;
    ///
    /// let page_count = AsyncTopicService::calculate_page_count(50, 30);
    /// assert_eq!(page_count, 2);
    ///
    /// let page_count = AsyncTopicService::calculate_page_count(25, 30);
    /// assert_eq!(page_count, 1);
    /// ```
    pub fn calculate_page_count(body_height: usize, api_limit: i32) -> usize {
        if api_limit == -1 {
            1 // API returns all items in one request
        } else {
            let api_limit = api_limit as usize;
            if body_height <= api_limit {
                1
            } else {
                (body_height + api_limit - 1) / api_limit
            }
        }
    }

    /// Validate if a page number is valid
    ///
    /// # Arguments
    /// * `page` - Page number to validate
    /// * `max_page` - Maximum allowed page number
    ///
    /// # Returns
    /// true if the page is valid, false otherwise
    ///
    /// # Examples
    /// ```
    /// use hkg::async_services::AsyncTopicService;
    ///
    /// assert!(AsyncTopicService::is_valid_page(1, 10));
    /// assert!(AsyncTopicService::is_valid_page(10, 10));
    /// assert!(!AsyncTopicService::is_valid_page(0, 10));
    /// assert!(!AsyncTopicService::is_valid_page(11, 10));
    /// ```
    pub fn is_valid_page(page: usize, max_page: usize) -> bool {
        page >= 1 && page <= max_page
    }

    /// Calculate the next page number with boundary checking
    ///
    /// # Arguments
    /// * `current_page` - Current page number
    /// * `max_page` - Maximum allowed page number
    ///
    /// # Returns
    /// Next page number, or max_page if already at the end
    ///
    /// # Examples
    /// ```
    /// use hkg::async_services::AsyncTopicService;
    ///
    /// assert_eq!(AsyncTopicService::next_page(1, 10), 2);
    /// assert_eq!(AsyncTopicService::next_page(9, 10), 10);
    /// assert_eq!(AsyncTopicService::next_page(10, 10), 10);
    /// ```
    pub fn next_page(current_page: usize, max_page: usize) -> usize {
        if current_page >= max_page {
            max_page
        } else {
            current_page + 1
        }
    }

    /// Calculate the previous page number with boundary checking
    ///
    /// # Arguments
    /// * `current_page` - Current page number
    ///
    /// # Returns
    /// Previous page number, or 1 if already at the start
    ///
    /// # Examples
    /// ```
    /// use hkg::async_services::AsyncTopicService;
    ///
    /// assert_eq!(AsyncTopicService::prev_page(2), 1);
    /// assert_eq!(AsyncTopicService::prev_page(1), 1);
    /// ```
    pub fn prev_page(current_page: usize) -> usize {
        if current_page <= 1 {
            1
        } else {
            current_page - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_page_count() {
        // When API returns all items
        assert_eq!(AsyncTopicService::calculate_page_count(50, -1), 1);

        // When body height fits in one page
        assert_eq!(AsyncTopicService::calculate_page_count(25, 30), 1);

        // When body height requires multiple pages
        assert_eq!(AsyncTopicService::calculate_page_count(50, 30), 2);
        assert_eq!(AsyncTopicService::calculate_page_count(60, 30), 2);
        assert_eq!(AsyncTopicService::calculate_page_count(61, 30), 3);
    }

    #[test]
    fn test_is_valid_page() {
        assert!(AsyncTopicService::is_valid_page(1, 10));
        assert!(AsyncTopicService::is_valid_page(5, 10));
        assert!(AsyncTopicService::is_valid_page(10, 10));

        assert!(!AsyncTopicService::is_valid_page(0, 10));
        assert!(!AsyncTopicService::is_valid_page(11, 10));
    }

    #[test]
    fn test_next_page() {
        assert_eq!(AsyncTopicService::next_page(1, 10), 2);
        assert_eq!(AsyncTopicService::next_page(5, 10), 6);
        assert_eq!(AsyncTopicService::next_page(9, 10), 10);
        assert_eq!(AsyncTopicService::next_page(10, 10), 10);
    }

    #[test]
    fn test_prev_page() {
        assert_eq!(AsyncTopicService::prev_page(5), 4);
        assert_eq!(AsyncTopicService::prev_page(2), 1);
        assert_eq!(AsyncTopicService::prev_page(1), 1);
    }
}

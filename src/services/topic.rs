//! Topic service for business logic related to topics
//!
//! This service handles topic-related operations including fetching,
//! pagination, filtering, and state management.

use crate::repository::TopicRepository;
use crate::domain::Topic;
use crate::errors::{AppResult, ApiError};
use log::{info, debug};

/// Service for topic-related business logic
///
/// The `TopicService` encapsulates all business logic related to topics,
/// providing a clean interface for the UI layer. It is generic over
/// any implementation of `TopicRepository`.
///
/// # Examples
/// ```no_run
/// use hkg::services::TopicService;
/// use hkg::repository::HkgoldenTopicRepository;
/// use hkg::api_client::HkgApiClient;
/// use std::sync::Arc;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = HkgApiClient::new()?;
/// let repo = HkgoldenTopicRepository::new(Arc::new(client));
/// let service = TopicService::new(repo);
/// let (topics, max_page) = service.fetch_topics("BW", 1)?;
/// # Ok(())
/// # }
/// ```
pub struct TopicService<R: TopicRepository> {
    repository: R,
}

impl<R: TopicRepository> TopicService<R> {
    /// Create a new topic service
    ///
    /// # Arguments
    /// * `repository` - Repository for topic data access
    ///
    /// # Examples
    /// ```
    /// use hkg::services::TopicService;
    /// use hkg::repository::HkgoldenTopicRepository;
    /// use hkg::api_client::HkgApiClient;
    /// use std::sync::Arc;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = HkgApiClient::new()?;
    /// let repo = HkgoldenTopicRepository::new(Arc::new(client));
    /// let service = TopicService::new(repo);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Fetch topics from a channel
    ///
    /// # Arguments
    /// * `channel` - Channel code (e.g., "BW", "HT")
    /// * `page` - Page number to fetch
    ///
    /// # Returns
    /// Tuple of (list of topics, maximum page number)
    ///
    /// # Errors
    /// Returns `ApiError` if the repository request fails
    ///
    /// # Examples
    /// ```no_run
    /// # use hkg::services::TopicService;
    /// # use hkg::repository::HkgoldenTopicRepository;
    /// # use hkg::api_client::HkgApiClient;
    /// # use std::sync::Arc;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = HkgApiClient::new()?;
    /// let repo = HkgoldenTopicRepository::new(Arc::new(client));
    /// let service = TopicService::new(repo);
    /// let (topics, max_page) = service.fetch_topics("BW", 1)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn fetch_topics(
        &self,
        channel: &str,
        page: i32,
    ) -> AppResult<(Vec<Topic>, i32)> {
        info!("[TopicService] Fetching topics for channel {} page {}",
              channel, page);

        // In the future, we could add:
        // - Caching logic here
        // - Rate limiting
        // - Request deduplication
        // - Background prefetching

        let result = self.repository.fetch_topics(channel, page)
            .map_err(|e| ApiError::ApiResponse { message: e.to_string() })?;

        debug!("[TopicService] Fetched {} topics", result.0.len());

        Ok(result)
    }
}

/// Calculate optimal page count for multi-page fetching
///
/// # Arguments
/// * `body_height` - Number of visible rows in terminal
/// * `api_max_per_page` - Max items per API page (default: 30)
///
/// # Returns
/// Number of API pages to fetch
///
/// # Examples
/// ```
/// use hkg::services::topic::calculate_page_count;
///
/// let pages = calculate_page_count(50, 30);
/// assert_eq!(pages, 2);
/// ```
#[must_use]
pub const fn calculate_page_count(body_height: usize, api_max_per_page: usize) -> usize {
    if body_height <= api_max_per_page {
        1
    } else {
        (body_height + api_max_per_page - 1) / api_max_per_page
    }
}

/// Validate page number
///
/// # Arguments
/// * `page` - Page number to validate
/// * `max_page` - Maximum page number available
///
/// # Returns
/// `true` if page is valid, `false` otherwise
///
/// # Examples
/// ```
/// use hkg::services::topic::is_valid_page;
///
/// assert!(is_valid_page(5, 10));
/// assert!(!is_valid_page(0, 10));
/// assert!(!is_valid_page(15, 10));
/// ```
#[must_use]
pub const fn is_valid_page(page: usize, max_page: usize) -> bool {
    page > 0 && page <= max_page
}

/// Calculate next page number with wrap-around
///
/// # Arguments
/// * `current_page` - Current page number
/// * `max_page` - Maximum page number
///
/// # Returns
/// Next page number (wraps to 1 if at max)
///
/// # Examples
/// ```
/// use hkg::services::topic::next_page;
///
/// assert_eq!(next_page(5, 10), 6);
/// assert_eq!(next_page(10, 10), 1);
/// ```
#[must_use]
pub const fn next_page(current_page: usize, max_page: usize) -> usize {
    if current_page >= max_page {
        1
    } else {
        current_page + 1
    }
}

/// Calculate previous page number with wrap-around
///
/// # Arguments
/// * `current_page` - Current page number
/// * `max_page` - Maximum page number
///
/// # Returns
/// Previous page number (wraps to max if at 1)
///
/// # Examples
/// ```
/// use hkg::services::topic::prev_page;
///
/// assert_eq!(prev_page(5, 10), 4);
/// assert_eq!(prev_page(1, 10), 10);
/// ```
#[must_use]
pub const fn prev_page(current_page: usize, max_page: usize) -> usize {
    if current_page <= 1 {
        max_page
    } else {
        current_page - 1
    }
}

/// Type alias for backward compatibility
///
/// This type alias provides the old non-generic interface for code that
/// hasn't been migrated to use the generic TopicService.
pub type HkgoldenTopicService = TopicService<crate::repository::HkgoldenTopicRepository>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_page_count() {
        assert_eq!(calculate_page_count(30, 30), 1);
        assert_eq!(calculate_page_count(31, 30), 2);
        assert_eq!(calculate_page_count(60, 30), 2);
        assert_eq!(calculate_page_count(90, 30), 3);
    }

    #[test]
    fn test_is_valid_page() {
        assert!(is_valid_page(1, 10));
        assert!(is_valid_page(5, 10));
        assert!(is_valid_page(10, 10));
        assert!(!is_valid_page(0, 10));
        assert!(!is_valid_page(11, 10));
    }

    #[test]
    fn test_next_page() {
        assert_eq!(next_page(5, 10), 6);
        assert_eq!(next_page(9, 10), 10);
        assert_eq!(next_page(10, 10), 1);
        assert_eq!(next_page(1, 10), 2);
    }

    #[test]
    fn test_prev_page() {
        assert_eq!(prev_page(5, 10), 4);
        assert_eq!(prev_page(2, 10), 1);
        assert_eq!(prev_page(1, 10), 10);
        assert_eq!(prev_page(10, 10), 9);
    }
}

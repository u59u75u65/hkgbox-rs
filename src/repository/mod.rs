//! Repository abstraction layer
//!
//! This module defines trait-based repository interfaces for data access.
//! These abstractions allow decoupling business logic from specific API implementations.

pub mod error;
pub mod hkgolden;
pub mod async_hkgolden;
pub mod lihkg;
pub mod mock_lihkg;

pub use error::{RepositoryError, RepositoryResult};
pub use hkgolden::{HkgoldenTopicRepository, HkgoldenThreadRepository};
pub use async_hkgolden::{AsyncHkgoldenTopicRepository, AsyncHkgoldenThreadRepository};
pub use lihkg::{LihkgTopicRepository, LihkgThreadRepository};
pub use mock_lihkg::MockLihkgTopicRepository;

use crate::domain::{Topic, ThreadView};

/// Trait for fetching topic data
///
/// This trait abstracts the data source for topics, allowing different
/// implementations (HKGolden API, LIHKG API, mock for testing, etc.)
pub trait TopicRepository: Send + Sync {
    /// Fetch topics from a channel/forum
    ///
    /// # Arguments
    /// * `channel` - Channel code (e.g., "BW", "HT")
    /// * `page` - Page number to fetch
    ///
    /// # Returns
    /// Vector of topics and the maximum page number
    ///
    /// # Errors
    /// Returns `RepositoryError` if the fetch fails
    fn fetch_topics(
        &self,
        channel: &str,
        page: i32,
    ) -> RepositoryResult<(Vec<Topic>, i32)>;

    /// Get the display title for a channel code
    ///
    /// # Arguments
    /// * `channel_code` - Channel code (e.g., "BW" for HKGolden, "1" for LIHKG)
    ///
    /// # Returns
    /// The display title for the channel, or the original code if not found
    fn get_channel_title(&self, channel_code: &str) -> String;
}

/// Trait for fetching thread data
///
/// This trait abstracts the data source for thread views, allowing different
/// implementations (HKGolden API, LIHKG API, mock for testing, etc.)
pub trait ThreadRepository: Send + Sync {
    /// Fetch a thread view with replies
    ///
    /// # Arguments
    /// * `thread_id` - Thread ID to fetch
    /// * `page` - Page number to fetch
    ///
    /// # Returns
    /// Thread view with main content and replies
    ///
    /// # Errors
    /// Returns `RepositoryError` if the fetch fails
    fn fetch_thread(
        &self,
        thread_id: i32,
        page: i32,
    ) -> RepositoryResult<ThreadView>;
}

/// Async trait for fetching topic data
///
/// This is the async version of `TopicRepository` for non-blocking I/O.
#[async_trait::async_trait]
pub trait AsyncTopicRepository: Send + Sync {
    /// Fetch topics from a channel/forum (async)
    ///
    /// # Arguments
    /// * `channel` - Channel code (e.g., "BW", "HT")
    /// * `page` - Page number to fetch
    ///
    /// # Returns
    /// Vector of topics and the maximum page number
    ///
    /// # Errors
    /// Returns `RepositoryError` if the fetch fails
    async fn fetch_topics(
        &self,
        channel: &str,
        page: i32,
    ) -> RepositoryResult<(Vec<Topic>, i32)>;
}

/// Async trait for fetching thread data
///
/// This is the async version of `ThreadRepository` for non-blocking I/O.
#[async_trait::async_trait]
pub trait AsyncThreadRepository: Send + Sync {
    /// Fetch a thread view with replies (async)
    ///
    /// # Arguments
    /// * `thread_id` - Thread ID to fetch
    /// * `page` - Page number to fetch
    ///
    /// # Returns
    /// Thread view with main content and replies
    ///
    /// # Errors
    /// Returns `RepositoryError` if the fetch fails
    async fn fetch_thread(
        &self,
        thread_id: i32,
        page: i32,
    ) -> RepositoryResult<ThreadView>;
}

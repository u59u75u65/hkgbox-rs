//! Async HKGolden API client
//!
//! This module provides an async client for interacting with the HKGolden API,
//! handling HTTP requests with non-blocking I/O for improved performance.

use reqwest::Client;
use std::time::Duration;

use log::info;

use crate::api_models::*;

/// Type alias for async Results with thread-safe error
pub type AsyncResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Async HTTP client for HKGolden API
///
/// The `AsyncHkgApiClient` provides async methods to fetch data from the HKGolden API,
/// including topic lists and thread views with replies. Uses non-blocking I/O for
/// better concurrency and performance.
///
/// # Examples
/// ```no_run
/// use hkg::async_api_client::AsyncHkgApiClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = AsyncHkgApiClient::new().await?;
///     let topics = client.fetch_topics("BW", 1).await?;
///     Ok(())
/// }
/// ```
pub struct AsyncHkgApiClient {
    client: Client,
    base_url: String,
}

impl AsyncHkgApiClient {
    /// Create a new async HKG API client with default settings
    ///
    /// # Returns
    /// A configured async API client or an error if HTTP client creation fails
    ///
    /// # Examples
    /// ```no_run
    /// use hkg::async_api_client::AsyncHkgApiClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///     let client = AsyncHkgApiClient::new().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new() -> AsyncResult<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;

        Ok(Self {
            client,
            base_url: "https://api.hkgolden.com/v1".to_string(),
        })
    }

    /// Create a new client with custom base URL
    ///
    /// # Arguments
    /// * `base_url` - Custom base URL for the API
    ///
    /// # Returns
    /// A configured async API client
    pub fn with_base_url(base_url: String) -> AsyncResult<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;

        Ok(Self {
            client,
            base_url,
        })
    }

    /// Fetch topic list for a forum (async)
    ///
    /// # Arguments
    /// * `forum` - Forum code (e.g., "BW", "HT")
    /// * `page` - Page number to fetch
    ///
    /// # Returns
    /// API response containing topic list
    ///
    /// # Examples
    /// ```no_run
    /// use hkg::async_api_client::AsyncHkgApiClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let client = AsyncHkgApiClient::new().await?;
    /// let topics = client.fetch_topics("BW", 1).await?;
    /// println!("Fetched {} topics", topics.data.list.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_topics(
        &self,
        forum: &str,
        page: i32,
    ) -> AsyncResult<ApiTopicListResponse> {
        let url = format!(
            "{}/topics/{}/{}?thumb=Y&sort=0&sensormode=Y&filtermodeS=N&hideblock=N&limit=-1",
            self.base_url, forum, page
        );

        info!("[Async API] Fetching topics from: {}", url);

        let response = self.client.get(&url).send().await?;
        info!("[Async API] Got response with status: {}", response.status());

        let json = response.text().await?;
        info!("[Async API] Received JSON data, parsing...");

        let data: ApiTopicListResponse = serde_json::from_str(&json)?;
        info!("[Async API] Successfully parsed {} topics", data.data.list.len());

        Ok(data)
    }

    /// Fetch thread view with replies (async)
    ///
    /// # Arguments
    /// * `thread_id` - Thread ID to fetch
    /// * `page` - Page number to fetch
    ///
    /// # Returns
    /// API response containing thread view and replies
    ///
    /// # Examples
    /// ```no_run
    /// use hkg::async_api_client::AsyncHkgApiClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let client = AsyncHkgApiClient::new().await?;
    /// let thread = client.fetch_thread(8045633, 1).await?;
    /// println!("Thread title: {}", thread.data.thread.title);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_thread(
        &self,
        thread_id: i32,
        page: i32,
    ) -> AsyncResult<ApiThreadViewResponse> {
        let url = format!(
            "{}/view/{}/{}?sensormode=Y&hideblock=N",
            self.base_url, thread_id, page
        );

        info!("[Async API] Fetching thread {} page {}", thread_id, page);

        let response = self.client.get(&url).send().await?;
        let json = response.text().await?;
        let data: ApiThreadViewResponse = serde_json::from_str(&json)?;

        info!("[Async API] Successfully fetched thread with {} replies",
              data.data.replies.len());

        Ok(data)
    }

    /// Fetch multiple pages concurrently (async)
    ///
    /// # Arguments
    /// * `forum` - Forum code
    /// * `pages` - List of page numbers to fetch
    ///
    /// # Returns
    /// Vector of topic lists, one per page
    ///
    /// # Examples
    /// ```no_run
    /// use hkg::async_api_client::AsyncHkgApiClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let client = AsyncHkgApiClient::new().await?;
    /// let pages = vec![1, 2, 3];
    /// let results = client.fetch_multiple_pages("BW", pages).await?;
    /// println!("Fetched {} pages", results.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_multiple_pages(
        &self,
        forum: &str,
        pages: Vec<i32>,
    ) -> AsyncResult<Vec<ApiTopicListResponse>> {
        use tokio::task::JoinSet;

        let mut join_set = JoinSet::new();

        for page in pages {
            let client = self.client.clone();
            let url = format!(
                "{}/topics/{}/{}?thumb=Y&sort=0&sensormode=Y&filtermodeS=N&hideblock=N&limit=-1",
                self.base_url, forum, page
            );

            join_set.spawn(async move {
                info!("[Async API] Fetching page {} concurrently", page);

                let response = client.get(&url).send().await?;
                let json = response.text().await?;
                let data: ApiTopicListResponse = serde_json::from_str(&json)?;

                info!("[Async API] Page {} fetched successfully", page);
                Ok::<_, Box<dyn std::error::Error + Send + Sync>>(data)
            });
        }

        let mut results = Vec::new();
        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(data)) => results.push(data),
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
            }
        }

        Ok(results)
    }
}

impl Default for AsyncHkgApiClient {
    fn default() -> Self {
        // Use tokio runtime block_in_place for Default impl
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async {
                    Self::new().await.expect("Failed to create async HKG API client")
                })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_client_creation() {
        let client = AsyncHkgApiClient::new().await;
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_async_client_with_custom_url() {
        let client = AsyncHkgApiClient::with_base_url("https://test.api.com".to_string());
        assert!(client.is_ok());
        assert_eq!(client.unwrap().base_url, "https://test.api.com");
    }
}

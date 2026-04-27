//! HKGolden API client
//!
//! This module provides a client for interacting with the HKGolden API,
//! handling HTTP requests for fetching topics and thread data.

use reqwest::blocking::Client;
use std::time::Duration;

use log::info;

use crate::api_models::*;

/// HTTP client for HKGolden API
///
/// The `HkgApiClient` provides methods to fetch data from the HKGolden API,
/// including topic lists and thread views with replies.
///
/// # Examples
/// ```
/// use hkg::api_client::HkgApiClient;
///
/// let client = HkgApiClient::new().unwrap();
/// let topics = client.fetch_topics("BW", 1).unwrap();
/// ```
pub struct HkgApiClient {
    client: Client,
    base_url: String,
}

impl HkgApiClient {
    /// Create a new HKG API client with default settings
    ///
    /// # Returns
    /// A configured API client or an error if HTTP client creation fails
    ///
    /// # Examples
    /// ```
    /// use hkg::api_client::HkgApiClient;
    ///
    /// let client = HkgApiClient::new().unwrap();
    /// ```
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;

        Ok(Self {
            client,
            base_url: "https://api.hkgolden.com/v1".to_string(),
        })
    }

    /// Fetch topic list for a forum
    pub fn fetch_topics(
        &self,
        forum: &str,
        page: i32,
    ) -> Result<ApiTopicListResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/topics/{}/{}?thumb=Y&sort=0&sensormode=Y&filtermodeS=N&hideblock=N&limit=-1",
            self.base_url, forum, page
        );

        info!("[API] Fetching topics from: {}", url);

        let response = self.client.get(&url).send()?;
        info!("[API] Got response with status: {}", response.status());

        let json = response.text()?;
        info!("[API] Received JSON data, parsing...");

        let data: ApiTopicListResponse = serde_json::from_str(&json)?;
        info!("[API] Successfully parsed {} topics", data.data.list.len());

        Ok(data)
    }

    /// Fetch thread view with replies
    pub fn fetch_thread(
        &self,
        thread_id: i32,
        page: i32,
    ) -> Result<ApiThreadViewResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/view/{}/{}?sensormode=Y&hideblock=N",
            self.base_url, thread_id, page
        );

        let response = self.client.get(&url).send()?;
        let json = response.text()?;
        let data: ApiThreadViewResponse = serde_json::from_str(&json)?;

        Ok(data)
    }
}

impl Default for HkgApiClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HKG API client")
    }
}

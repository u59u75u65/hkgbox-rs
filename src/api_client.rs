use reqwest::blocking::Client;
use std::time::Duration;
use log::{info, error};

use crate::api_models::*;

pub struct HkgApiClient {
    client: Client,
    base_url: String,
}

impl HkgApiClient {
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

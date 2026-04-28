//! Application context for dependency injection
//!
//! This module provides centralized dependency management through an
//! ApplicationContext that manages shared services and configuration.

use std::sync::Arc;
use crate::config::AppConfig;
use crate::api_client::HkgApiClient;
use crate::services::{HkgoldenTopicService, ChannelService, ImageService};
use crate::repository::HkgoldenTopicRepository;
use crate::state_manager::StateManager;
use std::sync::mpsc::Sender;

/// Application context for dependency injection
///
/// The `ApplicationContext` manages shared dependencies and provides
/// a centralized way to access services and configuration throughout
/// the application.
///
/// # Examples
/// ```
/// use hkg::context::ApplicationContext;
/// use hkg::config::AppConfig;
/// use std::sync::mpsc::channel;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let (tx, _) = channel();
/// let config = AppConfig::new();
/// let ctx = ApplicationContext::new(config, tx)?;
/// # Ok(())
/// # }
/// ```
pub struct ApplicationContext {
    /// Application configuration
    pub config: Arc<AppConfig>,

    /// HKG API client
    pub api_client: Arc<HkgApiClient>,

    /// Topic service
    pub topic_service: Arc<HkgoldenTopicService>,

    /// Channel service
    pub channel_service: Arc<ChannelService>,

    /// Image service
    pub image_service: Arc<ImageService>,

    /// State manager
    pub state_manager: StateManager,
}

impl ApplicationContext {
    /// Create a new application context
    ///
    /// # Arguments
    /// * `config` - Application configuration
    /// * `tx_state` - State change notification channel
    ///
    /// # Returns
    /// A new ApplicationContext or an error if initialization fails
    ///
    /// # Examples
    /// ```
    /// use hkg::context::ApplicationContext;
    /// use hkg::config::AppConfig;
    /// use std::sync::mpsc::channel;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let (tx, _) = channel();
    /// let config = AppConfig::new();
    /// let ctx = ApplicationContext::new(config, tx)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        config: AppConfig,
        tx_state: Sender<(crate::status::Status, crate::status::Status)>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Validate configuration
        config.validate()?;

        // Create API client
        let api_client = Arc::new(HkgApiClient::new()?);

        // Create repository with dependency injection
        let topic_repository = HkgoldenTopicRepository::new(api_client.clone());

        // Create services with dependency injection
        let topic_service = Arc::new(HkgoldenTopicService::new(topic_repository));
        let channel_service = Arc::new(ChannelService::new());
        let image_service = Arc::new(ImageService::new(
            config.image_cache_dir.clone(),
            config.max_image_cache_size_bytes,
        ));

        // Create state manager
        let state_manager = StateManager::new(tx_state);

        Ok(Self {
            config: Arc::new(config),
            api_client,
            topic_service,
            channel_service,
            image_service,
            state_manager,
        })
    }

    /// Create a new application context with default configuration
    ///
    /// # Arguments
    /// * `tx_state` - State change notification channel
    ///
    /// # Returns
    /// A new ApplicationContext with default configuration
    ///
    /// # Examples
    /// ```
    /// use hkg::context::ApplicationContext;
    /// use std::sync::mpsc::channel;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let (tx, _) = channel();
    /// let ctx = ApplicationContext::with_default_config(tx)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_default_config(
        tx_state: Sender<(crate::status::Status, crate::status::Status)>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::new(AppConfig::new(), tx_state)
    }

    /// Get a clone of the API client
    #[must_use]
    pub fn api_client(&self) -> Arc<HkgApiClient> {
        self.api_client.clone()
    }

    /// Get a clone of the topic service
    #[must_use]
    pub fn topic_service(&self) -> Arc<HkgoldenTopicService> {
        self.topic_service.clone()
    }

    /// Get a clone of the channel service
    #[must_use]
    pub fn channel_service(&self) -> Arc<ChannelService> {
        self.channel_service.clone()
    }

    /// Get a clone of the image service
    #[must_use]
    pub fn image_service(&self) -> Arc<ImageService> {
        self.image_service.clone()
    }

    /// Get a reference to the configuration
    #[must_use]
    pub fn config(&self) -> &AppConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn test_with_default_config() {
        let (tx, _) = channel();
        let ctx = ApplicationContext::with_default_config(tx);
        assert!(ctx.is_ok());
    }

    #[test]
    fn test_service_clones() {
        let (tx, _) = channel();
        let ctx = ApplicationContext::with_default_config(tx).unwrap();

        // Verify we can clone services
        let _topic = ctx.topic_service();
        let _channel = ctx.channel_service();
        let _image = ctx.image_service();
        let _api = ctx.api_client();
    }

    #[test]
    fn test_invalid_config() {
        let (tx, _) = channel();
        let mut config = AppConfig::new();
        config.api_timeout_secs = 0; // Invalid

        let ctx = ApplicationContext::new(config, tx);
        assert!(ctx.is_err());
    }
}

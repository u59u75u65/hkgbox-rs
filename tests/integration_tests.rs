//! Integration tests for HKG application
//!
//! These tests verify the integration between different components
//! and ensure the application works as expected.

use hkg::services::{TopicService, ChannelService, ImageService};
use hkg::screen::channel_dialog::ChannelInfo;

#[test]
fn test_channel_service_integration() {
    let service = ChannelService::new();

    // Test that default channels are available
    let channels = service.get_all_channels();
    assert_eq!(channels.len(), 42, "Should have 42 default channels");

    // Test channel lookup
    let bw_channel = service.get_channel("BW");
    assert!(bw_channel.is_some(), "BW channel should exist");
    assert_eq!(bw_channel.unwrap().title, "吹水台");

    // Test channel validation
    assert!(service.is_valid_channel("BW"), "BW should be valid");
    assert!(!service.is_valid_channel("INVALID"), "INVALID should not be valid");

    // Test channel filtering
    let results = service.filter_channels("台");
    assert!(!results.is_empty(), "Should find channels containing '台'");
}

#[test]
fn test_topic_service_calculations() {
    // Test page count calculations
    assert_eq!(TopicService::calculate_page_count(30, 30), 1);
    assert_eq!(TopicService::calculate_page_count(60, 30), 2);
    assert_eq!(TopicService::calculate_page_count(90, 30), 3);

    // Test page validation
    assert!(TopicService::is_valid_page(5, 10));
    assert!(!TopicService::is_valid_page(0, 10));
    assert!(!TopicService::is_valid_page(11, 10));

    // Test page navigation
    assert_eq!(TopicService::next_page(5, 10), 6);
    assert_eq!(TopicService::next_page(10, 10), 1);

    assert_eq!(TopicService::prev_page(5, 10), 4);
    assert_eq!(TopicService::prev_page(1, 10), 10);
}

#[test]
fn test_image_service_validation() {
    let service = ImageService::new(
        std::path::PathBuf::from("cache"),
        1000
    );

    // Test URL validation
    assert!(service.is_valid_image_url("https://example.com/image.jpg"));
    assert!(service.is_valid_image_url("http://example.com/image.png"));
    assert!(!service.is_valid_image_url("ftp://example.com/image.jpg"));
    assert!(!service.is_valid_image_url("not-a-url"));

    // Test image ID extraction
    let url = "https://example.com/images/abc123.jpg";
    let id = service.extract_image_id(url);
    assert!(id.is_ok());
    assert_eq!(id.unwrap(), "abc123");

    // Test cache size calculation
    let size = ImageService::calculate_cache_size(80, 24, 10);
    assert!(size > 0);

    // Test cache clearing logic
    assert!(!service.should_clear_cache(500));
    assert!(service.should_clear_cache(1500));
}

#[test]
fn test_service_layer_composition() {
    // Test that services can work together
    let channel_service = ChannelService::new();
    let image_service = ImageService::new(
        std::path::PathBuf::from("cache"),
        500 * 1024 * 1024 // 500MB
    );

    // Get a channel and process it
    if let Some(channel) = channel_service.get_channel("BW") {
        // Verify channel structure
        assert!(!channel.title.is_empty());
        assert!(!channel.channel.is_empty());

        // Test image service with channel data
        let size = ImageService::calculate_cache_size(80, 24, 1);
        assert!(size > 0);
    }
}

#[test]
fn test_channel_filtering_patterns() {
    let service = ChannelService::new();

    // Test exact match
    let exact_matches = service.filter_channels("吹水台");
    assert!(!exact_matches.is_empty());

    // Test partial match
    let partial_matches = service.filter_channels("台");
    assert!(partial_matches.len() > 10); // Many channels have "台"

    // Test channel code match
    let code_matches = service.filter_channels("BW");
    assert!(!code_matches.is_empty());

    // Test case-insensitive matching
    let lowercase = service.filter_channels("bw");
    assert!(!lowercase.is_empty());

    // Test empty pattern
    let empty = service.filter_channels("");
    assert!(empty.is_empty());
}

#[test]
fn test_pagination_edge_cases() {
    // Test edge cases for pagination calculations
    assert_eq!(TopicService::calculate_page_count(0, 30), 1);
    assert_eq!(TopicService::calculate_page_count(1, 30), 1);
    assert_eq!(TopicService::calculate_page_count(29, 30), 1);
    assert_eq!(TopicService::calculate_page_count(30, 30), 1);
    assert_eq!(TopicService::calculate_page_count(31, 30), 2);

    // Test wrapping behavior
    assert_eq!(TopicService::next_page(1, 1), 1);
    assert_eq!(TopicService::prev_page(1, 1), 1);
}

#[test]
fn test_configuration_validation() {
    use hkg::config::AppConfig;

    // Test default configuration
    let config = AppConfig::new();
    assert!(config.validate().is_ok());
    assert_eq!(config.api_timeout_secs, 10);
    assert_eq!(config.api_topics_per_page, 30);

    // Test invalid configurations
    let mut invalid_config = AppConfig::new();
    invalid_config.api_timeout_secs = 0;
    assert!(invalid_config.validate().is_err());

    invalid_config.api_timeout_secs = 10;
    invalid_config.max_concurrent_requests = 0;
    assert!(invalid_config.validate().is_err());

    invalid_config.max_concurrent_requests = 5;
    invalid_config.api_topics_per_page = 0;
    assert!(invalid_config.validate().is_err());
}

#[test]
fn test_error_types() {
    use hkg::errors::*;

    // Test error creation and display
    let api_err = ApiError::RateLimitExceeded;
    assert_eq!(api_err.to_string(), "Rate limit exceeded");

    let cache_err = CacheError::NotFound {
        key: "test_key".to_string(),
    };
    assert!(cache_err.to_string().contains("test_key"));

    let ui_err = UIError::TerminalSizeError;
    assert!(ui_err.to_string().contains("Terminal") || ui_err.to_string().contains("size"));

    // Test error conversion
    let app_err: AppError = api_err.into();
    assert!(app_err.to_string().contains("API error"));
}

#[test]
fn test_image_url_processing() {
    use hkg::html_parser;

    // Test image URL extraction
    let html = r#"
        <div>
            <img src="https://example.com/image1.jpg" alt="Image 1" />
            <img src="https://example.com/image2.png" alt="Image 2" />
            <p>Some text</p>
            <img src="https://example.com/image3.gif" />
        </div>
    "#;

    let urls = html_parser::extract_image_urls(html);
    assert_eq!(urls.len(), 3);
    assert_eq!(urls[0], "https://example.com/image1.jpg");
    assert_eq!(urls[1], "https://example.com/image2.png");
    assert_eq!(urls[2], "https://example.com/image3.gif");
}

#[test]
fn test_time_conversion() {
    use hkg::time_utils;

    // Test timestamp conversion
    let (date, time) = time_utils::timestamp_to_strings(1609459200000); // 2021-01-01 00:00:00 UTC

    // Verify format
    assert!(date.contains("/")); // DD/MM/YYYY format
    assert!(time.contains(":")); // HH:MM format

    // Test that different timestamps give different results
    let (date2, _) = time_utils::timestamp_to_strings(1609545600000); // 2021-01-02 00:00:00 UTC
    assert_ne!(date, date2);
}

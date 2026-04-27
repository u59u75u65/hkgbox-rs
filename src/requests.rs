//! Request sending utilities for HKG API
//!
//! This module provides functions for sending requests to the background
//! worker threads for fetching topics and thread data.

use crate::resources::*;
use crate::state_manager::StateManager;
use std::sync::mpsc::Sender;

/// Send a page request to the background worker for fetching topics
///
/// # Arguments
/// * `state_manager` - State manager to track request state
/// * `tx_req` - Channel sender for requests
/// * `page` - Page number to fetch
/// * `channel` - Channel code (e.g., "BW", "HT")
///
/// # Returns
/// Status message indicating success or failure
pub fn list_page(
    state_manager: &mut StateManager,
    tx_req: &Sender<ChannelItem>,
    page: usize,
    channel: &str,
) -> String {
    // Calculate how many API pages to fetch based on terminal height
    const API_MAX_PER_PAGE: usize = 30;
    let (_, terminal_height) = termion::terminal_size().expect("Failed to get terminal size");
    // Subtract 3 rows: 1 for header, 1 for separator, 1 for status bar
    let body_height = if terminal_height >= 3 {
        terminal_height as usize - 3
    } else {
        0
    };
    let page_count = if body_height <= API_MAX_PER_PAGE {
        1
    } else {
        (body_height + API_MAX_PER_PAGE - 1) / API_MAX_PER_PAGE
    };

    let ci = ChannelItem {
        extra: Some(ChannelItemType::Index(ChannelIndexItem {
            page,
            channel: channel.to_string(),
            page_count,
        })),
        result: Default::default(),
    };

    let status_message = match tx_req.send(ci) {
        Ok(()) => {
            state_manager.set_web_request(true);
            "SOK".to_string()
        }
        Err(e) => format!("{}:{}", "SFAIL", e).to_string(),
    };

    status_message
}

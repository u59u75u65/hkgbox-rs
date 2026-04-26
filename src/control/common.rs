//! Common utilities for control modules
use crate::resources::*;
use crate::state_manager::StateManager;
use std::sync::mpsc::Sender;

/// Send a page request to the background worker
pub fn send_page_request(
    postid: &str,
    page: usize,
    state_manager: &mut StateManager,
    tx_req: &Sender<ChannelItem>,
) -> String {
    let ci = ChannelItem {
        extra: Some(ChannelItemType::Show(ChannelShowItem {
            postid: postid.to_string(),
            page: page,
        })),
        result: String::from(""),
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

/// Format a page status message for display
pub fn format_page_status(postid: &str, page: usize, status_message: &str) -> String {
    format!("[{}-{}:{}]", postid, page, status_message)
}

/// Send an index page request to the background worker
pub fn send_index_page_request(
    page: usize,
    state_manager: &mut StateManager,
    tx_req: &Sender<ChannelItem>,
    channel: &str,
) -> String {
    send_index_page_request_with_count(page, 1, state_manager, tx_req, channel)
}

/// Send an index page request with page count to the background worker
pub fn send_index_page_request_with_count(
    page: usize,
    page_count: usize,
    state_manager: &mut StateManager,
    tx_req: &Sender<ChannelItem>,
    channel: &str,
) -> String {
    let ci = ChannelItem {
        extra: Some(ChannelItemType::Index(ChannelIndexItem {
            page,
            channel: channel.to_string(),
            page_count,
        })),
        result: String::from(""),
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

/// Format an index page status message for display
pub fn format_index_page_status(page: usize, status_message: &str) -> String {
    format!("[p{}:{}]", page, status_message)
}

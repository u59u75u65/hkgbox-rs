use crate::resources::*;
use crate::resources::common::*;
use crate::caches::common::*;
use crate::api_client::HkgApiClient;
use crate::api_models::*;
use crate::api_utils::*;

use crate::model::{ShowReplyItem, ShowItem, UrlQueryItem};
use log::info;

pub struct ShowResourceApi<'a, T: 'a + Cache> {
    client: HkgApiClient,
    _cache: &'a mut Box<T>,
    pub replies: Vec<ShowReplyItem>,
    pub title: String,
    pub total_replies: i32,
}

impl<'a, T: 'a + Cache> ShowResourceApi<'a, T> {
    pub fn new(cache: &'a mut Box<T>) -> Self {
        ShowResourceApi {
            client: HkgApiClient::new().expect("Failed to create API client"),
            _cache: cache,
            replies: Vec::new(),
            title: String::new(),
            total_replies: 0,
        }
    }
}

impl<'a, T: 'a + Cache> Resource for ShowResourceApi<'a, T> {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem {
        info!("[show_resource_api] Fetching from API");

        // Extract thread ID from item
        let (thread_id, page) = match &item.extra {
            Some(ChannelItemType::Show(show_item)) => {
                (show_item.postid.parse().unwrap_or(0), show_item.page)
            }
            _ => return ChannelItem {
                extra: Some(ChannelItemType::Show(ChannelShowItem::default())),
                result: "Invalid thread ID".to_string(),
            },
        };

        // Call API
        match self.client.fetch_thread(thread_id, page as i32) {
            Ok(response) => {
                info!("Got thread {} with {} replies", thread_id, response.data.total_replies);

                // Store metadata
                self.title = response.data.title.clone();
                self.total_replies = response.data.total_replies;

                // Parse main thread content if exists
                let main_content = if !response.data.content.is_empty() {
                    Some(self.convert_main_content(&response.data))
                } else {
                    None
                };

                // Convert API replies to existing ShowReplyItem format
                self.replies = response.data.replies
                    .into_iter()
                    .map(|api_reply| self.convert_reply(api_reply))
                    .collect();

                // Create ShowItem
                let show_item = ShowItem {
                    title: self.title.clone(),
                    url_query: UrlQueryItem {
                        channel: "BW".to_string(),
                        message: thread_id.to_string(),
                    },
                    page: page,
                    max_page: ((self.total_replies as f32) / 20.0).ceil() as usize,
                    reply_count: self.total_replies.to_string(),
                    main_content: main_content,
                    replies: self.replies.clone(),
                };

                ChannelItem {
                    extra: Some(ChannelItemType::ShowWithData(show_item)),
                    result: String::new(),
                }
            }
            Err(e) => {
                ChannelItem {
                    extra: Some(ChannelItemType::Show(ChannelShowItem::default())),
                    result: format!("Error: {}", e),
                }
            }
        }
    }
}

impl<'a, T: 'a + Cache> ShowResourceApi<'a, T> {
    fn convert_reply(&self, api_reply: ApiReply) -> ShowReplyItem {
        // Parse HTML content to extract nodes (images, text, etc.)
        let body_nodes = parse_html_content(&api_reply.content);

        // Convert timestamp
        let (date, time) = timestamp_to_strings(api_reply.reply_date);
        let published_at = format!("{} {}", date, time);

        ShowReplyItem {
            userid: api_reply.author_id.to_string(),
            username: api_reply.author_name.clone(),
            content: api_reply.content.clone(),
            body: body_nodes,
            published_at: published_at,
        }
    }

    fn convert_main_content(&self, api_data: &ApiThreadViewData) -> ShowReplyItem {
        // Parse HTML content to extract nodes (images, text, etc.)
        let body_nodes = parse_html_content(&api_data.content);

        // Use thread message_date as published time for main content
        let (date, time) = timestamp_to_strings(api_data.message_date);
        let published_at = format!("{} {}", date, time);

        ShowReplyItem {
            userid: api_data.author_id.to_string(),
            username: api_data.author_name.clone(),
            content: api_data.content.clone(),
            body: body_nodes,
            published_at: published_at,
        }
    }

    pub fn get_replies(&self, thread_id: i32, page: i32) -> Result<Vec<ShowReplyItem>, Box<dyn std::error::Error>> {
        let response = self.client.fetch_thread(thread_id, page)?;

        let replies: Vec<ShowReplyItem> = response.data.replies
            .into_iter()
            .map(|api_reply| self.convert_reply(api_reply))
            .collect();

        Ok(replies)
    }
}


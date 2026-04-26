use crate::resources::*;
use crate::resources::common::*;
use crate::caches::common::*;
use crate::api_client::HkgApiClient;
use crate::api_models::*;
use crate::api_utils::*;

use crate::model::{ListTopicItem, ListTopicTitleItem, UrlQueryItem, ListTopicAuthorItem};

pub struct IndexResource<'a, T: 'a + Cache> {
    client: HkgApiClient,
    _cache: &'a mut Box<T>,
    forum: String,
    pub list_items: Vec<ListTopicItem>,
}

impl<'a, T: 'a + Cache> IndexResource<'a, T> {
    pub fn new(cache: &'a mut Box<T>) -> Self {
        IndexResource {
            client: HkgApiClient::new().expect("Failed to create API client"),
            _cache: cache,
            forum: "BW".to_string(),  // Default forum
            list_items: Vec::new(),
        }
    }
}

impl<'a, T: 'a + Cache> IndexResource<'a, T> {
    pub fn set_forum(&mut self, forum: String) {
        self.forum = forum;
    }
}

impl<'a, T: 'a + Cache> Resource for IndexResource<'a, T> {
    fn fetch(&mut self, _item: &ChannelItem) -> ChannelItem {
        log::info!("[IndexResource] Starting fetch for forum: {}", self.forum);

        // Call API instead of scraping HTML
        match self.client.fetch_topics(&self.forum, 1) {
            Ok(response) => {
                log::info!("[IndexResource] Got {} topics from API", response.data.list.len());

                // Convert API topics to existing ListTopicItem format
                self.list_items = response.data.list
                    .into_iter()
                    .map(|api_topic| self.convert_to_list_item(api_topic))
                    .collect();

                log::info!("[IndexResource] Converted {} topics", self.list_items.len());

                ChannelItem {
                    extra: Some(ChannelItemType::IndexWithData(self.list_items.clone())),
                    result: String::new(),
                }
            }
            Err(e) => {
                log::error!("[IndexResource] API error: {}", e);
                ChannelItem {
                    extra: Some(ChannelItemType::Index(ChannelIndexItem {})),
                    result: format!("Error: {}", e),
                }
            }
        }
    }
}

impl<'a, T: 'a + Cache> IndexResource<'a, T> {
    fn convert_to_list_item(&self, api_topic: ApiTopic) -> ListTopicItem {
        // Convert timestamps
        let (date, time) = timestamp_to_strings(api_topic.last_reply_date);

        ListTopicItem {
            title: ListTopicTitleItem {
                url: format!("/messages/{}.aspx", api_topic.id),
                url_query: UrlQueryItem {
                    channel: crate::model::ChannelId::new(api_topic.forum.clone()),
                    message: crate::model::MessageId::new(api_topic.id.to_string()),
                },
                text: api_topic.title.clone(),
                num_of_pages: api_topic.total_page as usize,
            },
            author: ListTopicAuthorItem {
                url: format!("/ProfilePage.aspx?userid={}", api_topic.author_id),
                name: api_topic.author_name.clone(),
            },
            last_replied_date: date,
            last_replied_time: time,
            reply_count: api_topic.total_replies.to_string(),
            rating: api_topic.rating.to_string(),
        }
    }

    pub fn get_list_items(&self) -> Vec<ListTopicItem> {
        self.list_items.clone()
    }
}

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
    page: usize,
    page_count: usize,
    max_page: usize,
    pub list_items: Vec<ListTopicItem>,
}

impl<'a, T: 'a + Cache> IndexResource<'a, T> {
    pub fn new(cache: &'a mut Box<T>) -> Self {
        IndexResource {
            client: HkgApiClient::new().expect("Failed to create API client"),
            _cache: cache,
            forum: "BW".to_string(),  // Default forum
            page: 1,
            page_count: 1,
            max_page: 1,
            list_items: Vec::new(),
        }
    }
}

impl<'a, T: 'a + Cache> IndexResource<'a, T> {
    pub fn set_forum(&mut self, forum: String) {
        self.forum = forum;
    }

    pub fn set_page(&mut self, page: usize) {
        self.page = page;
    }

    pub fn set_page_count(&mut self, page_count: usize) {
        self.page_count = page_count;
    }

    pub fn get_page(&self) -> usize {
        self.page
    }

    pub fn get_max_page(&self) -> usize {
        self.max_page
    }
}

impl<'a, T: 'a + Cache> Resource for IndexResource<'a, T> {
    fn fetch(&mut self, _item: &ChannelItem) -> ChannelItem {
        log::info!("[IndexResource] Starting fetch for forum: {}, page: {}, page_count: {}",
                  self.forum, self.page, self.page_count);

        self.list_items.clear();
        let mut current_api_page = self.page;
        let mut fetched_pages = 0;
        let mut max_page_from_api = 0;

        // Fetch multiple pages if page_count > 1
        while fetched_pages < self.page_count {
            match self.client.fetch_topics(&self.forum, current_api_page as i32) {
                Ok(response) => {
                    log::info!("[IndexResource] Got {} topics from API page {}",
                             response.data.list.len(), current_api_page);

                    // Update max_page from first API response
                    if fetched_pages == 0 {
                        max_page_from_api = response.data.max_page as usize;
                        self.max_page = max_page_from_api;
                    }

                    // Convert and append topics
                    let page_items: Vec<ListTopicItem> = response.data.list
                        .into_iter()
                        .map(|api_topic| self.convert_to_list_item(api_topic))
                        .collect();

                    self.list_items.extend(page_items);
                    fetched_pages += 1;
                    current_api_page += 1;

                    // Stop if we've reached the last page
                    if current_api_page > max_page_from_api {
                        log::info!("[IndexResource] Reached max API page {}", max_page_from_api);
                        break;
                    }
                }
                Err(e) => {
                    log::error!("[IndexResource] API error on page {}: {}", current_api_page, e);
                    // Return what we have so far, or error if we have nothing
                    if self.list_items.is_empty() {
                        return ChannelItem {
                            extra: Some(ChannelItemType::Index(ChannelIndexItem {
                                page: self.page,
                                channel: self.forum.clone(),
                                page_count: self.page_count,
                            })),
                            result: format!("Error: {}", e),
                        };
                    }
                    break;
                }
            }
        }

        log::info!("[IndexResource] Total fetched {} topics from {} API pages (requested {} pages)",
                 self.list_items.len(), fetched_pages, self.page_count);

        ChannelItem {
            extra: Some(ChannelItemType::IndexWithPageData(
                self.list_items.clone(),
                self.page,
                self.max_page,
                self.forum.clone()
            )),
            result: String::new(),
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

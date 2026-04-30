use crate::resources::*;
use crate::resources::common::*;
use crate::caches::common::*;
use crate::api_client::HkgApiClient;
use crate::api_models::*;
use crate::api_utils::*;
use crate::cli::ForumService;
use crate::repository::{TopicRepository, HkgoldenTopicRepository, LihkgTopicRepository, MockLihkgTopicRepository};
use crate::domain::Topic;

use crate::model::{ListTopicItem, ListTopicTitleItem, UrlQueryItem, ListTopicAuthorItem};
use std::sync::Arc;

pub struct IndexResource<'a, T: 'a + Cache> {
    repository: Option<Box<dyn TopicRepository>>,
    hkg_client: Option<Arc<HkgApiClient>>,
    _cache: &'a mut Box<T>,
    forum: String,
    page: usize,
    page_count: usize,
    max_page: usize,
    pub list_items: Vec<ListTopicItem>,
}

impl<'a, T: 'a + Cache> IndexResource<'a, T> {
    pub fn new(cache: &'a mut Box<T>) -> Self {
        let hkg_client = Arc::new(HkgApiClient::new().expect("Failed to create API client"));
        IndexResource {
            repository: Some(Box::new(HkgoldenTopicRepository::new(hkg_client.clone()))),
            hkg_client: Some(hkg_client),
            _cache: cache,
            forum: "BW".to_string(),
            page: 1,
            page_count: 1,
            max_page: 1,
            list_items: Vec::new(),
        }
    }

    /// Set the forum service to use
    pub fn set_service(&mut self, service: ForumService) {
        match service {
            ForumService::Hkgolden => {
                // HKGolden repository is created in constructor, no need to recreate
                log::info!("[IndexResource] Using HKGolden service");
            }
            ForumService::Lihkg => {
                // LIHKG repository will be created in set_forum with the correct channel
                log::info!("[IndexResource] Using LIHKG service (repository will be created in set_forum)");
            }
        }
    }

    /// Fall back to mock LIHKG repository when real API fails
    pub fn use_mock_lihkg(&mut self) {
        let cat_id = self.forum.parse().unwrap_or(1);
        log::warn!("[IndexResource] Falling back to mock LIHKG repository with cat_id: {}", cat_id);
        self.repository = Some(Box::new(MockLihkgTopicRepository::new(cat_id)));
    }
}

impl<'a, T: 'a + Cache> IndexResource<'a, T> {
    pub fn set_forum(&mut self, forum: String) {
        self.forum = forum.clone();
        // Create/update repository based on forum type
        // Numeric channels are LIHKG, alphabetic are HKGolden
        if let Ok(cat_id) = forum.parse::<i32>() {
            // LIHKG: create repository with specific cat_id
            log::info!("[IndexResource] Creating LIHKG repository with cat_id: {} for forum: {}", cat_id, forum);
            self.repository = Some(Box::new(LihkgTopicRepository::new(cat_id)
                .expect("Failed to create LIHKG repository")));
        } else {
            // HKGolden: repository already created in set_service or new()
            log::info!("[IndexResource] Using HKGolden repository for forum: {}", forum);
        }
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

        // Safety limit: prevent infinite loops
        let max_iterations = self.page_count.min(100);  // Cap at 100 pages max
        let mut iterations = 0;

        // Safety limit: prevent unbounded memory growth
        let max_total_items = 10000;  // Cap at 10,000 topics max

        // Fetch multiple pages if page_count > 1
        while iterations < max_iterations {
            let repo = self.repository.as_ref().expect("Repository not initialized");

            let fetch_result = match repo.fetch_topics(&self.forum, current_api_page as i32) {
                Ok((topics, max_page)) => {
                    log::info!("[IndexResource] Got {} topics from repository page {}",
                             topics.len(), current_api_page);

                    // Update max_page from first API response
                    if fetched_pages == 0 {
                        max_page_from_api = max_page as usize;
                        self.max_page = max_page_from_api;
                    }

                    Ok((topics, max_page))
                }
                Err(e) => {
                    log::error!("[IndexResource] Repository error on page {}: {}", current_api_page, e);

                    // Check if it's a Cloudflare error and fall back to mock
                    if e.to_string().contains("Cloudflare") {
                        log::warn!("[IndexResource] API blocked by Cloudflare, falling back to mock data");

                        // Switch to mock repository and retry this page
                        self.use_mock_lihkg();
                        let mock_repo = self.repository.as_ref().expect("Mock repository not initialized");
                        match mock_repo.fetch_topics(&self.forum, current_api_page as i32) {
                            Ok((topics, max_page)) => {
                                log::info!("[IndexResource] Got {} topics from mock repository page {}",
                                         topics.len(), current_api_page);

                                if fetched_pages == 0 {
                                    max_page_from_api = max_page as usize;
                                    self.max_page = max_page_from_api;
                                }

                                Ok((topics, max_page))
                            }
                            Err(mock_e) => {
                                log::error!("[IndexResource] Mock repository also failed: {}", mock_e);
                                Err(format!("API unavailable: {}. Mock also failed: {}", e, mock_e))
                            }
                        }
                    } else {
                        Err(e.to_string())
                    }
                }
            };

            match fetch_result {
                Ok((topics, _max_page)) => {
                    // Convert domain topics to list items
                    let page_items: Vec<ListTopicItem> = topics
                        .into_iter()
                        .map(|topic| self.convert_domain_to_list_item(topic))
                        .collect();

                    // Safety check: prevent unbounded memory growth
                    if self.list_items.len() + page_items.len() > max_total_items {
                        log::warn!("[IndexResource] Reached max total items limit ({}), stopping fetch", max_total_items);
                        self.list_items.extend(page_items.into_iter().take(max_total_items.saturating_sub(self.list_items.len())));
                        break;
                    }

                    self.list_items.extend(page_items);
                    iterations += 1;
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
    /// Convert domain Topic to ListTopicItem (for repository-based services)
    fn convert_domain_to_list_item(&self, topic: Topic) -> ListTopicItem {
        // Convert timestamps
        let timestamp = topic.last_reply_date.unwrap_or(topic.message_date);
        let (date, time) = timestamp_to_strings(timestamp);

        ListTopicItem {
            title: ListTopicTitleItem {
                url: format!("/messages/{}.aspx", topic.id),
                url_query: UrlQueryItem {
                    channel: crate::model::ChannelId::new(topic.forum.clone()),
                    message: crate::model::MessageId::new(topic.id.to_string()),
                },
                text: topic.title.clone(),
                num_of_pages: topic.total_page as usize,
            },
            author: ListTopicAuthorItem {
                url: format!("/ProfilePage.aspx?userid={}", topic.author_id),
                name: topic.author_name.clone(),
            },
            last_replied_date: date,
            last_replied_time: time,
            reply_count: topic.total_replies.to_string(),
            rating: topic.rating.map_or("0".to_string(), |r| r.to_string()),
        }
    }

    /// Convert API topic to ListTopicItem (for direct API client)
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

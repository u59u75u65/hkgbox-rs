use crate::resources::*;
use crate::resources::common::*;
use crate::caches::common::*;
use crate::api_client::HkgApiClient;
use crate::api_models::*;
use crate::api_utils::*;
use crate::cli::ForumService;
use crate::repository::{TopicRepository, LihkgTopicRepository, MockLihkgTopicRepository};
use crate::domain::Topic;

use crate::model::{ListTopicItem, ListTopicTitleItem, UrlQueryItem, ListTopicAuthorItem};

// Enum to hold either real or mock LIHKG repository
enum LihkgRepositoryHolder {
    Real(LihkgTopicRepository),
    Mock(MockLihkgTopicRepository),
}

impl LihkgRepositoryHolder {
    fn fetch_topics(&self, channel: &str, page: i32) -> Result<(Vec<Topic>, i32), Box<dyn std::error::Error>> {
        match self {
            LihkgRepositoryHolder::Real(repo) => {
                repo.fetch_topics(channel, page)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            }
            LihkgRepositoryHolder::Mock(repo) => {
                repo.fetch_topics(channel, page)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            }
        }
    }
}

pub struct IndexResource<'a, T: 'a + Cache> {
    service: ForumService,
    hkg_client: Option<HkgApiClient>,
    lihkg_repo: Option<LihkgRepositoryHolder>,
    use_mock_lihkg: bool,
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
            service: ForumService::Hkgolden,
            hkg_client: Some(HkgApiClient::new().expect("Failed to create API client")),
            lihkg_repo: None,
            use_mock_lihkg: false,
            _cache: cache,
            forum: "BW".to_string(),  // Default forum
            page: 1,
            page_count: 1,
            max_page: 1,
            list_items: Vec::new(),
        }
    }

    /// Set the forum service to use
    pub fn set_service(&mut self, service: ForumService) {
        self.service = service;

        // Initialize the appropriate client/repository
        match service {
            ForumService::Hkgolden => {
                if self.hkg_client.is_none() {
                    self.hkg_client = Some(HkgApiClient::new().expect("Failed to create API client"));
                }
                self.lihkg_repo = None;
                self.use_mock_lihkg = false;
            }
            ForumService::Lihkg => {
                if self.lihkg_repo.is_none() {
                    // Map channel to LIHKG category ID
                    let cat_id = self.map_channel_to_cat_id(&self.forum);
                    self.lihkg_repo = Some(LihkgRepositoryHolder::Real(
                        LihkgTopicRepository::new(cat_id)
                            .expect("Failed to create LIHKG repository")
                    ));
                }
                self.hkg_client = None;
                self.use_mock_lihkg = false;
            }
        }
    }

    /// Fall back to mock LIHKG repository when real API fails
    pub fn use_mock_lihkg(&mut self) {
        if self.service == ForumService::Lihkg && !self.use_mock_lihkg {
            log::warn!("[IndexResource] Falling back to mock LIHKG repository");
            let cat_id = self.map_channel_to_cat_id(&self.forum);
            log::info!("[IndexResource] Creating mock LIHKG repository with cat_id: {} for channel: {}", cat_id, self.forum);
            self.lihkg_repo = Some(LihkgRepositoryHolder::Mock(
                MockLihkgTopicRepository::new(cat_id)
            ));
            self.use_mock_lihkg = true;
        }
    }

    /// Map channel codes to LIHKG category IDs
    /// For LIHKG: channels are already numeric (e.g., "1", "5", "22")
    /// For HKGolden: maps traditional channel codes to LIHKG equivalents
    fn map_channel_to_cat_id(&self, channel: &str) -> i32 {
        // First, try to parse as numeric (LIHKG channels)
        if let Ok(cat_id) = channel.parse::<i32>() {
            return cat_id;
        }

        // Fallback: map HKGolden channel codes to LIHKG category IDs
        match channel {
            "BW" => 1,    // 吹水台
            "HT" => 2,    // 高登熱 -> 熱門
            "NW" => 3,    // 最新 -> 最新
            "CA" => 5,    // 時事台
            "FN" => 15,   // 財經台
            _ => 1,       // Default to 吹水台
        }
    }
}

impl<'a, T: 'a + Cache> IndexResource<'a, T> {
    pub fn set_forum(&mut self, forum: String) {
        self.forum = forum;

        // Re-initialize LIHKG repository if service is LIHKG
        if self.service == ForumService::Lihkg {
            let cat_id = self.map_channel_to_cat_id(&self.forum);
            log::info!("[IndexResource] Updating LIHKG repository with cat_id: {} for channel: {}", cat_id, self.forum);

            self.lihkg_repo = Some(LihkgRepositoryHolder::Real(
                LihkgTopicRepository::new(cat_id)
                    .expect("Failed to create LIHKG repository")
            ));
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
        log::info!("[IndexResource] Starting fetch for forum: {}, service: {:?}, page: {}, page_count: {}",
                  self.forum, self.service, self.page, self.page_count);

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
            let fetch_result = match self.service {
                ForumService::Hkgolden => {
                    // Use HKGolden API client
                    let client = self.hkg_client.as_ref().expect("HKGolden client not initialized");
                    match client.fetch_topics(&self.forum, current_api_page as i32) {
                        Ok(response) => {
                            log::info!("[IndexResource] Got {} topics from HKGolden API page {}",
                                     response.data.list.len(), current_api_page);

                            // Update max_page from first API response
                            if fetched_pages == 0 {
                                max_page_from_api = response.data.max_page as usize;
                                self.max_page = max_page_from_api;
                            }

                            // Convert API topics to domain topics, then to list items
                            let topics: Vec<Topic> = response.data.list
                                .into_iter()
                                .map(|api_topic| crate::api_models::api_topic_to_domain(api_topic))
                                .collect();

                            Ok((topics, max_page_from_api as i32))
                        }
                        Err(e) => {
                            log::error!("[IndexResource] HKGolden API error on page {}: {}", current_api_page, e);
                            Err(e.to_string())
                        }
                    }
                }
                ForumService::Lihkg => {
                    // Use LIHKG repository (or mock if fallback triggered)
                    let repo = self.lihkg_repo.as_ref().expect("LIHKG repository not initialized");
                    match repo.fetch_topics(&self.forum, current_api_page as i32) {
                        Ok((topics, max_page)) => {
                            log::info!("[IndexResource] Got {} topics from LIHKG API page {}",
                                     topics.len(), current_api_page);

                            // Update max_page from first API response
                            if fetched_pages == 0 {
                                max_page_from_api = max_page as usize;
                                self.max_page = max_page_from_api;
                            }

                            Ok((topics, max_page))
                        }
                        Err(e) => {
                            log::error!("[IndexResource] LIHKG API error on page {}: {}", current_api_page, e);

                            // Check if it's a Cloudflare error and fall back to mock
                            if !self.use_mock_lihkg && e.to_string().contains("Cloudflare") {
                                log::warn!("[IndexResource] LIHKG API blocked by Cloudflare, falling back to mock data");
                                log::warn!("[IndexResource] The mock LIHKG repository returns sample data for demonstration.");

                                // Switch to mock repository and retry this page
                                self.use_mock_lihkg();
                                let mock_repo = self.lihkg_repo.as_ref().expect("Mock LIHKG repository not initialized");
                                match mock_repo.fetch_topics(&self.forum, current_api_page as i32) {
                                    Ok((topics, max_page)) => {
                                        log::info!("[IndexResource] Got {} topics from mock LIHKG repository page {}",
                                                 topics.len(), current_api_page);

                                        if fetched_pages == 0 {
                                            max_page_from_api = max_page as usize;
                                            self.max_page = max_page_from_api;
                                        }

                                        Ok((topics, max_page))
                                    }
                                    Err(mock_e) => {
                                        log::error!("[IndexResource] Mock LIHKG repository also failed: {}", mock_e);
                                        Err(format!("LIHKG API unavailable: {}. Mock also failed: {}", e, mock_e))
                                    }
                                }
                            } else {
                                Err(e.to_string())
                            }
                        }
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

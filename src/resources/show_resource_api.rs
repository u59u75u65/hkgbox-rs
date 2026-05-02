use crate::resources::*;
use crate::resources::common::*;
use crate::caches::common::*;
use crate::api_client::HkgApiClient;
use crate::api_models::*;
use crate::api_utils::*;
use crate::cli::ForumService;
use crate::repository::{ThreadRepository, HkgoldenThreadRepository, LihkgThreadRepository};
use crate::domain::{ThreadView, Reply};
use crate::parser::{ContentParser, HtmlContentParser};

use crate::model::{ShowReplyItem, ShowItem, UrlQueryItem};
use log::info;
use std::sync::Arc;

// Enum to hold either HKGolden or LIHKG thread repository
enum ThreadRepositoryHolder {
    Hkgolden(HkgoldenThreadRepository),
    Lihkg(LihkgThreadRepository),
}

impl ThreadRepositoryHolder {
    fn fetch_thread(&self, thread_id: i32, page: i32) -> Result<ThreadView, Box<dyn std::error::Error>> {
        match self {
            ThreadRepositoryHolder::Hkgolden(repo) => {
                // Fetch from HKGolden repository
                repo.fetch_thread(thread_id, page)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            }
            ThreadRepositoryHolder::Lihkg(repo) => {
                // Fetch from LIHKG repository
                repo.fetch_thread(thread_id, page)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            }
        }
    }
}

pub struct ShowResourceApi<'a, T: 'a + Cache> {
    service: ForumService,
    thread_repo: Option<ThreadRepositoryHolder>,
    _cache: &'a mut Box<T>,
    pub replies: Vec<ShowReplyItem>,
    pub title: String,
    pub total_replies: i32,
    parser: Arc<dyn ContentParser>,
}

impl<'a, T: 'a + Cache> ShowResourceApi<'a, T> {
    pub fn new(cache: &'a mut Box<T>) -> Self {
        let hkg_client = Arc::new(HkgApiClient::new().expect("Failed to create API client"));
        ShowResourceApi {
            service: ForumService::Hkgolden,
            thread_repo: Some(ThreadRepositoryHolder::Hkgolden(
                HkgoldenThreadRepository::new(hkg_client)
            )),
            _cache: cache,
            replies: Vec::new(),
            title: String::new(),
            total_replies: 0,
            parser: Arc::new(HtmlContentParser::new()),
        }
    }

    /// Set the forum service
    pub fn set_service(&mut self, service: ForumService) {
        self.service = service;

        // Initialize the appropriate repository
        match service {
            ForumService::Hkgolden => {
                let hkg_client = Arc::new(HkgApiClient::new().expect("Failed to create API client"));
                self.thread_repo = Some(ThreadRepositoryHolder::Hkgolden(
                    HkgoldenThreadRepository::new(hkg_client)
                ));
            }
            ForumService::Lihkg => {
                self.thread_repo = Some(ThreadRepositoryHolder::Lihkg(
                    LihkgThreadRepository::new().expect("Failed to create LIHKG repository")
                ));
            }
        }
    }

    /// Create a new ShowResourceApi with a custom parser
    ///
    /// This allows dependency injection for testing or using alternative parsers
    pub fn with_parser(cache: &'a mut Box<T>, parser: Arc<dyn ContentParser>) -> Self {
        let hkg_client = Arc::new(HkgApiClient::new().expect("Failed to create API client"));
        ShowResourceApi {
            service: ForumService::Hkgolden,
            thread_repo: Some(ThreadRepositoryHolder::Hkgolden(
                HkgoldenThreadRepository::new(hkg_client)
            )),
            _cache: cache,
            replies: Vec::new(),
            title: String::new(),
            total_replies: 0,
            parser,
        }
    }
}

impl<'a, T: 'a + Cache> Resource for ShowResourceApi<'a, T> {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem {
        info!("[show_resource_api] Fetching from API, service: {:?}", self.service);

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

        let thread_repo = self.thread_repo.as_ref().expect("Thread repository not initialized");

        // Call appropriate API based on service
        match thread_repo.fetch_thread(thread_id, page as i32) {
            Ok(thread_view) => {
                info!("Got thread {} with {} replies", thread_id, thread_view.replies.len());

                // Store metadata
                self.title = thread_view.title.clone();
                self.total_replies = thread_view.total_replies;

                // Determine channel based on service
                let channel = match self.service {
                    ForumService::Hkgolden => "BW".to_string(),
                    ForumService::Lihkg => "1".to_string(), // LIHKG category 1 (吹水台)
                };

                // Parse main thread content if exists
                let main_content = if !thread_view.content.is_empty() {
                    Some(self.convert_domain_main_content(&thread_view))
                } else {
                    None
                };

                // Convert domain ThreadView to existing ShowReplyItem format
                self.replies = self.convert_domain_thread_view_to_show_items(&thread_view);

                // Create ShowItem
                let show_item = ShowItem {
                    title: self.title.clone(),
                    url_query: UrlQueryItem {
                        channel: crate::model::ChannelId::new(channel),
                        message: crate::model::MessageId::new(thread_id.to_string()),
                    },
                    page: page,
                    max_page: thread_view.total_page as usize,
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
    /// Convert domain ThreadView main content to ShowReplyItem format
    fn convert_domain_main_content(&self, thread_view: &ThreadView) -> ShowReplyItem {
        // Parse HTML content using injected parser
        let content_nodes = self.parser.parse_content(&thread_view.content)
            .unwrap_or_default();

        // Convert ContentNode to NodeType for UI compatibility
        let body_nodes: Vec<crate::reply_model::NodeType> = content_nodes
            .into_iter()
            .map(|node| node.into())
            .collect();

        // Convert timestamp
        let (date, time) = timestamp_to_strings(thread_view.message_date);
        let published_at = format!("{} {}", date, time);

        ShowReplyItem {
            userid: thread_view.author_id.to_string(),
            username: thread_view.author_name.clone(),
            content: thread_view.content.clone(),
            body: body_nodes,
            published_at,
        }
    }

    /// Convert domain ThreadView to ShowReplyItem format
    fn convert_domain_thread_view_to_show_items(&self, thread_view: &ThreadView) -> Vec<ShowReplyItem> {
        thread_view.replies
            .iter()
            .map(|domain_reply| {
                // Parse HTML content using injected parser
                let content_nodes = self.parser.parse_content(&domain_reply.content)
                    .unwrap_or_default();

                // Convert ContentNode to NodeType for UI compatibility
                let body_nodes: Vec<crate::reply_model::NodeType> = content_nodes
                    .into_iter()
                    .map(|node| node.into())
                    .collect();

                // Convert timestamp
                let (date, time) = timestamp_to_strings(domain_reply.reply_date);
                let published_at = format!("{} {}", date, time);

                ShowReplyItem {
                    userid: domain_reply.author_id.to_string(),
                    username: domain_reply.author_name.clone(),
                    content: domain_reply.content.clone(),
                    body: body_nodes,
                    published_at,
                }
            })
            .collect()
    }
}

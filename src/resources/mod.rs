pub mod common;
pub mod index_resource;
pub mod index_resource_api;
pub mod show_resource;
pub mod show_resource_api;
pub mod image_resource;
pub mod default_resource;
pub mod web_resource;

use std::default::Default;

#[derive(Debug)]
#[derive(Clone)]
pub enum ChannelItemType {
    Show(ChannelShowItem),
    Index(ChannelIndexItem),
    Image(ChannelImageItem),
    IndexWithData(Vec<crate::model::ListTopicItem>),
    IndexWithPageData(Vec<crate::model::ListTopicItem>, usize, usize, String, String), // items, current_page, max_page, channel, channel_title
    ShowWithData(crate::model::ShowItem),
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
pub struct ChannelIndexItem {
    pub page: usize,
    pub channel: String,
    pub page_count: usize,  // Number of API pages to fetch (for multi-page fetching)
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
pub struct ChannelShowItem {
    pub postid: String,
    pub page: usize,
}

#[derive(Debug)]
#[derive(Default)]
pub struct ChannelItem {
    pub extra: Option<ChannelItemType>,
    pub result: String
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
pub struct ChannelImageItem {
    pub url: String,
    pub bytes: Vec<u8>,
    pub from_cache: bool
}

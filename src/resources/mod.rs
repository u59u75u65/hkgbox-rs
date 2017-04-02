pub mod common;
pub mod index_resource;
pub mod show_resource;
pub mod image_resource;
pub mod default_resource;
pub mod web_resource;

use std::default::Default;

#[derive(Clone)]
pub enum ChannelItemType {
    Show(ChannelShowItem),
    Index(ChannelIndexItem),
    Image(ChannelImageItem)
}

#[derive(Clone)]
#[derive(Default)]
pub struct ChannelIndexItem { }

#[derive(Clone)]
#[derive(Default)]
pub struct ChannelShowItem {
    pub postid: String,
    pub page: usize,
}

#[derive(Default)]
pub struct ChannelItem {
    pub extra: Option<ChannelItemType>,
    pub result: String
}

#[derive(Clone)]
#[derive(Default)]
pub struct ChannelImageItem {
    pub url: String,
    pub bytes: Vec<u8>
}

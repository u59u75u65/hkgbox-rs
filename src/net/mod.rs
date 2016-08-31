pub mod web_resource;

#[derive(Clone)]
pub enum ChannelItemType {
    Show(ChannelShowItem),
    Index(ChannelIndexItem)
}

#[derive(Clone)]
pub struct ChannelIndexItem { }

#[derive(Clone)]
pub struct ChannelShowItem {
    pub postid: String,
    pub page: usize,
}

pub struct ChannelItem {
    pub extra: ChannelItemType,
    pub result: String
}

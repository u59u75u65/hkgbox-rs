use utility::client::ChannelItem;
pub trait Resource {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem;
}

use net::ChannelItem;
pub trait Resource {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem;
}

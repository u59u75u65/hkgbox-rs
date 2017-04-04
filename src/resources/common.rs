use resources::*;
pub trait Resource {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem;
}

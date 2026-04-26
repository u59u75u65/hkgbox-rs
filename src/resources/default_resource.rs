use crate::resources::*;
use crate::resources::common::*;

pub struct DefaultResource {
}

impl DefaultResource {
    pub fn new() -> Self {
        DefaultResource {}
    }
}

impl Resource for DefaultResource {
    fn fetch(&mut self, _item: &ChannelItem) -> ChannelItem {
        Default::default()
    }
}

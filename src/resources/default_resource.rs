extern crate cancellation;
use self::cancellation::CancellationTokenSource;
use net::*;
use net::web_resource::*;
use resources::common::*;

pub struct DefaultResource<'a> {
    wr: &'a mut WebResource,
    ct: &'a CancellationTokenSource
}

impl <'a> DefaultResource<'a> {
    pub fn new(wr: &'a mut WebResource, ct: &'a CancellationTokenSource) -> Self {
        DefaultResource {
            wr: wr,
            ct: ct
        }
    }
}

impl <'a> Resource for DefaultResource<'a> {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem {
        ChannelItem {
            extra: ChannelItemType::Index(ChannelIndexItem { }),
            result: "".to_string()
        }
    }
}

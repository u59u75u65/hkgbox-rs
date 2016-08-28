extern crate cancellation;
use self::cancellation::CancellationTokenSource;
use utility::client::WebResource;
use utility::client::ChannelItem;
use utility::client::ChannelItemType;
use utility::client::ChannelIndexItem;
use resources::common::*;
use caches::common::*;
use caches::file_cache::*;

pub struct IndexResource<'a> {
    wr: &'a mut WebResource,
    ct: &'a CancellationTokenSource,
    fc: &'a FileCache
}

impl <'a> IndexResource<'a> {
    pub fn new(wr: &'a mut WebResource, ct: &'a CancellationTokenSource, fc: &'a mut FileCache) -> Self {
        IndexResource {
            wr: wr,
            ct: ct,
            fc: fc
        }
    }
}

impl <'a> Resource for IndexResource<'a> {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem {
        ChannelItem {
            extra: ChannelItemType::Index(ChannelIndexItem { }),
            result: "".to_string()
        }
    }
}

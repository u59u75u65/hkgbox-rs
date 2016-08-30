extern crate cancellation;
use self::cancellation::CancellationTokenSource;
use utility::client::WebResource;
use utility::client::ChannelItem;
use utility::client::ChannelItemType;
use utility::client::ChannelIndexItem;
use resources::common::*;
use caches::common::*;
use caches::file_cache::*;

pub struct IndexResource<'a, T: 'a + Cache> {
    wr: &'a mut WebResource,
    ct: &'a CancellationTokenSource,
    cache: &'a mut Box<T>,
}

impl<'a, T: 'a + Cache> IndexResource<'a, T> {
    pub fn new(wr: &'a mut WebResource, ct: &'a CancellationTokenSource, cache: &'a mut Box<T>) -> Self {
        IndexResource {
            wr: wr,
            ct: ct,
            cache: cache,
        }
    }
}

impl<'a, T: 'a + Cache> Resource for IndexResource<'a, T> {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem {
        ChannelItem {
            extra: ChannelItemType::Index(ChannelIndexItem { }),
            result: "".to_string()
        }
    }
}

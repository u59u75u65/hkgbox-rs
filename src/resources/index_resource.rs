
use ::cancellation::CancellationTokenSource;

use resources::*;
use resources::web_resource::*;
use resources::common::*;
use caches::common::*;
use caches::file_cache::*;

pub struct IndexResource<'a, T: 'a + Cache> {
    wr: &'a mut WebResource,
    ct: &'a CancellationTokenSource,
    cache: &'a mut Box<T>,
    url: &'static str
}

impl<'a, T: 'a + Cache> IndexResource<'a, T> {
    pub fn new(wr: &'a mut WebResource, ct: &'a CancellationTokenSource, cache: &'a mut Box<T>) -> Self {
        IndexResource {
            wr: wr,
            ct: ct,
            cache: cache,
            url: "http://archive.hkgolden.com/topics.aspx?type=BW"
        }
    }
}

impl<'a, T: 'a + Cache> Resource for IndexResource<'a, T> {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem {
        let time_format = |t: ::time::Tm| {
            match t.strftime("%Y%m%d%H%M") {
                Ok(s) => s.to_string(),
                Err(e) => panic!(e)
            }
        };

        let time = time_format(::time::now());

        let html_path = format!("data/cache/html/topics/");
        let file_name = format!("{time}.html", time = time);

        let (from_cache, result) = match self.cache.read(&html_path, &file_name) {
            Ok(result) => (true, result),
            Err(_) => {
                let url = self.url;
                let result = self.wr.get(&url);
                (false, result.into_bytes())
            }
        };

        if !from_cache {
            let result2 = result.clone();
            self.cache.write(&html_path, &file_name, result2);
        }

        let result_item = ChannelItem {
            extra: ChannelItemType::Index(ChannelIndexItem { }),
            result: String::from_utf8(result).expect("fail to build result item, reason: invalid string"),
        };
        result_item
    }
}

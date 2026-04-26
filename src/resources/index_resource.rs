use crate::resources::*;
use crate::resources::web_resource::*;
use crate::resources::common::*;
use crate::caches::common::*;

pub struct IndexResource<'a, T: 'a + Cache> {
    wr: &'a mut WebResource,
    cache: &'a mut Box<T>,
    url: &'static str
}

impl<'a, T: 'a + Cache> IndexResource<'a, T> {
    pub fn new(wr: &'a mut WebResource, cache: &'a mut Box<T>) -> Self {
        IndexResource {
            wr: wr,
            cache: cache,
            url: "http://archive.hkgolden.com/topics.aspx?type=BW"
        }
    }
}

impl<'a, T: 'a + Cache> Resource for IndexResource<'a, T> {
    fn fetch(&mut self, _item: &ChannelItem) -> ChannelItem {
        // Use a simpler time formatting approach
        let now = ::time::OffsetDateTime::now_utc();
        let time = format!("{:04}{:02}{:02}{:02}{:02}",
            now.year(), now.month() as u8, now.day(), now.hour(), now.minute());

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
            self.cache.write(&html_path, &file_name, result2).expect("fail to write cache");
        }

        let result_item = ChannelItem {
            extra: Some(ChannelItemType::Index(ChannelIndexItem {})),
            result: String::new(),
        };

        result_item
    }
}

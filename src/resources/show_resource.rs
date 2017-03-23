use resources::*;
use resources::web_resource::*;
use resources::common::*;
use caches::common::*;

pub struct ShowResource<'a, T: 'a + Cache> {
    wr: &'a mut WebResource,
    cache: &'a mut Box<T>,
    url: &'static str
}

impl<'a, T: 'a + Cache> ShowResource<'a, T> {
    pub fn new(wr: &'a mut WebResource, cache: &'a mut Box<T>) -> Self {
        ShowResource {
            wr: wr,
            cache: cache,
            url: "http://archive.hkgolden.com/topics.aspx?type=BW"
        }
    }
    fn post_url(&self, postid: &String, page: usize) -> String {
        let base_url = "http://forum1.hkgolden.com/view.aspx";
        let posturl = format!("{base_url}?type=BW&message={postid}&page={page}",
                              base_url = base_url,
                              postid = postid,
                              page = page);
        posturl
    }

}

impl<'a, T: 'a + Cache> Resource for ShowResource<'a, T> {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem {
        match item.extra.clone() {
            ChannelItemType::Show(extra) => {
                let html_path = format!("data/cache/html/{postid}/", postid = extra.postid);
                let show_file_name = format!("show_{page}.html", page = extra.page);

                let postid = extra.postid.clone();

                let (from_cache, result) = match self.cache.read(&html_path, &show_file_name) {
                    Ok(result) => (true, result),
                    Err(_) => {
                        let posturl = self.post_url(&extra.postid, extra.page);
                        let result = self.wr.get(&posturl);
                        (false, result.into_bytes())
                    }
                };

                if !from_cache {
                    let result2 = result.clone();
                    self.cache.write(&html_path, &show_file_name, result2).expect("fail to write cache");
                }

                let result_item = ChannelItem {
                    extra: ChannelItemType::Show(ChannelShowItem { postid: postid, page: extra.page }),
                    result: String::from_utf8(result).expect("fail to build result item, reason: invalid string"),
                };
                result_item
            },
            _ => {
                let result_item = ChannelItem {
                    extra: ChannelItemType::Show(ChannelShowItem { postid: "".to_string() , page: 0 }),
                    result: "".to_string(),
                };
                result_item
            }
        }
    }
}

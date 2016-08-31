extern crate cancellation;
use self::cancellation::CancellationTokenSource;
extern crate time;
use net::*;
use net::web_resource::*;
use resources::common::*;
use caches::common::*;
use caches::file_cache::*;

pub struct ShowResource<'a, T: 'a + Cache> {
    wr: &'a mut WebResource,
    ct: &'a CancellationTokenSource,
    cache: &'a mut Box<T>,
    url: &'static str
}

impl<'a, T: 'a + Cache> ShowResource<'a, T> {
    pub fn new(wr: &'a mut WebResource, ct: &'a CancellationTokenSource, cache: &'a mut Box<T>) -> Self {
        ShowResource {
            wr: wr,
            ct: ct,
            cache: cache,
            url: "http://forum1.hkgolden.com/topics_bw.htm"
        }
    }
    fn postUrl(&self, postid: &String, page: usize) -> String {
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
                let html_path = format!("data/html/{postid}/", postid = extra.postid);
                let show_file_name = format!("show_{page}.html", page = extra.page);

                let postid = extra.postid.clone();

                let (from_cache, result) = match self.cache.read(&html_path, &show_file_name) {
                    Ok(result) => (true, result),
                    Err(_) => {
                        let posturl = self.postUrl(&extra.postid, extra.page);
                        let result = self.wr.get(&posturl);
                        (false, result)
                    }
                };

                if !from_cache {
                    let result2 = result.clone();
                    self.cache.write(&html_path, &show_file_name, result2);
                }

                let result_item = ChannelItem {
                    extra: ChannelItemType::Show(ChannelShowItem { postid: postid, page: extra.page }),
                    result: result,
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

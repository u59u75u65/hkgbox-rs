use std::io::Cursor;

use kuchiki::NodeRef;
use kuchiki::NodeDataRef;
use kuchiki::NodeData;
use kuchiki::ElementData;

use model::ListTopicItem;
use model::ListTopicTitleItem;
use model::ListTopicAuthorItem;
use model::ShowItem;
use model::ShowReplyItem;
use model::UrlQueryItem;
use reply_model::*;

use regex::Regex;
use url::Url;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Index { }

impl Index {
    pub fn new() -> Self {
        Index {}
    }
    pub fn build(&mut self, document: &NodeRef) -> Vec<ListTopicItem> {

            let trs = match document.select(".Topic_ListPanel tr[id]") {
                    Ok(trs) => trs,
                    Err(e) => panic!("{:?}", e)
            };

            trs.enumerate().map(|(i, tr)| {
                let items = match tr.as_node().select("td") {
                    Ok(items) => items,
                    Err(e) => panic!("{:?}", e)
                };
                let mut result: ListTopicItem = Default::default();

                for (j, item) in items.enumerate().filter(|&(j, _)| j > 0 && j < 6) {
                    match j {
                        1 => { result.title = self.parse_list_topic_title_item(&item) },
                        2 => { result.author = self.parse_list_topic_author_item(&item) },
                        3 => {
                            let (date, time) = {
                                let text = item.text_contents().trim().to_string();
                                let map = text.split("\n").map(|x| x.trim().to_string()).collect::<Vec<_>>();
                                if map.len() < 2 {
                                    panic!("length of map is invalid.");
                                }
                                (map[0].clone(), map[1].clone())
                            };
                            result.last_replied_date = date;
                            result.last_replied_time = time;
                        },
                        4 => {
                            let text = item.text_contents().trim().to_string();
                            result.reply_count = text
                        },
                        5 => {
                            let text = item.text_contents().trim().to_string();
                            result.rating = text
                        },
                        _ => {}
                    }
                }
                result
            }).collect::<Vec<_>>()
    }
}

impl Index {
    fn parse_list_topic_title_item(&self, item: &NodeDataRef<ElementData>) -> ListTopicTitleItem {
        let (first_link, links_count) = {
            let mut links = match item.as_node().select("a") {
                Ok(links) => links,
                Err(e) => panic!("ERR: {:?}", e)
            };

            let first_link_option = links.next();
            let last_link_option = links.last();

            let first_link = match first_link_option {
                Some(first_link) => first_link,
                None => { panic!("ERR: Can't find 'first_link'.") }
            };

            let max_page = match last_link_option {
                Some(last_link) =>
                    last_link.text_contents().trim().to_string().parse::<usize>()
                    .unwrap_or(0)
                ,
                None => { 1 }
            };

            (first_link, max_page)
        };

        let (url_str, url_query_item) = {
            let attrs = &(first_link.attributes).borrow();
            let href = attrs.get("href").unwrap_or("");

            let base_url = match Url::parse("http://forum1.hkgolden.com/view.aspx") {
                Ok(url) => url,
                Err(e) => { panic!(format!("fail to parse Base URL. Reason: {}", e)) }
            };
            let url = match base_url.join(&href) {
                Ok(url) => url,
                Err(e) => { panic!(format!("fail to build URL. Reason: {}", e)) }
            };
            let url_str = url.into_string();
            let url_query_item = self.parse_url_query_item(&url_str);
            (url_str, url_query_item)
        };

        let text = first_link.text_contents().trim().to_string();

         ListTopicTitleItem {
            url: url_str,
            url_query: url_query_item,
            text: text,
            num_of_pages: links_count
        }
    }

    fn parse_list_topic_author_item(&self, item: &NodeDataRef<ElementData>) -> ListTopicAuthorItem {
        let (url, name) = {
            let mut links = match item.as_node().select("a") {
                Ok(links) => links,
                Err(e) => panic!("ERR: {:?}", e)
            };

            let first_link_option = links.next();

            let first_link = match first_link_option {
                Some(first_link) => first_link,
                None => { panic!("ERR: Can't find 'first_link'.") }
            };

            let attrs = &(first_link.attributes).borrow();
            let href = attrs.get("href").unwrap_or("").to_string();
            let text = first_link.text_contents().trim().to_string();
            (href, text)
        };

        ListTopicAuthorItem {
           url: url,
           name: name
        }
    }

    fn parse_url_query_item(&self, url_str: &str) -> UrlQueryItem {
        let url = Url::parse(&url_str).expect("fail to parse url query item, reason: invalid url");
        let query = url.query().expect("fail to parse url query item, reason: invalid url query");
        let re = Regex::new(r"(\\?|&)(?P<key>[^&=]+)=(?P<value>[^&]+)").expect("fail to parse url query item, reason: invalid regex");

        let (channel, message) = {

            let mut map = HashMap::new();

            for cap in re.captures_iter(query) {
                let key = cap.name("key").unwrap_or("").to_string();
                let value = cap.name("value").unwrap_or("").to_string();
                map.entry(key).or_insert(value);
            }

            if map.len() < 2 {
                panic!("length of map is invalid.")
            }

            (
                map.get("type").expect("fail to parse url query item, reason: can not get value of 'type' attribute").to_string(),
                map.get("message").expect("fail to parse url query item, reason: can not get value of 'message' attribute").to_string()
            )
        };

        UrlQueryItem {
            channel: String::from(channel),
            message: String::from(message)
        }

    }
}

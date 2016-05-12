extern crate rustc_serialize;
extern crate chrono;
extern crate kuchiki;
extern crate regex;
extern crate url;

use std::io::Cursor;
use std::io::BufReader;

use kuchiki::traits::*;
use kuchiki::NodeRef;

use model::ListTopicItem;
use model::ShowItem;
use model::ShowReplyItem;
use model::UrlQueryItem;

use regex::Regex;
use url::Url;
use std::collections::HashMap;

pub struct Builder { }

impl Builder {
    pub fn new() -> Self {
        Builder {}
    }

    pub fn show_item(&mut self, document: &NodeRef,  url: &str) -> ShowItem {
        parse_show_item(&document, &url)
    }

}

fn parse_show_item(document: &NodeRef, url: &str) -> ShowItem {

    let url_query = parse_url_query_item(&url);

    let (title, reply_count) = {
        let repliers_tr = document.select(".repliers tr").unwrap().next().unwrap();
        let repliers_header = repliers_tr.as_node()
                                         .select(".repliers_header")
                                         .unwrap()
                                         .last()
                                         .unwrap();
        let divs = repliers_header.as_node().select("div").unwrap().collect::<Vec<_>>();

        let topic_data = divs.iter()
                             .enumerate()
                             .map(|(index, div)| {
                                 let s_trimmed = div.text_contents().trim().to_string();
                                 if index == 1 {
                                     let re = Regex::new(r"^(?P<count>\d+)個回應$").unwrap();
                                     let cap = re.captures(&s_trimmed).unwrap();
                                     // String::from(cap.name("count").unwrap_or("0"))
                                     cap.name("count").unwrap_or("0").to_string()
                                 } else {
                                     s_trimmed
                                 }
                             })
                             .collect::<Vec<_>>();

        if topic_data.len() < 2 {
            panic!("length of topic_data is invalid.")
        }

        (topic_data.get(0).unwrap().to_string(), // return as title
         topic_data.get(1).unwrap().to_string() /* return as reply_count */)
    };

    let (page, max_page) = {

        let page_select = document.select("select[name='page']").unwrap().last().unwrap();
        let page_str = page_select.as_node()
                                  .select("option[selected='selected']")
                                  .unwrap()
                                  .next()
                                  .unwrap();
        let max_page_str = page_select.as_node().select("option").unwrap().last().unwrap();

        let page = page_str.text_contents().trim().to_string().parse::<usize>().unwrap_or(0);
        let max_page = max_page_str.text_contents()
                                   .trim()
                                   .to_string()
                                   .parse::<usize>()
                                   .unwrap_or(0);

        (page, max_page)
    };

    let replies = parse_show_reply_items(&document);

    ShowItem {
        url_query: url_query,
        replies: replies,
        page: page,
        max_page: max_page,
        reply_count: String::from(reply_count),
        title: String::from(title),
    }
}

fn parse_show_reply_items(document: &NodeRef) -> Vec<ShowReplyItem> {

    let replies_data = document.select(".repliers tr[userid][username]")
                               .unwrap()
                               .collect::<Vec<_>>();

    replies_data.iter()
                .enumerate()
                .map(|(index, tr)| {

                    let tr_attrs = (&tr.attributes).borrow();
                    let userid = tr_attrs.get("userid").unwrap();
                    let username = tr_attrs.get("username").unwrap();

                    let content_elm = tr.as_node()
                                        .select(".repliers_right .ContentGrid")
                                        .unwrap()
                                        .next()
                                        .unwrap(); // first

                    let mut buff = Cursor::new(Vec::new());
                    let serialize_result = content_elm.as_node().serialize(&mut buff);
                    let vec = buff.into_inner();
                    let content = String::from_utf8(vec).unwrap();

                    let datatime = tr.as_node()
                                     .select(".repliers_right span")
                                     .unwrap()
                                     .last()
                                     .unwrap()
                                     .text_contents();

                    ShowReplyItem {
                        userid: String::from(userid),
                        username: String::from(username),
                        content: String::from(content),
                        published_at: String::from(datatime),
                    }
                })
                .collect::<Vec<_>>()
}

fn parse_url_query_item(url_str: &str) -> UrlQueryItem {
    let url = Url::parse(&url_str).unwrap();
    let query = url.query().unwrap();
    let re = Regex::new(r"(\\?|&)(?P<key>[^&=]+)=(?P<value>[^&]+)").unwrap();

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
            map.get("type").unwrap().to_string(),
            map.get("message").unwrap().to_string()
        )
    };

    UrlQueryItem {
        message: String::from(message)
    }

}

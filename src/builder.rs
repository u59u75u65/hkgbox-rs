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
use model::ListTopicTitleItem;
use model::ListTopicAuthorItem;
use model::ShowItem;
use model::ShowReplyItem;
use model::UrlQueryItem;
use reply_model::*;

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

    pub fn url_query_item(&mut self, url: &str) -> UrlQueryItem {
        parse_url_query_item(&url)
    }
}

impl Builder {
    pub fn list_topic_items(&mut self, document: &NodeRef) -> Vec<ListTopicItem> {
        parse_list_topic_items(&document)
    }
}

fn default_list_item() -> ListTopicItem {
    ListTopicItem {
        title: ListTopicTitleItem {
            url: "".to_string(),
            url_query: UrlQueryItem {
                channel: "".to_string(),
                message: "".to_string()
            },
            text: "".to_string(),
            num_of_pages: 0
        },
        author: ListTopicAuthorItem {
            url: "".to_string(),
            name: "".to_string()
        },
        last_replied_date: "".to_string(),
        last_replied_time: "".to_string(),
        reply_count: "".to_string(),
        rating: "".to_string()
    }
}

fn parse_list_topic_items(document: &NodeRef) -> Vec<ListTopicItem>{

    let trs = match document.select(".Topic_ListPanel tr[id]") {
            Ok(trs) => trs,
            Err(e) => panic!("{:?}", e)
    };

    trs.enumerate().map(|(i, tr)| {
        let items = match tr.as_node().select("td") {
            Ok(items) => items,
            Err(e) => panic!("{:?}", e)
        };
        let mut result = default_list_item();

        for (j, item) in items.enumerate() {

            match j {
                0 => { },
                1 => {
                    let (first_link, links_count) = {
                        let mut links = match item.as_node().select("a") {
                            Ok(links) => links,
                            Err(e) => panic!("ERR: {:?}", e)
                        };

                        let firstLinkOption = links.next();
                        let lastLinkOption = links.last();

                        let first_link = match firstLinkOption {
                            Some(first_link) => first_link,
                            None => { panic!("ERR: Can't find 'first_link'.") }
                        };

                        let max_page = match lastLinkOption {
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

                        let base_url = Url::parse("http://forum1.hkgolden.com/view.aspx").unwrap();
                        let url = base_url.join(&href).unwrap();
                        let url_str = url.into_string();
                        let url_query_item = parse_url_query_item(&url_str);
                        (url_str, url_query_item)
                    };

                    let text = first_link.text_contents().trim().to_string();

                    result.title = ListTopicTitleItem {
                        url: url_str,
                        url_query: url_query_item,
                        text: text,
                        num_of_pages: links_count
                    };
                },
                2 => {
                    let (url, name) = {
                        let mut links = match item.as_node().select("a") {
                            Ok(links) => links,
                            Err(e) => panic!("ERR: {:?}", e)
                        };

                        let firstLinkOption = links.next();

                        let first_link = match firstLinkOption {
                            Some(first_link) => first_link,
                            None => { panic!("ERR: Can't find 'first_link'.") }
                        };

                        let attrs = &(first_link.attributes).borrow();
                        let href = attrs.get("href").unwrap_or("").to_string();
                        let text = first_link.text_contents().trim().to_string();
                        (href, text)
                    };

                     result.author = ListTopicAuthorItem {
                        url: url,
                        name: name
                    };
                },
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
            ()
        }
        result
    }).collect::<Vec<_>>()
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

                    let mut vec: Vec<NodeType> = Vec::new();

                    vec = recursive(content_elm.as_node());

                    ShowReplyItem {
                        userid: String::from(userid),
                        username: String::from(username),
                        content: String::from(content),
                        body: vec,
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
        channel: String::from(channel),
        message: String::from(message)
    }

}

fn recursive(elm: &kuchiki::NodeRef) -> Vec<NodeType> {

    let mut vec: Vec<NodeType> = Vec::new();

    for (index, child) in elm.children().enumerate() {
        // println!("[{}] => {:?}", index, child);
        let node_data = child.data().clone();

        match node_data {
            kuchiki::NodeData::Element(element_data) => {
                // println!("[{}] => [ELEMENT] => {:?}", index, element_data);
                // println!("[{}] => [ELEMENT] => {:?}", index, child);

                if element_data.name.local.trim().eq("blockquote") {

                    // println!("[{}] => [ELEMENT] => {:?}", index, child.children());
                    let subvec = recursive(&child);
                    let node = NodeType::BlockQuote(BlockQuoteNode { data: subvec });
                    vec.push(node);
                } else if element_data.name.local.trim().eq("br") {
                    let node = NodeType::Br(BrNode {});
                    vec.push(node);
                } else if element_data.name.local.trim().eq("img") {

                    let attrs = (&element_data.attributes).borrow();
                    let url = attrs.get("src").unwrap_or("");
                    let alt = attrs.get("alt").unwrap_or("");
                    let node = NodeType::Image(ImageNode { data: url.to_string(), alt: alt.to_string() });
                    vec.push(node);

                } else {
                    // println!("[{}] => [ELEMENT] => {:?}", index, child);
                    let mut subvec = recursive(&child);
                    vec.append(&mut subvec);
                }
            }
            kuchiki::NodeData::Text(rc) => {
                // println!("[{}] => [TEXT] => {:?}", index, rc);
                let d = rc.clone();
                let b = d.borrow();

                let mut s = b.trim().to_string();

                if s == "\n" {
                    // s = "\\n".to_string()
                    continue;
                }

                let node = NodeType::Text(TextNode { data: s });
                vec.push(node);
            }
            _ => {}
        }
    }
    vec
}

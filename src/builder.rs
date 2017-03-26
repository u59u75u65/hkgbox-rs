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
pub struct Builder { }

impl Builder {
    pub fn new() -> Self {
        Builder {}
    }
    pub fn url_query_item(&mut self, url: &str) -> UrlQueryItem {
        parse_url_query_item(&url)
    }
}

impl Builder {
    pub fn show_item(&mut self, document: &NodeRef,  url: &str) -> Result<ShowItem, &'static str> {

        let url_query = parse_url_query_item(&url);

        let (title, reply_count) = match parse_title_and_reply_count(&document, &url) {
            Ok((title,reply_count)) => (title, reply_count),
            Err(e) =>  { return Err(e) }
        };

        let (page, max_page) = {

            let page_select = document.select("select[name='page']").ok().expect("fail to build page and max_page, reason: 'page_select' not found").last().expect("fail to build page and max_page, reason: 'page_select' not found");
            let page_str = page_select.as_node()
                                      .select("option[selected='selected']")
                                      .ok().expect("fail to build page and max_page, reason: 'page_str' not found")
                                      .next()
                                      .expect("fail to build page and max_page, reason: 'page_str' not found");
            let max_page_str = page_select.as_node().select("option").ok().expect("fail to build page and max_page, reason: 'max_page_str' not found").last().expect("fail to build page and max_page, reason: 'max_page_str' not found");

            let page = page_str.text_contents().trim().to_string().parse::<usize>().unwrap_or(0);
            let max_page = max_page_str.text_contents()
                                       .trim()
                                       .to_string()
                                       .parse::<usize>()
                                       .unwrap_or(0);

            (page, max_page)
        };

        let replies = parse_show_reply_items(&document);

        let show_item = ShowItem {
            url_query: url_query,
            replies: replies,
            page: page,
            max_page: max_page,
            reply_count: String::from(reply_count),
            title: String::from(title),
        };

        Ok(show_item)
    }
    pub fn default_show_item(&self) -> ShowItem {
        default_show_item()
    }
}

impl Builder {
    pub fn list_topic_items(&mut self, document: &NodeRef) -> Vec<ListTopicItem> {
        parse_list_topic_items(&document)
    }
    pub fn default_list_item(&self) -> ListTopicItem {
        default_list_item()
    }
}

fn default_show_item() -> ShowItem {
    ShowItem {
        url_query: UrlQueryItem { channel: "".to_string(), message: String::from("") },
        replies: vec![],
        page: 0,
        max_page: 0,
        reply_count: String::from(""),
        title: String::from(""),
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

        for (j, item) in items.enumerate().filter(|&(j, _)| j > 0 && j < 6) {
            match j {
                1 => { result.title = parse_list_topic_title_item(&item) },
                2 => { result.author = parse_list_topic_author_item(&item) },
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

fn parse_list_topic_title_item(item: &NodeDataRef<ElementData>) -> ListTopicTitleItem {
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
        let url_query_item = parse_url_query_item(&url_str);
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

fn parse_list_topic_author_item(item: &NodeDataRef<ElementData>) -> ListTopicAuthorItem {
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

// Show Item
fn parse_title_and_reply_count (document: &NodeRef,  url: &str) -> Result<(String, String), &'static str> {

    return match document.select(".repliers tr") {
        Ok(mut trs) => {
            return match trs.next()  {
                Some(repliers_tr) => {

                    let repliers_header_option = repliers_tr.as_node().select(".repliers_header").ok().map_or(None, |x| x.last() );

                    let repliers_header = match repliers_header_option {
                        Some(x) => x,
                        None => { return Err("fail to build title and reply_count, reason: 'repliers_header' not found"); }
                    };

                    let divs = match repliers_header.as_node().select("div").ok() {
                        Some(x) => x.collect::<Vec<_>>(),
                        None => { return Err("fail to build title and reply_count, reason: 'divs' not found"); }
                    };

                    let topic_data = divs.iter().enumerate().map(|(index, div)| {
                                             let s_trimmed = div.text_contents().trim().to_string();
                                             if index == 1 {
                                                 let re = Regex::new(r"^(?P<count>\d+)個回應$").expect("fail to build title and reply_count, reason: invalid regex");
                                                 let cap = re.captures(&s_trimmed).expect("fail to build title and reply_count, reason: 'cap' not found");
                                                 // String::from(cap.name("count").unwrap_or("0"))
                                                 cap.name("count").unwrap_or("0").to_string()
                                             } else {
                                                 s_trimmed
                                             }
                                         })
                                         .collect::<Vec<_>>();

                    if topic_data.len() < 2 {
                        Err(&"length of topic_data is invalid.")
                    } else {
                        Ok(
                            (topic_data.get(0).expect("fail to build title and reply_count, reason: 'topic_data' not found").to_string(), // return as title
                            topic_data.get(1).expect("fail to build title and reply_count, reason: 'topic_data' not found").to_string() /* return as reply_count */)
                        )
                    }
                },
                None => Err(&"fail to build title and reply_count, reason: 'repliers_tr' not found")
            }
        },
        Err(e) =>  Err("fail to build title and reply_count, reason: 'repliers_tr' not found")
    };

}

fn parse_show_reply_items(document: &NodeRef) -> Vec<ShowReplyItem> {

    let replies_data = document.select(".repliers tr[userid][username]")
                               .ok().expect("fail to parse show reply items, reason: 'replies_data' not found")
                               .collect::<Vec<_>>();

    replies_data.iter()
                .enumerate()
                .map(|(index, tr)| {

                    let tr_attrs = (&tr.attributes).borrow();
                    let userid = tr_attrs.get("userid").expect("fail to parse show reply item, reason: 'userid' not found");
                    let username = tr_attrs.get("username").expect("fail to parse show reply item, reason: 'userame' not found");

                    let content_elm = tr.as_node()
                                        .select(".repliers_right .ContentGrid")
                                        .ok().expect("fail to parse show reply item, reason: 'content_elm' not found")
                                        .next()
                                        .expect("fail to parse show reply item, reason: 'content_elm' not found"); // first

                    let mut buff = Cursor::new(Vec::new());
                    let serialize_result = content_elm.as_node().serialize(&mut buff);
                    let vec = buff.into_inner();
                    let content = String::from_utf8(vec).expect("fail to parse show reply item, reason: 'content' invalid");

                    let datatime = tr.as_node()
                                     .select(".repliers_right span")
                                     .ok().expect("fail to parse show reply item, reason: 'datatime' not found")
                                     .last()
                                     .expect("fail to parse show reply item, reason: 'datatime' not found")
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

fn recursive(elm: &NodeRef) -> Vec<NodeType> {

    let mut vec: Vec<NodeType> = Vec::new();

    for (index, child) in elm.children().enumerate() {
        // println!("[{}] => {:?}", index, child);
        let node_data = child.data().clone();

        match node_data {
            NodeData::Element(element_data) => {
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
            NodeData::Text(rc) => {
                // println!("[{}] => [TEXT] => {:?}", index, rc);
                let d = rc.clone();
                let b = d.borrow();

                let s = b.trim().to_string();

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

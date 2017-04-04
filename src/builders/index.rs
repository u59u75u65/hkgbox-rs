use std::io::Cursor;

use kuchiki::NodeRef;
use kuchiki::NodeDataRef;
use kuchiki::NodeData;
use kuchiki::ElementData;

use model::ListTopicItem;
use model::ListTopicTitleItem;
use model::ListTopicAuthorItem;
use model::UrlQueryItem;

use regex::Regex;
use url::Url;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Index {}

impl Index {
    pub fn new() -> Self {
        Index {}
    }
    pub fn build(&mut self, document: &NodeRef) -> Result<Vec<ListTopicItem>, &'static str> {

        let trs_option = document.select(".Topic_ListPanel tr[id]");

        if trs_option.is_err() {
            return Err("fail to parse list topics item, reaons: trs_option not found");
        }

        let trs = trs_option.unwrap();

        let list_topics = trs.enumerate().map(list_topic_items_handler).collect::<Vec<_>>();

        let err_list_topics_option = list_topics.iter().filter(|x| x.is_err()).next();

        if err_list_topics_option.is_some() {
            return Err(&"fail to parse list topics items, reason: 'error list topic item' was found");
        }

        let result = list_topics.iter().map(|x| x.clone().unwrap()).collect::<Vec<_>>();

        Ok(result)
    }
}

fn list_topic_items_handler((index, tr): (usize, ::kuchiki::NodeDataRef<::kuchiki::ElementData>)) -> Result<ListTopicItem, &'static str> {

    let items_option = tr.as_node().select("td");

    if items_option.is_err() {
        return Err("fail to parse list topic items, reason: items_option not found");
    }

    let items = items_option.unwrap();

    let mut result: ListTopicItem = Default::default();

    for (j, item) in items.enumerate().filter(|&(j, _)| j > 0 && j < 6) {
        match j {
            1 => {
                match parse_list_topic_title_item(&item) {
                    Ok(s) => result.title = s,
                    Err(_) => {}
                }
            }
            2 => {
                match parse_list_topic_author_item(&item) {
                    Ok(s) => result.author = s,
                    Err(_) => {}
                }
            }
            3 => {
                let (date, time) = {
                    let text = item.text_contents().trim().to_string();
                    let map = text.split("\n").map(|x| x.trim().to_string()).collect::<Vec<_>>();
                    let count = map.len();

                    if count < 2 {
                        error!("length of map is invalid. length: {}", count);
                        return Err(&"length of map is invalid.");
                    }

                    let mut map_enumerator = map.iter().enumerate();
                    let date_option = match map_enumerator.clone()
                              .filter(|&(i, _)| i == 0)
                              .map(|(i, e)| (i, e))
                              .next() {
                        Some((i, text)) => Some(text),
                        None => None,
                    };

                    let time_option = match map_enumerator.clone()
                              .filter(|&(i, _)| i == 1)
                              .map(|(i, e)| (i, e))
                              .next() {
                        Some((i, text)) => Some(text),
                        None => None,
                    };

                    if date_option.is_none() {
                        return Err("fail to parse list topic items, reason: date_option not found");
                    }

                    if time_option.is_none() {
                        return Err("fail to parse list topic items, reason: time_option not found");
                    }

                    (date_option.unwrap().clone(), time_option.unwrap().clone())
                };
                result.last_replied_date = date;
                result.last_replied_time = time;
            }
            4 => {
                let text = item.text_contents().trim().to_string();
                result.reply_count = text
            }
            5 => {
                let text = item.text_contents().trim().to_string();
                result.rating = text
            }
            _ => {}
        }
    }

    Ok(result)

}

fn parse_list_topic_title_item(item: &NodeDataRef<ElementData>) -> Result<ListTopicTitleItem, &'static str> {
    let (first_link, links_count) = {
        let mut links_option = item.as_node().select("a");

        if links_option.is_err() {
            return Err("fail to parse list topic title item, reason: links not found");
        }

        let mut links = links_option.unwrap();

        let first_link_option = links.next();
        let last_link_option = links.last();

        if first_link_option.is_none() {
            return Err("fail to parse list topic title item, reason: first_link_option not found");
        }
        let first_link = first_link_option.unwrap();

        let max_page = match last_link_option {
            Some(last_link) => {
                last_link.text_contents()
                    .trim()
                    .to_string()
                    .parse::<usize>()
                    .unwrap_or(0)
            }
            None => 1,
        };

        (first_link, max_page)
    };

    let (url_str, url_query_item) = {
        let attrs = &(first_link.attributes).borrow();
        let href = attrs.get("href").unwrap_or("");

        let base_url_option = Url::parse("http://forum1.hkgolden.com/view.aspx");

        if base_url_option.is_err() {
            return Err("fail to parse list topic title item, reason: base_url_option not found");
        }
        let base_url = base_url_option.unwrap();

        let url_result = base_url.join(&href);

        if url_result.is_err() {
            return Err("fail to parse list topic title item, reason: fail to build URL. ");
        }
        let url = url_result.unwrap();

        let url_str = url.into_string();
        let url_query_option = parse_url_query_item(&url_str);

        if url_query_option.is_err() {
            return Err("fail to parse list topic title item, reason: fail to parse url query");
        }

        let url_query = url_query_option.unwrap();

        (url_str, url_query)
    };

    let text = first_link.text_contents().trim().to_string();

    Ok(ListTopicTitleItem {
           url: url_str,
           url_query: url_query_item,
           text: text,
           num_of_pages: links_count,
       })
}

fn parse_list_topic_author_item(item: &NodeDataRef<ElementData>) -> Result<ListTopicAuthorItem, &'static str> {
    let (url, name) = {

        let links_option = item.as_node().select("a");

        if links_option.is_err() {
            return Err("fail to parse list topic author item, reason: links_option not found");
        }

        let mut links = links_option.unwrap();

        let first_link_option = links.next();

        if first_link_option.is_none() {
            return Err("fail to parse list topic title item, reason: first_link_option not found");
        }
        let first_link = first_link_option.unwrap();

        let attrs = &(first_link.attributes).borrow();
        let href = attrs.get("href").unwrap_or("").to_string();
        let text = first_link.text_contents().trim().to_string();
        (href, text)
    };

    Ok(ListTopicAuthorItem {
           url: url,
           name: name,
       })
}

fn parse_url_query_item(url_str: &str) -> Result<UrlQueryItem, &'static str> {

    let url_option = Url::parse(&url_str);
    if url_option.is_err() {
        return Err("fail to parse url query item, reason: invalid url");
    }
    let url = url_option.unwrap();

    let query_option = url.query();
    if query_option.is_none() {
        return Err("fail to parse url query item, reason: invalid url query");
    }
    let query = query_option.unwrap();

    let re = Regex::new(r"(\\?|&)(?P<key>[^&=]+)=(?P<value>[^&]+)").expect("fail to parse url query item, reason: invalid regex");

    let (channel, message) = {

        let mut map = HashMap::new();

        for cap in re.captures_iter(query) {
            let key = cap.name("key").unwrap_or("").to_string();
            let value = cap.name("value").unwrap_or("").to_string();
            map.entry(key).or_insert(value);
        }

        let count = map.len();
        if count < 2 {
            error!("length of map is invalid. length: {}", count);
            return Err(&"length of map is invalid.");
        }

        let type_option = map.get("type");
        if type_option.is_none() {
            return Err(&"fail to parse url query item, reason: can not get value of 'type' attribute");
        }

        let message_option = map.get("message");
        if message_option.is_none() {
            return Err(&"fail to parse url query item, reason: can not get value of 'message' attribute");
        }

        (type_option.unwrap().to_string(), message_option.unwrap().to_string())
    };

    Ok(UrlQueryItem {
           channel: String::from(channel),
           message: String::from(message),
       })
}

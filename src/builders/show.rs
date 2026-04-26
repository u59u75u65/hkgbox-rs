use log::{info, error};
use std::io::Cursor;

use kuchiki::NodeRef;
use kuchiki::NodeData;

use crate::model::ShowItem;
use crate::model::ShowReplyItem;
use crate::model::UrlQueryItem;
use crate::reply_model::*;
use crate::HkgError;

use regex::Regex;
use url::Url;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Show { }

impl Show {
    pub fn new() -> Self {
        Show {}
    }

    pub fn build(&mut self, document: &NodeRef,  url: &str) -> Result<ShowItem, HkgError> {

        let url_query = match self.parse_url_query_item(&url) {
            Ok(url_query) => url_query,
            Err(e) =>  {
                error!("{}", e);
                return Err(e)
            }
        };

        let (title, reply_count) = match self.parse_title_and_reply_count(&document, &url) {
            Ok((title,reply_count)) => (title, reply_count),
            Err(e) =>  {
                error!("{}", e);
                return Err(e)
            }
        };

        let (page, max_page) = {

            let page_select_option = document.select("select[name='page']").ok().map_or(None, |x| x.last());

            if page_select_option.is_none() {
                return Err(HkgError::HtmlParse("fail to build page and max_page, reason: 'page_select' not found".into()));
            }

            let page_select = page_select_option.unwrap();

            let page_str_option = page_select.as_node().select("option[selected='selected']").ok().map_or(None, |mut x| x.next());

            if page_str_option.is_none() {
                return Err(HkgError::HtmlParse("fail to build page and max_page, reason: 'page_str' not found".into()));
            }

            let page_str = page_str_option.unwrap();

            let max_page_str_option = page_select.as_node().select("option").ok().map_or(None, |x| x.last());

            if max_page_str_option.is_none() {
                return Err(HkgError::HtmlParse("fail to build page and max_page, reason: 'max_page_str' not found".into()));
            }

            let max_page_str = max_page_str_option.unwrap();

            let page = page_str.text_contents().trim().to_string().parse::<usize>()
                .map_err(|e| HkgError::HtmlParse(format!("Failed to parse page number: {}", e)))?;

            let max_page = max_page_str.text_contents()
                                       .trim()
                                       .to_string()
                                       .parse::<usize>()
                                       .map_err(|e| HkgError::HtmlParse(format!("Failed to parse max_page number: {}", e)))?;

            if page == 0 || max_page == 0 {
                return Err(HkgError::HtmlParse("Invalid page numbers: both must be greater than 0".into()));
            }

            (page, max_page)
        };

        let replies = {
            let replies_option = self.parse_show_reply_items(&document);

            if replies_option.is_err() {
                let e = replies_option.err().unwrap();
                error!("{:?}", e);
                return Err(e);
            }

            replies_option.unwrap()
        };

        let show_item = ShowItem {
            url_query: url_query,
            replies: replies,
            page: page,
            max_page: max_page,
            reply_count: String::from(reply_count),
            title: String::from(title),
            main_content: None, // Old HTML parser doesn't extract main content
        };

        Ok(show_item)
    }

}


impl Show {

    fn parse_url_query_item(&self, url_str: &str) -> Result<UrlQueryItem, HkgError> {

        let url_option = Url::parse(&url_str);
        if url_option.is_err() {
            return Err(HkgError::UrlParse(format!("{}", url_option.unwrap_err())));
        }
        let url = url_option.unwrap();

        let query_option = url.query();
        if query_option.is_none() {
            return Err(HkgError::UrlParse("Invalid URL: missing query string".into()));
        }
        let query = query_option.unwrap();

        let re = Regex::new(r"(\\?|&)(?P<key>[^&=]+)=(?P<value>[^&]+)")
            .map_err(|e| HkgError::Config(format!("Invalid regex pattern: {}", e)))?;

        let (channel, message) = {

            let mut map = HashMap::new();

            for cap in re.captures_iter(query) {
                let key = cap.name("key").map(|m| m.as_str()).unwrap_or("").to_string();
                let value = cap.name("value").map(|m| m.as_str()).unwrap_or("").to_string();
                map.entry(key).or_insert(value);
            }

            let count = map.len();
            if count < 2 {
                error!("length of map is invalid. length: {}", count);
                return Err(HkgError::UrlParse("Invalid URL query: insufficient parameters".into()));
            }

            let type_option = map.get("type");
            if type_option.is_none() {
                return Err(HkgError::UrlParse("Invalid URL query: missing 'type' parameter".into()));
            }

            let message_option = map.get("message");
            if message_option.is_none() {
                return Err(HkgError::UrlParse("Invalid URL query: missing 'message' parameter".into()));
            }

            (
                type_option.unwrap().to_string(),
                message_option.unwrap().to_string()
            )
        };

        Ok(
            UrlQueryItem {
                channel: crate::model::ChannelId::new(channel),
                message: crate::model::MessageId::new(message)
            }
        )
    }

    fn parse_title_and_reply_count (&self, document: &NodeRef,  _url: &str) -> Result<(String, String), HkgError> {

        return match document.select(".repliers tr") {
            Ok(mut trs) => {
                return match trs.next()  {
                    Some(repliers_tr) => {

                        let repliers_header_option = repliers_tr.as_node().select(".repliers_header").ok().map_or(None, |x| x.last() );

                        if repliers_header_option.is_none() {
                            return Err(HkgError::HtmlParse("fail to build title and reply_count, reason: 'repliers_header' not found".into()));
                        }

                        let repliers_header = repliers_header_option.unwrap();

                        let divs_option = repliers_header.as_node().select("div").ok().map_or(None, |x| Some(x.collect::<Vec<_>>()));

                        if divs_option.is_none() {
                            return Err(HkgError::HtmlParse("fail to build title and reply_count, reason: 'divs' not found".into()));
                        }

                        let divs = divs_option.unwrap();

                        let divs_enumerator = divs.iter().enumerate();

                        let count = divs_enumerator.clone().count();
                        if  count < 2 {
                            error!("length of topic_data is invalid. length: {}", count);
                            return Err(HkgError::HtmlParse("length of topic_data is invalid.".into()));
                        }

                        let title_option = match divs_enumerator.clone().filter(|&(i, _)| i == 0).map(|(i, e)| (i,e)).next() {
                            Some((i, div)) => {
                                info!("{} => {:?}", i, div.text_contents());
                                Some(div.text_contents().trim().to_string())
                            },
                            None => None
                        };

                        let reply_count = match divs_enumerator.clone().filter(|&(i, _)| i == 1).map(|(i, e)| (i,e)).next() {
                            Some((i, div)) => {
                                info!("{} => {:?}", i, div.text_contents());
                                let re = Regex::new(r"^(?P<count>\d+)個回應$").expect("Failed to compile reply count regex pattern");
                                let s_trimmed = div.text_contents().trim().to_string();
                                let cap_option = re.captures(&s_trimmed);
                                if cap_option.is_none() {
                                    None
                                } else {
                                    Some(cap_option.unwrap().name("count").map(|m| m.as_str()).unwrap_or("0").to_string())
                                }
                            },
                            None => None
                        };

                        if  title_option.is_none() || reply_count.is_none() {
                            return Err(HkgError::HtmlParse("fail to build title and reply_count, reason: 'topic_data' not found".into()));
                        }

                        Ok(
                            (
                                title_option.unwrap().to_string(),
                                reply_count.unwrap().to_string()
                            )
                        )
                    },
                    None => Err(HkgError::HtmlParse("fail to build title and reply_count, reason: 'repliers_tr' not found".into()))
                }
            },
            Err(_e) =>  Err(HkgError::HtmlParse("fail to build title and reply_count, reason: 'repliers_tr' not found".into()))
        };

    }

    fn parse_show_reply_items(&self, document: &NodeRef) -> Result<Vec<ShowReplyItem>, HkgError>  {

        let replies_data_option = document.select(".repliers tr[userid][username]").ok().map_or(None, |x| Some(x.collect::<Vec<_>>()) );

        if replies_data_option.is_none() {
            return Err(HkgError::HtmlParse("fail to parse show reply items, reason: 'replies_data' not found".into()));
        }

        let replies_data = replies_data_option.unwrap();

        let show_replies = replies_data.iter().enumerate().map(reply_items_handler).collect::<Vec<_>>();

        let err_show_reply_option = show_replies.iter().filter(|x| x.is_err()).next();

        if err_show_reply_option.is_some() {
            return Err(HkgError::HtmlParse("fail to parse show reply items, reason: 'error show reply item' was found".into()));
        }

        let result = show_replies.iter()
            .filter_map(|x| x.as_ref().ok())
            .cloned()
            .collect::<Vec<_>>();

        Ok(result)
    }
}


fn reply_items_handler((_index,tr): (usize, &::kuchiki::NodeDataRef<::kuchiki::ElementData>)) -> Result<ShowReplyItem, HkgError> {
    let tr_attrs = (&tr.attributes).borrow();
    let userid_option = tr_attrs.get("userid");

    if userid_option.is_none() {
        return Err(HkgError::HtmlParse("fail to parse show reply item, reason: 'userid' not found".into()));
    }

    let userid = userid_option.unwrap();

    let username_option = tr_attrs.get("username");

    if username_option.is_none() {
        return Err(HkgError::HtmlParse("fail to parse show reply item, reason: 'userame' not found".into()));
    }

    let username = username_option.unwrap();

    let content_elm_option = tr.as_node().select(".repliers_right .ContentGrid").ok().map_or(None, |mut x| x.next());

    if content_elm_option.is_none() {
        return Err(HkgError::HtmlParse("fail to parse show reply item, reason: 'content_elm' not found".into()));
    }
    let content_elm = content_elm_option.unwrap();

    let mut buff = Cursor::new(Vec::new());
    let _serialize_result = content_elm.as_node().serialize(&mut buff);
    let vec = buff.into_inner();
    let content_result = String::from_utf8(vec);

    if content_result.is_err() {
        return Err(HkgError::HtmlParse("fail to parse show reply item, reason: 'content' invalid".into()));
    }

    let content = content_result.unwrap();

    let datatime_option = tr.as_node().select(".repliers_right span").ok()
                    .map_or(None, |x| x.last() )
                    .map_or(None, |x| Some(x.text_contents()));

    if datatime_option.is_none() {
        return Err(HkgError::HtmlParse("fail to parse show reply item, reason: 'datatime' not found".into()));
    }

    let datatime = datatime_option.unwrap();

    let vec = recursive(content_elm.as_node());

    Ok(
        ShowReplyItem {
            userid: String::from(userid),
            username: String::from(username),
            content: String::from(content),
            body: vec,
            published_at: String::from(datatime),
        }
    )
}


fn recursive(elm: &NodeRef) -> Vec<NodeType> {

    let mut vec: Vec<NodeType> = Vec::new();

    for (_index, child) in elm.children().enumerate() {
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

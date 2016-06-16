extern crate hkg;
extern crate rustbox;
extern crate rustc_serialize;
extern crate chrono;
extern crate kuchiki;
extern crate hyper;
extern crate cancellation;

use kuchiki::traits::*;
use kuchiki::NodeRef;

use std::default::Default;

use rustbox::{Color, RustBox, Key};
use rustc_serialize::json;
use rustc_serialize::json::Json;

use chrono::*;

use hkg::utility::cache;
use hkg::model::ListTopicItem;
use hkg::model::ShowItem;
use hkg::model::ShowReplyItem;
use hkg::model::UrlQueryItem;
use hkg::utility::client::*;

use std::path::Path;

use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::io::{Error, ErrorKind};

use std::io::Cursor;
use std::io::BufReader;

use std::collections::HashMap;

use hyper::Client;
use std::sync::{Arc, Mutex};
use std::thread;
use cancellation::{CancellationToken, CancellationTokenSource, OperationCanceled};
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::channel;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Status {
    List,
    Show,
}

fn main() {

    // GUI init
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1,
                  1,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Black,
                  &format!("start => {}", Local::now()));

    let title = String::from("高登");
    let s = cache::readfile(String::from("data/topics.json"));
    let collection: Vec<ListTopicItem> = json::decode(&s).unwrap();

    // initialize show with empty page
    let mut show_item = ShowItem {
        url_query: UrlQueryItem { message: String::from("") },
        replies: vec![],
        page: 0,
        max_page: 0,
        reply_count: String::from(""),
        title: String::from(""),
    };

    let mut status = String::from("> ");

    let mut state = Status::List;
    let mut prev_state = state;
    let mut prev_width = rustbox.width();

    let mut list = hkg::screen::list::List::new(&rustbox);
    let mut show = hkg::screen::show::Show::new(&rustbox);

    let mut builder = hkg::builder::Builder::new();

    // let url = String::from("http://www.alexa.com/");
    // let url = String::from("http://localhost:3000");
    // let url = String::from("https://www.yahoo.com.hk/");

    let (tx_req, rx_req) = channel::<ChannelItem>();
    let (tx_res, rx_res) = channel::<ChannelItem>();

    let wclient = thread::spawn(move || {
        let mut wr = WebResource::new();
        let mut ct = CancellationTokenSource::new();
        ct.cancel_after(std::time::Duration::new(10, 0));
        loop {
            match rx_req.recv() {
                Ok(item) => {

                    let th = thread::current();
                    ct.run(|| {
                               th.unpark();
                           },
                           || {

                               let html_path = format!("data/html/{postid}/",  postid = item.postid);
                               let show_file_name = format!("show_{page}.html", page = item.page);

                               let postid = item.postid.clone();
                               let (from_cache, result) = match read_cache(&html_path, &show_file_name) {
                                   Ok(result) => (true, result),
                                   Err(e) => {
                                       let posturl = get_posturl(&item.postid, item.page);
                                       let result = wr.get(&posturl);
                                       (false, result)
                                   }
                               };

                                if !from_cache {
                                    let result2 = result.clone();
                                    write_cache(&html_path, &show_file_name, result2);
                                }

                               let result_item = ChannelItem {
                                   postid: postid,
                                   page: item.page,
                                   result: result,
                               };

                               tx_res.send(result_item).unwrap();

                           });

                    if ct.is_canceled() {
                        thread::park_timeout(std::time::Duration::new(0, 250));
                        // Err(OperationCanceled)
                    } else {
                        // Ok(())
                    }

                }
                Err(e) => {}
            }
        }
    });

    let mut is_web_requesting = false;

    loop {

        // show UI
        if prev_state != state {
            hkg::screen::common::clear(&rustbox); // clear screen when switching state
            prev_state = state;
        }

        match rx_res.try_recv() {
            Ok(item) => {
                let document = kuchiki::parse_html().from_utf8().one(item.result.as_bytes());

                let posturl = get_posturl(&item.postid, item.page);
                show_item = builder.show_item(&document, &posturl);

                let w = rustbox.width();
                status = format_status(status,
                                       w,
                                       &format!("[{}-{}:ROK][{}]",
                                                show_item.url_query.message,
                                                show_item.page, is_web_requesting));

                show.resetY();
                hkg::screen::common::clear(&rustbox);
                state = Status::Show;
                is_web_requesting = false;
            }
            Err(e) => { }
        }

        match state {
            Status::List => {
                list.print(&title, &collection);
            }
            Status::Show => {
                show.print(&title, &show_item);
            }
        }

        print_status(&rustbox, &status);

        rustbox.present();

        if !is_web_requesting {

            match rustbox.poll_event(false) {
                Ok(rustbox::Event::KeyEvent(key)) => {

                    if prev_width != rustbox.width() {
                        hkg::screen::common::clear(&rustbox);
                        prev_width = rustbox.width();
                    }

                    match key {
                        Key::Char('q') => {
                            break;
                        }
                        Key::PageUp => {
                            let w = rustbox.width();
                            status = format_status(status, w, " PU");

                            match state {
                                Status::List => {}
                                Status::Show => {
                                    let bh = show.body_height();
                                    if show.scrollUp(bh) {
                                        hkg::screen::common::clear(&rustbox);
                                    }
                                }
                            }
                        }
                        Key::PageDown => {
                            let w = rustbox.width();
                            status = format_status(status, w, " PD");

                            match state {
                                Status::List => {}
                                Status::Show => {
                                    let bh = show.body_height();
                                    if show.scrollDown(bh) {
                                        hkg::screen::common::clear(&rustbox);
                                    }
                                }
                            }
                        }
                        Key::Up => {
                            let w = rustbox.width();
                            status = format_status(status, w, "U");

                            match state {
                                Status::List => {
                                    let tmp = list.get_selected_topic();
                                    if tmp > 1 {
                                        list.select_topic(tmp - 1);
                                    }
                                }
                                Status::Show => {
                                    if show.scrollUp(2) {
                                        hkg::screen::common::clear(&rustbox);
                                    }
                                }
                            }

                        }
                        Key::Down => {
                            let w = rustbox.width();
                            status = format_status(status, w, "D");

                            match state {
                                Status::List => {
                                    let tmp = list.get_selected_topic();
                                    if tmp < list.body_height() {
                                        list.select_topic(tmp + 1);
                                    }
                                }
                                Status::Show => {
                                    if show.scrollDown(2) {
                                        hkg::screen::common::clear(&rustbox);
                                    }
                                }
                            }
                        }
                        Key::Left => {
                            match state {
                                Status::List => {}
                                Status::Show => {
                                    if show_item.page > 1 {
                                        let postid = &show_item.url_query.message;
                                        let page = &show_item.page - 1;
                                        let posturl = get_posturl(postid, page);

                                        let ci = ChannelItem {
                                            // url: posturl.clone(),
                                            postid: postid.clone(),
                                            page: page,
                                            result: String::from(""),
                                        };

                                        match tx_req.send(ci) {
                                            Ok(()) => {
                                                let w = rustbox.width();
                                                status = format_status(status,
                                                                       w,
                                                                       &format!("[{}-{}:SOK]", postid, page));
                                                is_web_requesting = true;
                                            }
                                            Err(e) => {
                                                let w = rustbox.width();
                                                status = format_status(status,
                                                                       w,
                                                                       &format!("[{}-{}:SFAIL:{}]",
                                                                                postid,
                                                                                page,
                                                                                e));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Key::Right => {
                            match state {
                                Status::List => {}
                                Status::Show => {
                                    if show_item.max_page > show_item.page {

                                        let postid = &show_item.url_query.message;
                                        let page = &show_item.page + 1;
                                        let posturl = get_posturl(postid, page);

                                        let ci = ChannelItem {
                                            // url: posturl.clone(),
                                            postid: postid.clone(),
                                            page: page,
                                            result: String::from(""),
                                        };

                                        match tx_req.send(ci) {
                                            Ok(()) => {
                                                let w = rustbox.width();
                                                status = format_status(status,
                                                                       w,
                                                                       &format!("[{}-{}:SOK][{}]", postid, page, is_web_requesting));
                                                is_web_requesting = true;
                                            }
                                            Err(e) => {
                                                let w = rustbox.width();
                                                status = format_status(status,
                                                                       w,
                                                                       &format!("[{}-{}:SFAIL:{}]",
                                                                                postid,
                                                                                page,
                                                                                e));
                                            }
                                        }

                                    }
                                }
                            }
                        }
                        Key::Enter => {
                            let w = rustbox.width();
                            status = format_status(status, w, "E");
                            match state {
                                Status::List => {

                                    let index = list.get_selected_topic();
                                    if index > 0 {
                                        let topic_item = &collection[index - 1];

                                        let posturl = &topic_item.title.url;
                                        let postid = &topic_item.title.url_query.message;

                                        let ci = ChannelItem {
                                            // url: posturl.clone(),
                                            postid: postid.clone(),
                                            page: 1,
                                            result: String::from(""),
                                        };

                                        match tx_req.send(ci) {
                                            Ok(()) => {
                                                let w = rustbox.width();
                                                is_web_requesting = true;
                                                status = format_status(status,
                                                                       w,
                                                                       &format!("[{}:SOK][{}]", postid, is_web_requesting));

                                            }
                                            Err(e) => {
                                                let w = rustbox.width();
                                                status = format_status(status,
                                                                       w,
                                                                       &format!("[{}:SFAIL:{}]",
                                                                                postid,
                                                                                e));
                                            }
                                        }

                                    }
                                }
                                Status::Show => {}
                            }
                        }
                        Key::Backspace => {
                            let w = rustbox.width();
                            status = format_status(status, w, "B");
                            match state {
                                Status::List => {}
                                Status::Show => {
                                    state = Status::List;
                                }
                            }
                        }

                        _ => {}
                    }
                }
                Err(e) => panic!("{}", e),
                _ => {}
            }
        }

    }
}

fn read_cache<P: AsRef<Path>, S : AsRef<Path>>(cache_path: P, file_name: S) -> Result<String, String>{
    let file_path = cache_path.as_ref().join(file_name);
    let mut file = try!(File::open(file_path).map_err(|e| e.to_string()));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents).map_err(|e| e.to_string()));
    Ok(contents)
}

fn write_cache<P: AsRef<Path>, S : AsRef<Path>>(cache_path: P, file_name: S, s: String) -> Result<(), String>{
    let file_path = cache_path.as_ref().join(file_name);
    try!(fs::create_dir_all(&cache_path).map_err(|e| e.to_string()));
    let mut file = try!(File::create(file_path).map_err(|e| e.to_string()));
    try!(file.write_all(&s.into_bytes()).map_err(|e| e.to_string()));
    Ok(())
}

fn get_posturl(postid: &String, page: usize) -> String {
    let base_url = "http://forum1.hkgolden.com/view.aspx";
    let posturl = format!("{base_url}?type=BW&message={postid}&page={page}",
                          base_url = base_url,
                          postid = postid,
                          page = page);
    posturl
}

fn print_status(rustbox: &rustbox::RustBox, status: &str) {
    // for status bar only
    let w = rustbox.width();
    let h = rustbox.height();

    let status_width = if w > status.len() {
        w - status.len()
    } else {
        0
    };
    let status_spacing = (0..status_width).map(|_| " ").collect::<Vec<_>>().join("");

    rustbox.print(0,
                  h - 1,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  &format!("{status}{status_spacing}",
                           status = status,
                           status_spacing = status_spacing));

}

fn format_status(status: String, w: usize, s: &str) -> String {
    if status.len() >= w {
        String::from(format!("{}{}", &"> ", s))
    } else {
        String::from(format!("{}{}", &status, s))
    }
}

fn show_item_build_example(rustbox: &rustbox::RustBox, collection: &Vec<ListTopicItem>) {

    rustbox.print(1,
                  1,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Black,
                  &format!("before parse => {}", Local::now()));

    let mut builder = hkg::builder::Builder::new();

    let url = &collection[1].title.url;
    rustbox.print(1, 2, rustbox::RB_NORMAL, Color::White, Color::Black, url);

    let uqi = builder.url_query_item(&url);
    let postid = "6360604"; //uqi.message;
    let page = 1;
    let path = format!("data/html/{postid}/show_{page}.html",
                       postid = postid,
                       page = page);

    rustbox.print(1,
                  3,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Black,
                  &format!("path: {}", path));

    let show_item = match kuchiki::parse_html().from_utf8().from_file(&path) {
        Ok(document) => Some(builder.show_item(&document, &url)),
        Err(e) => None,
    };

    match show_item {
        Some(si) => {

            rustbox.print(1,
                          5,
                          rustbox::RB_NORMAL,
                          Color::White,
                          Color::Black,
                          &format!("url_query->message: {} title:{} reploy count: {} page: {} \
                                    max_page: {}",
                                   si.url_query.message,
                                   si.title,
                                   si.reply_count,
                                   si.page,
                                   si.max_page));

            for (index, item) in si.replies.iter().enumerate() {
                rustbox.print(1,
                              index + 7,
                              rustbox::RB_NORMAL,
                              Color::White,
                              Color::Black,
                              &format!("{:<2}={:?}", index, item));
            }
        }
        _ => {}
    }
}

// for (jndex, elm) in tr.as_node().select(".repliers_right .ContentGrid").unwrap().enumerate() {
//     let content = elm.as_node().text_contents();
//     let name = &elm.name;
//     rustbox.print(1, jndex + index + 4, rustbox::RB_NORMAL, Color::White, Color::Black, &format!("[{:<2}][{:<2}]={:?}", index, jndex, content));
// }

// let content = tr.text_contents();
// rustbox.print(1,
//               index + 4,
//               rustbox::RB_NORMAL,
//               Color::White,
//               Color::Black,
//               &format!("[{:<2}]={:?}", index, content));

// for (jndex, div) in tr.as_node().children().enumerate() {
//     let content = div.as_text();
//     rustbox.print(1, jndex + index + 4, rustbox::RB_NORMAL, Color::White, Color::Black, &format!("[{:<2}]={:?}", index + jndex, content));
// }

// for (jndex, div) in tr.as_node().select(".repliers_right").unwrap().enumerate() {
//     let content = div.as_node().as_text();
//     rustbox.print(1, jndex + index + 4, rustbox::RB_NORMAL, Color::White, Color::Black, &format!("[{:<2}]={:?}", index + jndex, content));
// }

// for (jndex, div) in tr.as_node().select(".repliers_right").unwrap().collect::<Vec<_>>().iter().enumerate() {
//     let content = div.as_node().as_text();
//     rustbox.print(1, jndex + index + 4, rustbox::RB_NORMAL, Color::White, Color::Black, &format!("[{:<2}]={:?}", index + jndex, content));
// }

// let c = &tr.as_node().select(".repliers_right .ContentGrid").unwrap().collect::<Vec<_>>()[0];
// let content = c.as_node().as_text();
// rustbox.print(1, index + 4, rustbox::RB_NORMAL, Color::White, Color::Black, &format!("[{:<2}]={:?}", index, content));


// fn date_operation_example(rustbox: &rustbox::RustBox) {
//     let now = Local::now();
//
//     let dt1 = match Local.datetime_from_str("30/4/2016 9:22", "%d/%m/%Y %H:%M") {
//         Ok(v) => v,
//         Err(e) => Local::now(),
//     };
//
//     let dt2 = now.checked_sub(Duration::seconds(46)).unwrap();
//     let dt3 = now.checked_sub(Duration::minutes(6)).unwrap();
//     let dt4 = now.checked_sub(Duration::days(17)).unwrap();
//     let dt5 = now.checked_sub(Duration::weeks(9)).unwrap();
//
//     rustbox.print(0,
//                   0,
//                   rustbox::RB_BOLD,
//                   Color::White,
//                   Color::Black,
//                   &format!("{} {} {} {}",
//                    duration_format(&(now - dt2)),
//                    duration_format(&(now - dt3)),
//                    duration_format(&(now - dt4)),
//                    duration_format(&(now - dt5))
//               ));
//
// }

// fn debug_load_and_print_topics() {
//     let s = cache::readfile(String::from("topics.json"));
//     let collection: Vec<TopicItem> = json::decode(&s).unwrap();
//
//     println!("topics {:?}", collection.len());
//     debug_print_topics(collection);
// }
//
// fn debug_print_topics(collection: Vec<TopicItem>) {
//     for (i, item) in collection.iter().enumerate() {
//
//         println!("item[{}]= {title} {author_name} {last_replied_date} {last_replied_time} \
//                   {reply_count} {rating}",
//                  i,
//                  title = item.titles[0].text,
//                  author_name = item.author.name,
//                  last_replied_date = item.last_replied_date,
//                  last_replied_time = item.last_replied_time,
//                  reply_count = item.reply_count,
//                  rating = item.rating);
//     }
// }

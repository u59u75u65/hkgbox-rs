extern crate hkg;
extern crate termion;
extern crate rustc_serialize;
extern crate chrono;
extern crate kuchiki;
extern crate hyper;
extern crate cancellation;

use kuchiki::traits::*;
use kuchiki::NodeRef;

use std::default::Default;

use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};
use termion::event::Key;
use termion::terminal_size;

use rustc_serialize::json;
use rustc_serialize::json::Json;

use chrono::*;

use hkg::utility::cache;
use hkg::model::IconItem;
use hkg::model::ListTopicItem;
use hkg::model::ShowItem;
use hkg::model::ShowReplyItem;
use hkg::model::UrlQueryItem;
use hkg::utility::client::*;

use std::path::Path;

// use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::io::{Error, ErrorKind};
use std::io::Cursor;
use std::io::BufReader;
use std::io::{Read, Write, Stdout, Stdin};
use std::io::{stdout, stdin};

use std::collections::HashMap;

use hyper::Client;
use std::sync::{Arc, Mutex};
use std::thread;
use cancellation::{CancellationToken, CancellationTokenSource, OperationCanceled};
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Status {
    List,
    Show,
}

fn main() {

    // Initialize 'em all.
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    // Clear the screen.
    print!("{}", termion::clear::All); // stdout.clear().unwrap();

    let title = String::from("高登");
    let icon_manifest_string = cache::readfile(String::from("data/icon.manifest.json"));
    let icon_collection: Vec<IconItem> = json::decode(&icon_manifest_string).unwrap();

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
    let mut prev_width = terminal_size().unwrap().0; //rustbox.width();

    let mut index = hkg::screen::index::Index::new();
    let mut show = hkg::screen::show::Show::new();

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
                               tx_res.send(page_request(&item, &mut wr, &ct)).unwrap();
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
            print!("{}", termion::clear::All); // stdout.clear().unwrap(); // hkg::screen::common::clear(&rustbox); // clear screen when switching state
            prev_state = state;
        }

        match rx_res.try_recv() {
            Ok(item) => {
                let document = kuchiki::parse_html().from_utf8().one(item.result.as_bytes());

                let posturl = get_posturl(&item.postid, item.page);
                show_item = builder.show_item(&document, &posturl);

                let w = terminal_size().unwrap().0 as usize; //rustbox.width();
                status = format_status(status,
                                       w,
                                       &format!("[{}-{}:ROK][{}]",
                                                show_item.url_query.message,
                                                show_item.page,
                                                is_web_requesting));

                show.resetY(); // show.resetY();
                print!("{}", termion::clear::All); // stdout.clear().unwrap();  // hkg::screen::common::clear(&rustbox);
                state = Status::Show;
                is_web_requesting = false;
            }
            Err(e) => {}
        }

        match state {
            Status::List => {
                // list.print(&title, &collection);
                index.print(&mut stdout, &collection);
            }
            Status::Show => {
                // show.print(&title, &show_item);
                show.print(&mut stdout, &title, &show_item);
            }
        }

        print_status(&mut stdout, &status); // print_status(&rustbox, &status);

        stdout.flush().unwrap();         // rustbox.present();

        if !is_web_requesting {

            let stdin = stdin();

            for c in stdin.keys() {

                if prev_width != terminal_size().unwrap().0 {
                    print!("{}", termion::clear::All); // stdout.clear().unwrap(); //hkg::screen::common::clear(&rustbox);
                   prev_width = terminal_size().unwrap().0;
                }

                let w = terminal_size().unwrap().0;
                match c.unwrap() {
                    Key::Char('q') => {
                        print!("{}{}{}", termion::clear::All, style::Reset, termion::cursor::Show); // stdout.clear().unwrap();
                        return
                    },
                    Key::Char('\n') => {
                        // status = format_status(status, w as usize, &format!("ENTER"));
                        status = format_status(status, w as usize, "ENTER");
                        match state {
                            Status::List => {
                                let i = index.get_selected_topic();
                                if i > 0 {
                                    let topic_item = &collection[i - 1];
                                    let postid = &topic_item.title.url_query.message;
                                    let page = 1;
                                    let status_message = show_page(&postid, page, &mut is_web_requesting, &tx_req);
                                    status = format_status(status.clone(),
                                                           w as usize,
                                                           &get_show_page_status_message(postid, page, &status_message));
                                }
                            }
                            Status::Show => {}
                        }
                        break
                    },
                    Key::Alt(c) => {
                        status = format_status(status, w as usize, &format!("^{}", c));
                        break
                    },
                    Key::Ctrl(c) => {
                        status = format_status(status, w as usize, &format!("*{}", c));
                        break
                    },
                    Key::Left => {
                        status = format_status(status, w as usize, &format!("←"));
                        match state {
                            Status::List => {}
                            Status::Show => {
                                if show_item.page > 1 {
                                    let postid = &show_item.url_query.message;
                                    let page = &show_item.page - 1;
                                    let status_message = show_page(&postid, page, &mut is_web_requesting, &tx_req);
                                    status = format_status(status.clone(),
                                                           w as usize,
                                                           &get_show_page_status_message(postid, page, &status_message));
                                }
                            }
                        }
                        break
                    }
                    Key::Right => {
                        status = format_status(status, w as usize, &format!("→"));
                        match state {
                            Status::List => {}
                            Status::Show => {
                                if show_item.max_page > show_item.page {
                                    let postid = &show_item.url_query.message;
                                    let page = &show_item.page + 1;
                                    let status_message = show_page(&postid, page, &mut is_web_requesting, &tx_req);
                                    status = format_status(status.clone(),
                                                           w as usize,
                                                           &get_show_page_status_message(postid, page, &status_message));
                                }
                            }
                        }
                        break
                    },
                    Key::PageUp => {
                        status = format_status(status, w as usize, "↑");

                        match state {
                            Status::List => {
                                let tmp = index.get_selected_topic();
                                status = format_status(status, w as usize, &format!("{}", tmp));

                                if tmp > 1 {
                                    index.select_topic(tmp - 1);
                                }
                            }
                            Status::Show => {
                                let bh = show.body_height();
                                if show.scrollUp(bh) {
                                    print!("{}", termion::clear::All); // hkg::screen::common::clear(&rustbox);
                                }
                            }
                        }

                        break
                    },
                    Key::PageDown => {
                        status = format_status(status, w as usize, "↓");

                        match state {
                            Status::List => {}
                            Status::Show => {
                                let bh = show.body_height();
                                if show.scrollDown(bh) {
                                    print!("{}", termion::clear::All); //hkg::screen::common::clear(&rustbox);
                                }
                            }
                        }
                        break
                    },
                    Key::Up => {
                        status = format_status(status, w as usize, "↑");

                        match state {
                            Status::List => {
                                let tmp = index.get_selected_topic();
                                status = format_status(status, w as usize, &format!("{}", tmp));

                                if tmp > 1 {
                                    index.select_topic(tmp - 1);
                                }
                            }
                            Status::Show => {
                                if show.scrollUp(2) {
                                    print!("{}", termion::clear::All); // stdout.clear().unwrap(); // hkg::screen::common::clear(&rustbox);
                                }
                            }
                        }

                        break
                    },
                    Key::Down => {
                        status = format_status(status, w as usize, "↓");

                        match state {
                            Status::List => {
                                let tmp = index.get_selected_topic();
                                status = format_status(status, w as usize, &format!("{}", tmp));

                                if tmp < index.body_height() {
                                    index.select_topic(tmp + 1);
                                }
                            }
                            Status::Show => {
                                if show.scrollDown(2) {
                                    print!("{}", termion::clear::All); // stdout.clear().unwrap(); //hkg::screen::common::clear(&rustbox);
                                }
                            }
                        }
                        break
                    },
                    Key::Backspace => {
                        // status = format_status(status, w as usize, &format!("×"));
                        status = format_status(status, w as usize, "B");
                        match state {
                            Status::List => {}
                            Status::Show => {
                                state = Status::List;
                                print!("{}", termion::clear::All); 
                            }
                        }
                        break
                    },
                    Key::Char(c) => { status = format_status(status, w as usize, &format!(" {}", c));break },
                    // Key::Invalid => {
                    //     status = format_status(status, w as usize, &format!("???"));
                    //     break
                    // },
                    _ => {},
                }
            }
        }

    }
}

fn read_cache<P: AsRef<Path>, S: AsRef<Path>>(cache_path: P,
                                              file_name: S)
                                              -> Result<String, String> {
    let file_path = cache_path.as_ref().join(file_name);
    let mut file = try!(File::open(file_path).map_err(|e| e.to_string()));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents).map_err(|e| e.to_string()));
    Ok(contents)
}

fn write_cache<P: AsRef<Path>, S: AsRef<Path>>(cache_path: P,
                                               file_name: S,
                                               s: String)
                                               -> Result<(), String> {
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

fn page_request(item: &ChannelItem,
                wr: &mut WebResource,
                ct: &CancellationTokenSource)
                -> ChannelItem {

    let html_path = format!("data/html/{postid}/", postid = item.postid);
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

    result_item

}

fn show_page(postid: &String, page: usize, is_web_requesting: &mut bool, tx_req: &Sender<ChannelItem>) -> String {
    let posturl = get_posturl(postid, page);

    let ci = ChannelItem {
        postid: postid.clone(),
        page: page,
        result: String::from(""),
    };

    let status_message = match tx_req.send(ci) {
        Ok(()) => {
            *is_web_requesting = true;
            "SOK".to_string()
        }
        Err(e) => format!("{}:{}", "SFAIL", e).to_string(),
    };

    status_message
}

fn get_show_page_status_message(postid: &String, page: usize, status_message: &String) -> String {
    format!("[{}-{}:{}]", postid, page, status_message)
}

fn print_status(stdout: &mut termion::raw::RawTerminal<std::io::StdoutLock>, status: &str) {
    // // for status bar only
    let w = terminal_size().unwrap().0; // let w = rustbox.width();
    let h = terminal_size().unwrap().1; // let h = rustbox.height();

    write!(stdout, "{}{}{}{}{}{}",
            termion::cursor::Goto(1, h),
            color::Fg(color::White),
            style::Bold,
            format!("{status}", status = status),
            style::Reset,
            termion::cursor::Hide);
}

fn format_status(status: String, w: usize, s: &str) -> String {
    if status.len() >= w {
        String::from(format!("{}{}", &"> ", s))
    } else {
        String::from(format!("{}{}", &status, s))
    }
}

// fn show_item_build_example(rustbox: &rustbox::RustBox, collection: &Vec<ListTopicItem>) {
//
//     rustbox.print(1,
//                   1,
//                   rustbox::RB_NORMAL,
//                   Color::White,
//                   Color::Black,
//                   &format!("before parse => {}", Local::now()));
//
//     let mut builder = hkg::builder::Builder::new();
//
//     let url = &collection[1].title.url;
//     rustbox.print(1, 2, rustbox::RB_NORMAL, Color::White, Color::Black, url);
//
//     let uqi = builder.url_query_item(&url);
//     let postid = "6360604"; //uqi.message;
//     let page = 1;
//     let path = format!("data/html/{postid}/show_{page}.html",
//                        postid = postid,
//                        page = page);
//
//     rustbox.print(1,
//                   3,
//                   rustbox::RB_NORMAL,
//                   Color::White,
//                   Color::Black,
//                   &format!("path: {}", path));
//
//     let show_item = match kuchiki::parse_html().from_utf8().from_file(&path) {
//         Ok(document) => Some(builder.show_item(&document, &url)),
//         Err(e) => None,
//     };
//
//     match show_item {
//         Some(si) => {
//
//             rustbox.print(1,
//                           5,
//                           rustbox::RB_NORMAL,
//                           Color::White,
//                           Color::Black,
//                           &format!("url_query->message: {} title:{} reploy count: {} page: {} \
//                                     max_page: {}",
//                                    si.url_query.message,
//                                    si.title,
//                                    si.reply_count,
//                                    si.page,
//                                    si.max_page));
//
//             for (index, item) in si.replies.iter().enumerate() {
//                 rustbox.print(1,
//                               index + 7,
//                               rustbox::RB_NORMAL,
//                               Color::White,
//                               Color::Black,
//                               &format!("{:<2}={:?}", index, item));
//             }
//         }
//         _ => {}
//     }
// }

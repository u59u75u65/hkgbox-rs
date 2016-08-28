extern crate hkg;
extern crate termion;
extern crate rustc_serialize;
extern crate kuchiki;
extern crate chrono;
extern crate hyper;
extern crate cancellation;
extern crate time;
extern crate url;

use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::{stdout, stdin, Read, Write};
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use cancellation::{CancellationToken, CancellationTokenSource, OperationCanceled};
use kuchiki::traits::*;
use rustc_serialize::json;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::terminal_size;
use termion::{color, style};
use hkg::status::*;
use hkg::utility::cache;
use hkg::model::IconItem;
use hkg::model::ListTopicItem;
use hkg::utility::client::*;
use hkg::state_manager::*;

fn main() {

    // Initialize
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    // Clear the screen.
    print!("{}", termion::clear::All);

    let mut builder = hkg::builder::Builder::new();
    let mut status = String::from("> ");

    let mut sm = StateManager::new();
    // let mut state = Status::Startup;
    // let mut prev_state = state;
    // let mut prev_width = terminal_size().unwrap().0;
    // let mut is_web_requesting = false;

    let icon_manifest_string = cache::readfile(String::from("data/icon.manifest.json"));
    let icon_collection: Box<Vec<IconItem>> = Box::new(json::decode(&icon_manifest_string).unwrap());

    // initialize empty page
    let mut list_topic_items: Vec<ListTopicItem> = vec![];
    let mut show_item = builder.default_show_item();

    let mut index = hkg::screen::index::Index::new();
    let mut show = hkg::screen::show::Show::new(icon_collection);

    let (tx_req, rx_req) = channel::<ChannelItem>();
    let (tx_res, rx_res) = channel::<ChannelItem>();

    // web client
    thread::spawn(move || {
        let mut wr = WebResource::new();
        let ct = CancellationTokenSource::new();
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
                Err(_) => {}
            }
        }
    });

    // topics request
    let status_message = list_page(&mut sm, &tx_req);
    status = format_status(status.clone(), &status_message);

    loop {

        // show UI
        if sm.isStateChanged() {
            print!("{}", termion::clear::All); // clear screen when switching state
        }

        match rx_res.try_recv() {
            Ok(item) => {
                match item.extra {
                    ChannelItemType::Show(extra) => {
                        let document = kuchiki::parse_html().from_utf8().one(item.result.as_bytes());

                        let posturl = get_posturl(&extra.postid, extra.page);
                        show_item = builder.show_item(&document, &posturl);

                        status = format_status(status,
                                               &format!("[{}-{}:ROK][{}]",
                                                        show_item.url_query.message,
                                                        show_item.page,
                                                        sm.isWebRequest()));

                        show.resetY();
                        print!("{}", termion::clear::All);
                        sm.updateState(Status::Show); //state = Status::Show;
                        sm.setWebRequest(false); // is_web_requesting = false;
                    },
                    ChannelItemType::Index(_) => {
                        let document = kuchiki::parse_html().from_utf8().one(item.result.as_bytes());

                        list_topic_items = builder.list_topic_items(&document);

                        status = format_status(status,
                                               &format!("[TOPICS:ROK]"));

                        print!("{}", termion::clear::All);

                        sm.updateState(Status::List); // state = Status::List;
                        sm.setWebRequest(false); // is_web_requesting = false;

                    }
                }
            }
            Err(_) => {}
        }

        match sm.getState() {
            Status::Startup => {

            },
            Status::List => {
                index.print(&mut stdout, &list_topic_items);
            }
            Status::Show => {
                show.print(&mut stdout, &show_item);
            }
        }

        print_status(&mut stdout, &status);

        stdout.flush().unwrap();

        if !sm.isWebRequest() {

            let stdin = stdin();

            for c in stdin.keys() {

                sm.updateWidth();
                if sm.isWidthChanged() {
                    print!("{}", termion::clear::All);
                }

                match sm.getState() {
                    Status::Startup => {},
                    Status::List => {
                        match c.unwrap() {
                            Key::Char('q') => {
                                print!("{}{}{}", termion::clear::All, style::Reset, termion::cursor::Show);
                                return
                            },
                            Key::Char('\n') => {
                                status = format_status(status, "ENTER");
                                let i = index.get_selected_topic();
                                if i > 0 {
                                    let topic_item = &list_topic_items[i - 1];
                                    let postid = &topic_item.title.url_query.message;
                                    let page = 1;
                                    let status_message = show_page(&postid, page, &mut sm, &tx_req);

                                    status = format_status(status.clone(),
                                                           &get_show_page_status_message(postid, page, &status_message));
                                }
                                break
                            },
                            Key::PageUp => {
                                status = format_status(status, "↑");
                                let tmp = index.get_selected_topic();
                                status = format_status(status, &format!("{}", tmp));

                                if tmp > 1 {
                                    index.select_topic(tmp - 1);
                                }
                                break
                            },
                            Key::Up => {
                                status = format_status(status, "↑");
                                let tmp = index.get_selected_topic();
                                status = format_status(status, &format!("{}", tmp));

                                if tmp > 1 {
                                    index.select_topic(tmp - 1);
                                }
                                break
                            },
                            Key::Down => {
                                status = format_status(status, "↓");
                                let tmp = index.get_selected_topic();
                                status = format_status(status, &format!("{}", tmp));

                                if tmp < index.body_height() {
                                    index.select_topic(tmp + 1);
                                }
                                break
                            },
                            _ => {},
                        }
                    },
                    Status::Show => {
                            match c.unwrap() {
                                Key::Char('q') => {
                                    print!("{}{}{}", termion::clear::All, style::Reset, termion::cursor::Show);
                                    return
                                },
                                Key::Left => {
                                    status = format_status(status, &format!("←"));
                                    if show_item.page > 1 {
                                        let postid = &show_item.url_query.message;
                                        let page = &show_item.page - 1;
                                        let status_message = show_page(&postid, page, &mut sm, &tx_req);

                                        status = format_status(status.clone(),
                                                               &get_show_page_status_message(postid, page, &status_message));
                                    }
                                    break
                                }
                                Key::Right => {
                                    status = format_status(status, &format!("→"));
                                    if show_item.max_page > show_item.page {
                                        let postid = &show_item.url_query.message;
                                        let page = &show_item.page + 1;
                                        let status_message = show_page(&postid, page, &mut sm, &tx_req);

                                        status = format_status(status.clone(),
                                                               &get_show_page_status_message(postid, page, &status_message));
                                    }
                                    break
                                },
                                Key::PageUp => {
                                    status = format_status(status, "↑");
                                    let bh = show.body_height();
                                    if show.scrollUp(bh) {
                                        print!("{}", termion::clear::All);
                                    }
                                    break
                                },
                                Key::PageDown => {
                                    status = format_status(status, "↓");
                                    let bh = show.body_height();
                                    if show.scrollDown(bh) {
                                        print!("{}", termion::clear::All);
                                    }
                                    break
                                },
                                Key::Up => {
                                    status = format_status(status, "↑");
                                    if show.scrollUp(2) {
                                        print!("{}", termion::clear::All);
                                    }
                                    break
                                },
                                Key::Down => {
                                    status = format_status(status, "↓");
                                    if show.scrollDown(2) {
                                        print!("{}", termion::clear::All);
                                    }
                                    break
                                },
                                Key::Backspace => {
                                    status = format_status(status, "B");
                                    sm.updateState(Status::List); // state = Status::List;
                                    print!("{}", termion::clear::All);
                                    break
                                },
                                _ => {},
                            }
                        }
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

fn get_topic_bw_url() -> String {
    let base_url = "http://forum1.hkgolden.com";
    let url = format!("{base_url}/topics_bw.htm", base_url = base_url);
    url
}

fn page_request(item: &ChannelItem,
                wr: &mut WebResource,
                ct: &CancellationTokenSource)
                -> ChannelItem {

    match item.extra.clone() {
        ChannelItemType::Show(extra) => {
            let html_path = format!("data/html/{postid}/", postid = extra.postid);
            let show_file_name = format!("show_{page}.html", page = extra.page);

            let postid = extra.postid.clone();

            let (from_cache, result) = match read_cache(&html_path, &show_file_name) {
                Ok(result) => (true, result),
                Err(_) => {
                    let posturl = get_posturl(&extra.postid, extra.page);
                    let result = wr.get(&posturl);
                    (false, result)
                }
            };

            if !from_cache {
                let result2 = result.clone();
                write_cache(&html_path, &show_file_name, result2);
            }

            let result_item = ChannelItem {
                extra: ChannelItemType::Show(ChannelShowItem { postid: postid, page: extra.page }),
                result: result,
            };
            result_item
        },
        ChannelItemType::Index(_) => {

            let time_format = |t: time::Tm| {
                match t.strftime("%Y%m%d%H%M") {
                    Ok(s) => s.to_string(),
                    Err(e) => panic!(e)
                }
            };

            let time = time_format(time::now());

            let html_path = format!("data/html/topics/");
            let file_name = format!("{time}.html", time = time);

            let (from_cache, result) = match read_cache(&html_path, &file_name) {
                Ok(result) => (true, result),
                Err(_) => {
                    let url = get_topic_bw_url();
                    let result = wr.get(&url);
                    (false, result)
                }
            };

            if !from_cache {
                let result2 = result.clone();
                write_cache(&html_path, &file_name, result2);
            }

            let result_item = ChannelItem {
                extra: ChannelItemType::Index(ChannelIndexItem { }),
                result: result,
            };
            result_item
        }
    }

}

fn list_page(sm: &mut StateManager, tx_req: &Sender<ChannelItem>) -> String {

    let ci = ChannelItem {
        extra: ChannelItemType::Index(ChannelIndexItem { }),
        result: String::from(""),
    };

    let status_message = match tx_req.send(ci) {
        Ok(()) => {
            sm.setWebRequest(true);    // *is_web_requesting = true;
            "SOK".to_string()
        }
        Err(e) => format!("{}:{}", "SFAIL", e).to_string(),
    };

    status_message
}

fn show_page(postid: &String, page: usize, sm: &mut StateManager, tx_req: &Sender<ChannelItem>) -> String {

    let ci = ChannelItem {
        extra: ChannelItemType::Show(ChannelShowItem { postid: postid.clone(), page: page }),
        result: String::from(""),
    };

    let status_message = match tx_req.send(ci) {
        Ok(()) => {
            sm.setWebRequest(true); // *is_web_requesting = true;
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
    // for status bar only
    let h = terminal_size().unwrap().1;

    write!(stdout, "{}{}{}{}{}{}",
            termion::cursor::Goto(1, h),
            color::Fg(color::White),
            style::Bold,
            format!("{status}", status = status),
            style::Reset,
            termion::cursor::Hide);
}

fn format_status(status: String, s: &str) -> String {
    let w = terminal_size().unwrap().0 as usize;;
    if status.len() >= w {
        String::from(format!("{}{}", &"> ", s))
    } else {
        String::from(format!("{}{}", &status, s))
    }
}

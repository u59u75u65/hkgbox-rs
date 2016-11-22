extern crate hkg;
extern crate termion;
extern crate rustc_serialize;
extern crate kuchiki;
extern crate chrono;
extern crate hyper;
extern crate cancellation;
extern crate time;
extern crate url;

use std::io::{stdout, stdin, Write};
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use cancellation::{CancellationToken, CancellationTokenSource, OperationCanceled};
use kuchiki::traits::*;
use rustc_serialize::json;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::event::Key;
use hkg::status::*;
use hkg::model::IconItem;
use hkg::model::ListTopicItem;
use hkg::state_manager::*;
use hkg::screen_manager::*;
use hkg::resources::common::*;
use hkg::caches::file_cache::*;
use hkg::net::*;
use hkg::net::web_resource::*;

fn main() {

    // Initialize
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    // Clear the screen.
    hkg::screen::common::clear_screen();

    let mut builder = hkg::builder::Builder::new();

    let mut state_manager = StateManager::new();
    let mut screen_manager = ScreenManager::new();

    let icon_manifest_string = hkg::utility::readfile(String::from("data/icon.manifest.json"));
    let icon_collection: Box<Vec<IconItem>> = Box::new(json::decode(&icon_manifest_string).unwrap());

    // initialize empty page
    let mut list_topic_items: Vec<ListTopicItem> = vec![];
    let mut show_item = builder.default_show_item();

    let mut status_bar = hkg::screen::status_bar::StatusBar::new();
    let mut index = hkg::screen::index::Index::new();
    let mut show = hkg::screen::show::Show::new(icon_collection);

    let (tx_req, rx_req) = channel::<ChannelItem>();
    let (tx_res, rx_res) = channel::<ChannelItem>();

    // web client
    thread::spawn(move || {
        let mut wr = WebResource::new();
        let mut fc = Box::new(FileCache::new());
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
                                match item.extra.clone() {
                                    ChannelItemType::Index(_) => {
                                        let mut index_resource = hkg::resources::index_resource::IndexResource::new(&mut wr, &ct, &mut fc);
                                        tx_res.send(index_resource.fetch(&item)).unwrap();
                                    },
                                    ChannelItemType::Show(_) => {
                                        let mut show_resource = hkg::resources::show_resource::ShowResource::new(&mut wr, &ct, &mut fc);
                                        tx_res.send(show_resource.fetch(&item)).unwrap();
                                    },
                                    ChannelItemType::Image(_) => {
                                    }
                                }
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
    let status_message = list_page(&mut state_manager, &tx_req);
    status_bar.append(&screen_manager, &status_message);

    loop {

        match rx_res.try_recv() {
            Ok(item) => {
                match item.extra {
                    ChannelItemType::Show(extra) => {
                        let document = kuchiki::parse_html().from_utf8().one(item.result.as_bytes());

                        let posturl = get_posturl(&extra.postid, extra.page);
                        show_item = builder.show_item(&document, &posturl);

                        status_bar.append(&screen_manager, &format!("[{}-{}:ROK][{}]",
                                 show_item.url_query.message,
                                 show_item.page,
                                 state_manager.isWebRequest()));

                        show.resetY();
                        hkg::screen::common::clear_screen();
                        state_manager.updateState(Status::Show); //state = Status::Show;
                        state_manager.setWebRequest(false); // is_web_requesting = false;
                    },
                    ChannelItemType::Index(_) => {
                        let document = kuchiki::parse_html().from_utf8().one(item.result.as_bytes());

                        list_topic_items = builder.list_topic_items(&document);

                        status_bar.append(&screen_manager,
                                               &format!("[TOPICS:ROK]"));

                        hkg::screen::common::clear_screen();

                        state_manager.updateState(Status::List); // state = Status::List;
                        state_manager.setWebRequest(false); // is_web_requesting = false;

                    },
                    ChannelItemType::Image(extra) => {
                    }
                }
            }
            Err(_) => {}
        }

        match state_manager.getState() {
            Status::Startup => {

            },
            Status::List => {
                index.print(&mut stdout, &list_topic_items);
            }
            Status::Show => {
                show.print(&mut stdout, &show_item);
            }
        }

        status_bar.print(&screen_manager);

        stdout.flush().unwrap();

        if !state_manager.isWebRequest() {

            let stdin = stdin();

            for c in stdin.keys() {

                if screen_manager.isWidthChanged() {
                    hkg::screen::common::clear_screen();
                }

                match state_manager.getState() {
                    Status::Startup => {},
                    Status::List => {
                        match c.unwrap() {
                            Key::Char('q') => {
                                hkg::screen::common::reset_screen();
                                return
                            },
                            Key::Char('\n') => {
                                status_bar.append(&screen_manager, "ENTER");
                                let i = index.get_selected_topic();
                                if i > 0 {
                                    let topic_item = &list_topic_items[i - 1];
                                    let postid = &topic_item.title.url_query.message;
                                    let page = 1;
                                    let status_message = show_page(&postid, page, &mut state_manager, &tx_req);

                                    status_bar.append(&screen_manager,
                                                           &get_show_page_status_message(postid, page, &status_message));
                                }
                                break
                            },
                            Key::PageUp => {
                                status_bar.append(&screen_manager, "↑");
                                let tmp = index.get_selected_topic();
                                status_bar.append(&screen_manager, &format!("{}", tmp));

                                if tmp > 1 {
                                    index.select_topic(tmp - 1);
                                }
                                break
                            },
                            Key::Up => {
                                status_bar.append(&screen_manager, "↑");
                                let tmp = index.get_selected_topic();
                                status_bar.append(&screen_manager, &format!("{}", tmp));

                                if tmp > 1 {
                                    index.select_topic(tmp - 1);
                                }
                                break
                            },
                            Key::Down => {
                                status_bar.append(&screen_manager, "↓");
                                let tmp = index.get_selected_topic();
                                status_bar.append(&screen_manager, &format!("{}", tmp));

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
                                    hkg::screen::common::reset_screen(); // print!("{}{}{}", termion::clear::All, style::Reset, termion::cursor::Show);
                                    return
                                },
                                Key::Left => {
                                    status_bar.append(&screen_manager, &format!("←"));
                                    if show_item.page > 1 {
                                        let postid = &show_item.url_query.message;
                                        let page = &show_item.page - 1;
                                        let status_message = show_page(&postid, page, &mut state_manager, &tx_req);

                                        status_bar.append(&screen_manager,
                                                               &get_show_page_status_message(postid, page, &status_message));
                                    }
                                    break
                                }
                                Key::Right => {
                                    status_bar.append(&screen_manager, &format!("→"));
                                    if show_item.max_page > show_item.page {
                                        let postid = &show_item.url_query.message;
                                        let page = &show_item.page + 1;
                                        let status_message = show_page(&postid, page, &mut state_manager, &tx_req);

                                        status_bar.append(&screen_manager,
                                                               &get_show_page_status_message(postid, page, &status_message));
                                    }
                                    break
                                },
                                Key::PageUp => {
                                    status_bar.append(&screen_manager, "↑");
                                    let bh = show.body_height();
                                    if show.scrollUp(bh) {
                                    hkg::screen::common::clear_screen();
                                    }
                                    break
                                },
                                Key::PageDown => {
                                    status_bar.append(&screen_manager, "↓");
                                    let bh = show.body_height();
                                    if show.scrollDown(bh) {
                                        hkg::screen::common::clear_screen();
                                    }
                                    break
                                },
                                Key::Up => {
                                    status_bar.append(&screen_manager, "↑");
                                    if show.scrollUp(2) {
                                        hkg::screen::common::clear_screen();
                                    }
                                    break
                                },
                                Key::Down => {
                                    status_bar.append(&screen_manager, "↓");
                                    if show.scrollDown(2) {
                                        hkg::screen::common::clear_screen();
                                    }
                                    break
                                },
                                Key::Backspace => {
                                    status_bar.append(&screen_manager, "B");
                                    state_manager.updateState(Status::List); // state = Status::List;
                                    hkg::screen::common::clear_screen();
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
fn get_posturl(postid: &String, page: usize) -> String {
    let base_url = "http://forum1.hkgolden.com/view.aspx";
    let posturl = format!("{base_url}?type=BW&message={postid}&page={page}",
                          base_url = base_url,
                          postid = postid,
                          page = page);
    posturl
}

fn list_page(state_manager: &mut StateManager, tx_req: &Sender<ChannelItem>) -> String {

    let ci = ChannelItem {
        extra: ChannelItemType::Index(ChannelIndexItem { }),
        result: String::from(""),
    };

    let status_message = match tx_req.send(ci) {
        Ok(()) => {
            state_manager.setWebRequest(true);    // *is_web_requesting = true;
            "SOK".to_string()
        }
        Err(e) => format!("{}:{}", "SFAIL", e).to_string(),
    };

    status_message
}

fn show_page(postid: &String, page: usize, state_manager: &mut StateManager, tx_req: &Sender<ChannelItem>) -> String {

    let ci = ChannelItem {
        extra: ChannelItemType::Show(ChannelShowItem { postid: postid.clone(), page: page }),
        result: String::from(""),
    };

    let status_message = match tx_req.send(ci) {
        Ok(()) => {
            state_manager.setWebRequest(true); // *is_web_requesting = true;
            "SOK".to_string()
        }
        Err(e) => format!("{}:{}", "SFAIL", e).to_string(),
    };

    status_message
}

fn get_show_page_status_message(postid: &String, page: usize, status_message: &String) -> String {
    format!("[{}-{}:{}]", postid, page, status_message)
}

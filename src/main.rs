extern crate hkg;
extern crate termion;
extern crate rustc_serialize;
extern crate kuchiki;
extern crate chrono;
extern crate cancellation;

#[macro_use]
extern crate log;
extern crate log4rs;

use std::io::{stdout, stdin, Write};
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
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
use hkg::caches::file_cache::*;
use hkg::resources::*;
use hkg::resources::web_resource::*;
use hkg::resources::common::*;
use hkg::web::*;

use log4rs::*;

fn main() {

    // Initialize
    log4rs::init_file("config/log4rs.yaml", Default::default());

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().expect("fail to lock stdout");

    // Clear the screen.
    hkg::screen::common::clear_screen();

    // web background services
    let (tx_req, rx_req) = channel::<ChannelItem>();
    let (tx_res, rx_res) = channel::<ChannelItem>();

    let mut app = {

        let icon_collection: Box<Vec<IconItem>> = {
            let icon_manifest_string = hkg::utility::readfile(String::from("data/icon.manifest.json"));
            Box::new(json::decode(&icon_manifest_string).expect("fail to lock stdout"))
        };

        hkg::App {
            builder: hkg::builder::Builder::new(),
            state_manager: StateManager::new(),
            screen_manager: ScreenManager::new(),

            // initialize empty page
            list_topic_items: vec![],
            show_item: hkg::builder::Builder::new().default_show_item(),

            status_bar: hkg::screen::status_bar::StatusBar::new(),
            index: hkg::screen::index::Index::new(),
            show: hkg::screen::show::Show::new(icon_collection),
            image_request_count_lock: Arc::new(Mutex::new(0)),
            is_bg_request: false,
            tx_req: &tx_req,
            rx_res: &rx_res
        }
    };

    Requester::new(rx_req, tx_res);

    // topics request
    let status_message = list_page(&mut app.state_manager, &tx_req);
    app.status_bar.append(&app.screen_manager, &status_message);

    loop {

        match app.rx_res.try_recv() {
            Ok(item) => {
                match item.extra {
                    ChannelItemType::Show(extra) => {
                        let document = kuchiki::parse_html().from_utf8().one(item.result.as_bytes());

                        let posturl = get_posturl(&extra.postid, extra.page);
                        app.show_item = app.builder.show_item(&document, &posturl);

                        app.status_bar.append(&app.screen_manager, &format!("[{}-{}:ROK][{}]",
                                 app.show_item.url_query.message,
                                 app.show_item.page,
                                 app.state_manager.isWebRequest()));

                        // get all images links in an array, and send to background download
                        let maps = app.show_item.replies.iter().flat_map(|reply|
                            {
                                 let f = reply.body.iter().filter(|node| {
                                     let node2 = node.clone();
                                     match *node2 {
                                         hkg::reply_model::NodeType::Image(ref n) => (n.data.starts_with("http") || n.data.starts_with("https")) && n.alt.starts_with("[img]") && n.alt.ends_with("[/img]"),
                                         _ => false
                                     }
                                 }).collect::<Vec<_>>();
                                 return f;
                            }
                        ).collect::<Vec<_>>();

                        let mut count = app.image_request_count_lock.lock().expect("fail to lock image request count");
                        *count = maps.len();
                        app.is_bg_request = true;
                        app.status_bar.append(&app.screen_manager,
                                                &format!("[SIMG:{count}]", count = *count ));

                        for node in &maps {
                            let node2 = node.clone();
                            match *node2 {
                                 hkg::reply_model::NodeType::Image(ref n) => {
                                     let status_message = image_request(&n.data, &mut app.state_manager, &app.tx_req);
                                     app.status_bar.append(&app.screen_manager,&status_message);
                                 },
                                 _ => {}
                            }
                        }

                        app.show.resetY();
                        hkg::screen::common::clear_screen();
                        app.state_manager.updateState(Status::Show); //state = Status::Show;
                        app.state_manager.setWebRequest(false); // is_web_requesting = false;
                    },
                    ChannelItemType::Index(_) => {
                        let document = kuchiki::parse_html().from_utf8().one(item.result.as_bytes());

                        app.list_topic_items.clear();
                        for item in app.builder.list_topic_items(&document) {
                            app.list_topic_items.push(item);
                        }

                        app.status_bar.append(&app.screen_manager,
                                               &format!("[TOPICS:ROK]"));

                        hkg::screen::common::clear_screen();

                        app.state_manager.updateState(Status::List); // state = Status::List;
                        app.state_manager.setWebRequest(false); // is_web_requesting = false;

                    },
                    ChannelItemType::Image(extra) => {
                        match app.image_request_count_lock.lock() {
                            Ok(mut count) => {
                                if *count == 0 {
                                    app.status_bar.append(&app.screen_manager,
                                                           &format!("[RIMG:CERR]"));
                                } else {
                                    *count -= 1;
                                    if (item.result != "") {
                                        app.status_bar.append(&app.screen_manager,
                                                           &format!("[RIMG:E-{count}-{error}]", count = *count, error = item.result ));
                                    } else {
                                        app.status_bar.append(&app.screen_manager,
                                                           &format!("[RIMG:S-{count}]", count = *count ));
                                    }
                                }

                                if *count <= 0 {
                                    app.is_bg_request = false;
                                    hkg::screen::common::clear_screen();
                                    app.state_manager.setWebRequest(false); // is_web_requesting = false;
                                }
                            },
                            Err(poisoned) => { app.status_bar.append(&app.screen_manager,
                                                   &format!("[IMAGES:LOCKERR]")); }
                        };
                    }
                }
            }
            Err(_) => {}
        }

        match app.state_manager.getState() {
            Status::Startup => {

            },
            Status::List => {
                app.index.print(&mut stdout, &app.list_topic_items);
            }
            Status::Show => {
                app.show.print(&mut stdout, &app.show_item);
            }
        }

        app.status_bar.print(&app.screen_manager);

        stdout.flush().expect("fail to flush the stdout");

        if !app.state_manager.isWebRequest() {

            let stdin = stdin();

            for c in stdin.keys() {

                if app.screen_manager.isWidthChanged() {
                    hkg::screen::common::clear_screen();
                }

                match app.state_manager.getState() {
                    Status::Startup => {},
                    Status::List => {
                        match c.ok().expect("fail to get stdin keys") {
                            Key::Char('q') => {
                                hkg::screen::common::reset_screen();
                                return
                            },
                            Key::Char('\n') => {
                                app.status_bar.append(&app.screen_manager, "ENTER");
                                let i = app.index.get_selected_topic();
                                if i > 0 {
                                    let topic_item = &app.list_topic_items[i - 1];
                                    let postid = &topic_item.title.url_query.message;
                                    let page = 1;
                                    let status_message = show_page(&postid, page, &mut app.state_manager, &app.tx_req);

                                    app.status_bar.append(&app.screen_manager,
                                                           &get_show_page_status_message(postid, page, &status_message));
                                }
                                break
                            },
                            Key::PageUp => {
                                app.status_bar.append(&app.screen_manager, "↑");
                                let tmp = app.index.get_selected_topic();
                                app.status_bar.append(&app.screen_manager, &format!("{}", tmp));

                                if tmp > 1 {
                                    app.index.select_topic(tmp - 1);
                                }
                                break
                            },
                            Key::Up => {
                                app.status_bar.append(&app.screen_manager, "↑");
                                let tmp = app.index.get_selected_topic();
                                app.status_bar.append(&app.screen_manager, &format!("{}", tmp));

                                if tmp > 1 {
                                    app.index.select_topic(tmp - 1);
                                }
                                break
                            },
                            Key::Down => {
                                app.status_bar.append(&app.screen_manager, "↓");
                                let tmp = app.index.get_selected_topic();
                                app.status_bar.append(&app.screen_manager, &format!("{}", tmp));

                                if tmp < app.index.body_height() {
                                    app.index.select_topic(tmp + 1);
                                }
                                break
                            },
                            _ => {},
                        }
                    },
                    Status::Show => {
                            match c.ok().expect("fail to get stdin keys") {
                                Key::Char('q') => {
                                    hkg::screen::common::reset_screen(); // print!("{}{}{}", termion::clear::All, style::Reset, termion::cursor::Show);
                                    return
                                },
                                Key::Left => {
                                    app.status_bar.append(&app.screen_manager, &format!("←"));
                                    if app.show_item.page > 1 {
                                        let postid = &app.show_item.url_query.message;
                                        let page = &app.show_item.page - 1;
                                        let status_message = show_page(&postid, page, &mut app.state_manager, &app.tx_req);

                                        app.status_bar.append(&app.screen_manager,
                                                               &get_show_page_status_message(postid, page, &status_message));
                                    }
                                    break
                                }
                                Key::Right => {
                                    app.status_bar.append(&app.screen_manager, &format!("→"));
                                    if app.show_item.max_page > app.show_item.page {
                                        let postid = &app.show_item.url_query.message;
                                        let page = &app.show_item.page + 1;
                                        let status_message = show_page(&postid, page, &mut app.state_manager, &app.tx_req);

                                        app.status_bar.append(&app.screen_manager,
                                                               &get_show_page_status_message(postid, page, &status_message));
                                    }
                                    break
                                },
                                Key::PageUp => {
                                    app.status_bar.append(&app.screen_manager, "↑");
                                    let bh = app.show.body_height();
                                    if app.show.scrollUp(bh) {
                                    hkg::screen::common::clear_screen();
                                    }
                                    break
                                },
                                Key::PageDown => {
                                    app.status_bar.append(&app.screen_manager, "↓");
                                    let bh = app.show.body_height();
                                    if app.show.scrollDown(bh) {
                                        hkg::screen::common::clear_screen();
                                    }
                                    break
                                },
                                Key::Up => {
                                    app.status_bar.append(&app.screen_manager, "↑");
                                    if app.show.scrollUp(2) {
                                        hkg::screen::common::clear_screen();
                                    }
                                    break
                                },
                                Key::Down => {
                                    app.status_bar.append(&app.screen_manager, "↓");
                                    if app.show.scrollDown(2) {
                                        hkg::screen::common::clear_screen();
                                    }
                                    break
                                },
                                Key::Backspace => {
                                    app.status_bar.append(&app.screen_manager, "B");
                                    app.state_manager.updateState(Status::List); // state = Status::List;
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

fn image_request(url: &String, state_manager: &mut StateManager, tx_req: &Sender<ChannelItem>) -> String {

    let ci = ChannelItem {
        extra: ChannelItemType::Image(ChannelImageItem { url: url.to_string(), bytes: Vec::new() }),
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

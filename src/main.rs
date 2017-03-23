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
use hkg::responser::*;
use log4rs::*;

fn main() {

    // Initialize
    log4rs::init_file("config/log4rs.yaml", Default::default());

    // Clear the screen.
    hkg::screen::common::clear_screen();

    let _stdout = stdout();

    // web background services
    let (tx_req, rx_req) = channel::<ChannelItem>();
    let (tx_res, rx_res) = channel::<ChannelItem>();

    let mut app = {

        let mut stdout = {
            Box::new(_stdout.lock().into_raw_mode().expect("fail to lock stdout"))
        };

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
            rx_res: &rx_res,
            stdout: stdout
        }
    };

    Requester::new(rx_req, tx_res);

    let respsoner = Responser::new();

    // topics request
    let status_message = list_page(&mut app.state_manager, &tx_req);
    app.status_bar.append(&app.screen_manager, &status_message);

    loop {

        respsoner.try_recv(&mut app);

        print_screen(&mut app);

        app.status_bar.print(&app.screen_manager);

        app.stdout.flush().expect("fail to flush the stdout");

        if !app.state_manager.isWebRequest() {

            let stdin = stdin();

            for c in stdin.keys() {

                if app.screen_manager.isWidthChanged() {
                    hkg::screen::common::clear_screen();
                }

                match app.state_manager.getState() {
                    Status::Startup => {}
                    Status::List => {
                        match index_control(c.ok().expect("fail to get stdin keys"), &mut app) {
                            Some(i) => return if i == 0 { return } else { break },
                            None => {}
                        }
                    }
                    Status::Show => {
                        match show_control(c.ok().expect("fail to get stdin keys"), &mut app) {
                            Some(i) => return if i == 0 { return } else { break },
                            None => {}
                        }
                    }
                }


            }
        }
    }
}

fn list_page(state_manager: &mut StateManager, tx_req: &Sender<ChannelItem>) -> String {

    let ci = ChannelItem {
        extra: ChannelItemType::Index(ChannelIndexItem {}),
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
        extra: ChannelItemType::Show(ChannelShowItem {
                                         postid: postid.clone(),
                                         page: page,
                                     }),
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

fn print_screen(app: &mut hkg::App) {
    match app.state_manager.getState() {
        Status::Startup => {}
        Status::List => {
            app.index.print(&mut app.stdout, &app.list_topic_items);
        }
        Status::Show => {
            app.show.print(&mut app.stdout, &app.show_item);
        }
    }
}

fn show_control(c: termion::event::Key,app: &mut hkg::App) -> Option<i32> {
    match c {
        Key::Char('q') => {
            hkg::screen::common::reset_screen(); // print!("{}{}{}", termion::clear::All, style::Reset, termion::cursor::Show);
            Some(0)
        }
        Key::Left => {
            app.status_bar.append(&app.screen_manager, &format!("←"));
            if app.show_item.page > 1 {
                let postid = &app.show_item.url_query.message;
                let page = &app.show_item.page - 1;
                let status_message = show_page(&postid, page, &mut app.state_manager, &app.tx_req);

                app.status_bar.append(&app.screen_manager,
                                      &get_show_page_status_message(postid, page, &status_message));
            }
            Some(1)
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
            Some(1)
        }
        Key::PageUp => {
            app.status_bar.append(&app.screen_manager, "↑");
            let bh = app.show.body_height();
            if app.show.scrollUp(bh) {
                hkg::screen::common::clear_screen();
            }
            Some(1)
        }
        Key::PageDown => {
            app.status_bar.append(&app.screen_manager, "↓");
            let bh = app.show.body_height();
            if app.show.scrollDown(bh) {
                hkg::screen::common::clear_screen();
            }
            Some(1)
        }
        Key::Up => {
            app.status_bar.append(&app.screen_manager, "↑");
            if app.show.scrollUp(2) {
                hkg::screen::common::clear_screen();
            }
            Some(1)
        }
        Key::Down => {
            app.status_bar.append(&app.screen_manager, "↓");
            if app.show.scrollDown(2) {
                hkg::screen::common::clear_screen();
            }
            Some(1)
        }
        Key::Backspace => {
            app.status_bar.append(&app.screen_manager, "B");
            app.state_manager.updateState(Status::List); // state = Status::List;
            hkg::screen::common::clear_screen();
            Some(1)
        }
        _ => None,
    }
}


fn index_control(c: termion::event::Key,app: &mut hkg::App)-> Option<i32> {
    match c {
        Key::Char('q') => {
            hkg::screen::common::reset_screen();
            Some(0)
        }
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
            Some(1)
        }
        Key::PageUp => {
            app.status_bar.append(&app.screen_manager, "↑");
            let tmp = app.index.get_selected_topic();
            app.status_bar.append(&app.screen_manager, &format!("{}", tmp));

            if tmp > 1 {
                app.index.select_topic(tmp - 1);
            }
            Some(1)
        }
        Key::Up => {
            app.status_bar.append(&app.screen_manager, "↑");
            let tmp = app.index.get_selected_topic();
            app.status_bar.append(&app.screen_manager, &format!("{}", tmp));

            if tmp > 1 {
                app.index.select_topic(tmp - 1);
            }
            Some(1)
        }
        Key::Down => {
            app.status_bar.append(&app.screen_manager, "↓");
            let tmp = app.index.get_selected_topic();
            app.status_bar.append(&app.screen_manager, &format!("{}", tmp));

            if tmp < app.index.body_height() {
                app.index.select_topic(tmp + 1);
            }
            Some(1)
        }
        _ => None,
    }
}

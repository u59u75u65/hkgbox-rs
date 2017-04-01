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
use std::io::{self, Read};
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use rustc_serialize::json;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use hkg::status::*;
use hkg::model::IconItem;
use hkg::state_manager::*;
use hkg::screen_manager::*;
use hkg::resources::*;
use hkg::web::*;
use hkg::responser::*;
use std::thread;

fn main() {

    // Initialize
    log4rs::init_file("config/log4rs.yaml", Default::default()).expect("fail to init log4rs");

    // Clear the screen.
    hkg::screen::common::clear_screen();

    let _stdout = stdout();

    // web background services
    let (tx_req, rx_req) = channel::<ChannelItem>();
    let (tx_res, rx_res) = channel::<ChannelItem>();

    let (tx_state, rx_state) = channel::<(Status,Status)>();

    let working = Arc::new(AtomicBool::new(true));
    let control = Arc::downgrade(&working);

    let mut app = {

        let stdout = {
            Box::new(_stdout.lock().into_raw_mode().expect("fail to lock stdout"))
        };

        let icon_collection: Box<Vec<IconItem>> = {
            let icon_manifest_string = hkg::utility::readfile(String::from("data/icon.manifest.json"));
            Box::new(json::decode(&icon_manifest_string).expect("fail to lock stdout"))
        };

        hkg::App {
            index_builder: hkg::builders::index::Index::new(),
            show_builder: hkg::builders::show::Show::new(),
            state_manager: StateManager::new(tx_state),
            screen_manager: ScreenManager::new(),

            // initialize empty page
            list_topic_items: Default::default(),
            show_item: Default::default(),

            status_bar: hkg::screen::status_bar::StatusBar::new(),
            index: hkg::screen::index::Index::new(),
            show: hkg::screen::show::Show::new(icon_collection),

            image_request_count_lock: Arc::new(Mutex::new(0)),
            tx_req: &tx_req,
            rx_res: &rx_res,

            stdout: stdout,
        }
    };

    Requester::new(rx_req, tx_res, working.clone());

    let respsoner = Responser::new();

    let mut index_control = hkg::control::index::Index::new();
    let mut show_control = hkg::control::show::Show::new();

    // topics request
    let status_message = list_page(&mut app.state_manager, &tx_req);
    app.status_bar.append(&app.screen_manager, &status_message);


    let (tx_in, rx_in) = channel::<::termion::event::Key>();

    let working1 = working.clone();
    let working2 = working.clone();

    thread::spawn(move || {
        while (*working1).load(Ordering::Relaxed) {

            let stdin = stdin();

            for c in stdin.keys() {
                // println!("{:?}", c);
                tx_in.send(c.ok().unwrap()).unwrap();

            }
        }
    });

    while (*working2).load(Ordering::Relaxed) {

        respsoner.try_recv(&mut app);

        match rx_in.try_recv() {
            Ok(c) => {
                info!("receive input!!! {:?}", c);
                if !app.state_manager.is_web_request() {

                    match app.state_manager.get_state() {
                        Status::Startup => {}
                        Status::List => {
                            match index_control.handle(c, &mut app) {
                                Some(i) => {
                                    if i == 0 {
                                        match control.upgrade() {
                                            Some(working) => (*working).store(false, Ordering::Relaxed),
                                            None => {}
                                        }
                                    } else {
                                        print_screen(&mut app);
                                    }
                                }
                                None => {}
                            }
                        }
                        Status::Show => {
                            match show_control.handle(c, &mut app) {
                                Some(i) => {
                                    if i == 0 {
                                        match control.upgrade() {
                                            Some(working) => (*working).store(false, Ordering::Relaxed),
                                            None => {}
                                        }
                                    } else {
                                        print_screen(&mut app);
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                }
            }
            Err(e) => {
                match rx_state.try_recv() {
                    Ok((prev_state, current_state)) => {
                        info!("receive state change");
                        print_screen(&mut app);
                    }
                    Err(_) => { }
                };
            }
        };

        if app.screen_manager.is_width_changed() {
            hkg::screen::common::clear_screen();
            print_screen(&mut app);
        }

        thread::sleep(std::time::Duration::from_millis(50));
    }
}

fn list_page(state_manager: &mut StateManager, tx_req: &Sender<ChannelItem>) -> String {

    let ci = ChannelItem {
        extra: ChannelItemType::Index(ChannelIndexItem {}),
        result: String::from(""),
    };

    let status_message = match tx_req.send(ci) {
        Ok(()) => {
            state_manager.set_web_request(true);    // *is_web_requesting = true;
            "SOK".to_string()
        }
        Err(e) => format!("{}:{}", "SFAIL", e).to_string(),
    };

    status_message
}

fn print_screen(app: &mut hkg::App) {
    match app.state_manager.get_state() {
        Status::Startup => {}
        Status::List => {
            app.index.print(&mut app.stdout, &app.list_topic_items);
        }
        Status::Show => {
            app.show.print(&mut app.stdout, &app.show_item);
        }
    }

    app.status_bar.print(&app.screen_manager);

    app.stdout.flush().expect("fail to flush the stdout");
}

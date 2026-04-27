use std::io::{stdout, stdin, Write};
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use log::{info, error};
use hkg::status::*;
use hkg::model::IconItem;
use hkg::state_manager::*;
use hkg::resources::*;
use hkg::web::*;
use hkg::responser::*;
use hkg::requests;
use hkg::rendering;
use hkg::screen::common;
use hkg::HkgError;
use std::thread;

fn main() -> Result<(), HkgError> {

    // Initialize logging first
    if let Err(e) = log4rs::init_file("config/log4rs.yaml", Default::default()) {
        eprintln!("Warning: Failed to initialize logging: {}", e);
    }

    info!("app start");

    // Clear the screen.
    common::clear_screen();

    let _stdout = stdout();

    // web background services
    let (tx_req, rx_req) = channel::<ChannelItem>();
    let (tx_res, rx_res) = channel::<ChannelItem>();

    let (tx_state, _rx_state) = channel::<(Status,Status)>();

    let working = Arc::new(AtomicBool::new(true));
    let control = Arc::downgrade(&working);

    let mut app = {
        let stdout = {
            let locked = _stdout.lock();
            locked.into_raw_mode()
                .map_err(|e| HkgError::Terminal(format!("Failed to acquire stdout lock: {}", e)))?
        };

        let icon_manifest_string = hkg::utility::readfile(String::from("data/icon.manifest.json"));
        let icon_collection: Box<Vec<IconItem>> = serde_json::from_str(&icon_manifest_string)
            .map_err(|e| HkgError::Config(format!("Failed to parse icon manifest JSON: {}", e)))?;

        hkg::App::builder()
            .channels(&tx_req, &rx_res)
            .icon_collection(icon_collection)
            .state_channel(tx_state)
            .build(Box::new(stdout))?
    };

    Requester::new(rx_req, tx_res, working.clone());

    let respsoner = Responser::new();

    let mut index_control = hkg::control::index::Index::new();
    let mut show_control = hkg::control::show::Show::new();
    let mut dialog_control = hkg::control::dialog::Dialog::new();
    let mut channel_dialog_control = hkg::control::channel_dialog::ChannelDialog::new();

    // topics request
    let status_message = requests::list_page(&mut app.state_manager, &tx_req, app.index_page, &app.current_channel);
    app.status_bar.append(&app.screen_manager, &status_message);


    let (tx_in, rx_in) = channel::<::termion::event::Key>();

    let working1 = working.clone();
    let working2 = working.clone();

    thread::spawn(move || {
        while (*working1).load(Ordering::Relaxed) {

            let stdin = stdin();

            for c in stdin.keys() {
                match c {
                    Ok(key) => {
                        if tx_in.send(key).is_err() {
                            info!("Main thread disconnected, exiting input thread");
                            break;
                        }
                    }
                    Err(e) => {
                        error!("Failed to read key: {}", e);
                        break;
                    }
                }
            }
        }
        info!("Input thread exiting");
    });

    while (*working2).load(Ordering::Relaxed) {

        respsoner.try_recv(&mut app);

        match rx_in.try_recv() {
            Ok(c) => {
                info!("receive input: {:?}", c);
                info!("current state: {:?}", app.state_manager.get_state() );

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
                                    rendering::print_screen(&mut app);
                                }
                            }
                            None => error!("index_control handle receive none.")
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
                                    rendering::print_screen(&mut app);
                                }
                            }
                            None => error!("show_control handle receive none.")
                        }
                    }
                    Status::Dialog => {
                        match dialog_control.handle(c, &mut app) {
                            Some(i) => {
                                if i == 0 {
                                    match control.upgrade() {
                                        Some(working) => (*working).store(false, Ordering::Relaxed),
                                        None => {}
                                    }
                                } else {
                                    rendering::print_screen(&mut app);
                                }
                            }
                            None => error!("dialog_control handle receive none.")
                        }
                    }
                    Status::ChannelDialog => {
                        match channel_dialog_control.handle(c, &mut app) {
                            Some(i) => {
                                if i == 0 {
                                    match control.upgrade() {
                                        Some(working) => (*working).store(false, Ordering::Relaxed),
                                        None => {}
                                    }
                                } else {
                                    rendering::print_screen(&mut app);
                                }
                            }
                            None => error!("channel_dialog_control handle receive none.")
                        }
                    }
                }
            }
            Err(_e) => {}
        };

        if app.state_manager.is_to_print_screen() {
            rendering::print_screen(&mut app);
            app.state_manager.set_to_print_screen(false);
        }

        if app.screen_manager.is_width_changed() || app.screen_manager.is_height_changed() {
            common::clear_screen();
            rendering::print_screen(&mut app);
        }

        thread::sleep(std::time::Duration::from_millis(50));
    }

    info!("app shutdown");
    Ok(())
}

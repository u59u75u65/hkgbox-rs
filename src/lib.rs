extern crate termion;
extern crate rustc_serialize;
extern crate kuchiki;
extern crate chrono;
extern crate hyper;
extern crate hyper_native_tls;
extern crate cancellation;
extern crate time;
extern crate url;
extern crate regex;

#[macro_use]
extern crate log;
extern crate log4rs;

pub mod caches;
pub mod resources;
pub mod status;
pub mod state_manager;
pub mod screen_manager;
pub mod utility;
pub mod reply_model;
pub mod model;
pub mod web;
pub mod responser;
pub mod builders;
pub mod screen;
pub mod control;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

pub struct App <'a>{
    pub builder: builders::Builder,
    pub show_builder: builders::show::Show,
    pub state_manager: state_manager::StateManager,
    pub screen_manager: screen_manager::ScreenManager,
    // pub icon_collection: &'a Box<Vec<model::IconItem>>,
    pub list_topic_items: Vec<model::ListTopicItem>,
    pub show_item: model::ShowItem,

    pub status_bar: screen::status_bar::StatusBar,
    pub index: screen::index::Index,
    pub show: screen::show::Show,

    pub image_request_count_lock: Arc<Mutex<usize>>,
    pub tx_req: &'a Sender<resources::ChannelItem>,
    pub rx_res: &'a Receiver<resources::ChannelItem>,

    pub stdout: Box<termion::raw::RawTerminal<std::io::StdoutLock<'a>>>
}

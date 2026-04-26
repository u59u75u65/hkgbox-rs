pub mod api_client;
pub mod api_models;
pub mod api_utils;
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
pub mod error;

pub use error::{HkgError, Result};

use std::sync::mpsc::{Receiver, Sender};

pub struct App <'a>{
    pub index_builder: builders::index::Index,
    pub show_builder: builders::show::Show,
    pub state_manager: state_manager::StateManager,
    pub screen_manager: screen_manager::ScreenManager,
    pub list_topic_items: Vec<model::ListTopicItem>,
    pub show_item: model::ShowItem,

    pub index_page: usize,
    pub index_max_page: usize,

    pub status_bar: screen::status_bar::StatusBar,
    pub index: screen::index::Index,
    pub show: screen::show::Show,

    pub tx_req: &'a Sender<resources::ChannelItem>,
    pub rx_res: &'a Receiver<resources::ChannelItem>,

    pub stdout: Box<termion::raw::RawTerminal<std::io::StdoutLock<'a>>>
}

impl<'a> App<'a> {
    /// Create a new App builder for structured initialization
    pub fn builder() -> AppBuilder<'a> {
        AppBuilder::default()
    }
}

/// Builder pattern for App initialization
pub struct AppBuilder<'a> {
    tx_req: Option<&'a Sender<resources::ChannelItem>>,
    rx_res: Option<&'a Receiver<resources::ChannelItem>>,
    icon_collection: Option<Box<Vec<model::IconItem>>>,
    tx_state: Option<Sender<(status::Status, status::Status)>>,
}

impl<'a> Default for AppBuilder<'a> {
    fn default() -> Self {
        Self {
            tx_req: None,
            rx_res: None,
            icon_collection: None,
            tx_state: None,
        }
    }
}

impl<'a> AppBuilder<'a> {
    pub fn channels(mut self, tx_req: &'a Sender<resources::ChannelItem>, rx_res: &'a Receiver<resources::ChannelItem>) -> Self {
        self.tx_req = Some(tx_req);
        self.rx_res = Some(rx_res);
        self
    }

    pub fn icon_collection(mut self, icons: Box<Vec<model::IconItem>>) -> Self {
        self.icon_collection = Some(icons);
        self
    }

    pub fn state_channel(mut self, tx_state: Sender<(status::Status, status::Status)>) -> Self {
        self.tx_state = Some(tx_state);
        self
    }

    pub fn build(self, stdout: Box<termion::raw::RawTerminal<std::io::StdoutLock<'a>>>) -> Result<App<'a>> {
        let icon_collection = self.icon_collection
            .ok_or_else(|| HkgError::Config("Icon collection not provided".into()))?;

        let tx_state = self.tx_state
            .ok_or_else(|| HkgError::Config("State channel not provided".into()))?;

        let tx_req = self.tx_req
            .ok_or_else(|| HkgError::Config("Request channel not provided".into()))?;

        let rx_res = self.rx_res
            .ok_or_else(|| HkgError::Config("Response channel not provided".into()))?;

        Ok(App {
            index_builder: builders::index::Index::new(),
            show_builder: builders::show::Show::new(),
            state_manager: state_manager::StateManager::new(tx_state),
            screen_manager: screen_manager::ScreenManager::new(),
            list_topic_items: Default::default(),
            show_item: Default::default(),
            index_page: 1,
            index_max_page: 1,
            status_bar: screen::status_bar::StatusBar::new(),
            index: screen::index::Index::new(),
            show: screen::show::Show::new(icon_collection),
            tx_req,
            rx_res,
            stdout,
        })
    }
}

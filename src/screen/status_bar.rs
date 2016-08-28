extern crate termion;

use screen_manager::*;
use termion::{color, style};

pub struct StatusBar {
    store: String,
}

impl StatusBar {
    pub fn new () -> Self {
        StatusBar {
            store: ">".to_string()
        }
    }

    pub fn append(&mut self, sm: &ScreenManager, value: &str) {
        if self.store.len() >= sm.getWidth() {
            self.store = format!("{}{}", &"> ", value).to_string()
        } else {
            self.store = format!("{}{}", &self.store, value).to_string()
        }
    }
    pub fn print(&mut self, sm: &ScreenManager) -> String {
        let h = sm.getHeight() as u16;
        format!( "{}{}{}{}{}{}",
                termion::cursor::Goto(1, h),
                color::Fg(color::White),
                style::Bold,
                format!("{status}", status = self.store),
                style::Reset,
                termion::cursor::Hide).to_string()
    }
}

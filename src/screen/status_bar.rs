
use screen_manager::*;

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
        if self.store.len() >= sm.get_width() {
            self.store = format!("{}{}", &"> ", value).to_string()
        } else {
            self.store = format!("{}{}", &self.store, value).to_string()
        }
    }
    pub fn print(&mut self, sm: &ScreenManager) {
        let h = sm.get_height() as u16;
        print!("{}{}{}{}{}{}",
                ::termion::cursor::Goto(1, h),
                ::termion::color::Fg(::termion::color::White),
                ::termion::style::Bold,
                format!("{status}", status = self.store),
                ::termion::style::Reset,
                ::termion::cursor::Hide);
    }
}

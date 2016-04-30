extern crate rustbox;
use rustbox::*;
use screen::common::*;
use utility::string::*;
use model::TopicItem;

pub struct Show<'a> {
    rustbox: &'a rustbox::RustBox,
}

impl<'a> Show<'a> {
    pub fn new(rustbox: &'a rustbox::RustBox) -> Self {
        Show { rustbox: &rustbox }
    }
    pub fn print(&mut self, title: &str) {
        print_header(&self.rustbox, self.rustbox.width(), &title);
    }
}

fn print_header(rustbox: &rustbox::RustBox, width: usize, text: &str) {
    let title_len = jks_len(text);
    let padding = (if width >= title_len {
        width - title_len
    } else {
        0
    }) / 2;

    let header_bottom = (0..width).map(|_| "─").collect::<Vec<_>>().join("");

    clearline(&rustbox, width, 0, 0);
    rustbox.print(padding,
                  0,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  text);
    rustbox.print(0,
                  1,
                  rustbox::RB_BOLD,
                  Color::Yellow,
                  Color::Black,
                  &header_bottom);
}

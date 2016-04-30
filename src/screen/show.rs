extern crate rustbox;
use rustbox::*;
use screen::common::*;
use utility::string::*;
use model::ShowItem;

pub struct Show<'a> {
    rustbox: &'a rustbox::RustBox,
}

impl<'a> Show<'a> {
    pub fn new(rustbox: &'a rustbox::RustBox) -> Self {
        Show { rustbox: &rustbox }
    }
    pub fn print(&mut self, title: &str, item: &ShowItem) {

        print_header(&self.rustbox,
                     self.rustbox.width(),
                     &format!("{} - {}", item.title, title));
        print_body(&self.rustbox,
                   self.body_width(),
                   2,
                   self.body_height(),
                   &item);
    }
    pub fn body_height(&self) -> usize {
        if self.rustbox.height() >= 3 {
            self.rustbox.height() - 3
        } else {
            0
        }
    }

    pub fn body_width(&self) -> usize {
        if self.rustbox.width() >= 2 {
            self.rustbox.width() - 2
        } else {
            0
        }
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

fn print_body(rustbox: &rustbox::RustBox,
              width: usize,
              offset_y: usize,
              rows: usize,
              item: &ShowItem) {

    for (i, reply) in item.replies.iter().take(1).enumerate() {
        rustbox.print(0,
                      i + offset_y,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Black,
                      &format!("{}", reply.content));
    }
}

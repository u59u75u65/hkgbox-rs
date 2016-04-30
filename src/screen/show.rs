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

    let mut y = offset_y;
    let replier_max_width = 14;

    let separator_width = if rustbox.width() >= 4 {
        rustbox.width() - 4
    } else {
        0
    };
    let separator_padding_width = if rustbox.width() > separator_width {
        rustbox.width() - separator_width
    } else {
        0
    } / 2;

    let separator_padding = (0..separator_padding_width).map(|_| " ").collect::<Vec<_>>().join("");


    let separator_bottom = make_separator_bottom(separator_width, &separator_padding);

    for (i, reply) in item.replies.iter().take(rows).enumerate() {
        let contents: Vec<&str> = reply.content.split("\n").collect();

        let mut m = 0;

        for (j, content) in contents.iter().enumerate() {
            rustbox.print(0,
                          j + y,
                          rustbox::RB_NORMAL,
                          Color::White,
                          Color::Black,
                          &format!(" {}", content));
            m += 1;
        }
        let replier_name = reply.username.clone();
        let separator_top = make_separator_top(separator_width,
                                               &separator_padding,
                                               replier_max_width,
                                               &replier_name);

        rustbox.print(0,
                      m + y,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Black,
                      &separator_top);
        m += 1;

        rustbox.print(0,
                      m + y,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Black,
                      &separator_bottom);
        m += 1;
        y += m;
    }
}


fn make_separator_top(separator_width: usize,
                      separator_padding: &str,
                      replier_max_width: usize,
                      replier_name: &str)
                      -> String {
    let replier_name_len = jks_len(&replier_name);
    let replier_name_spacing_width = replier_max_width - replier_name_len;
    let is_replier_name_spacing_width_odd = replier_name_spacing_width & 1 == 1;
    let replier_name_right_spacing_width = replier_name_spacing_width / 2;
    let replier_name_left_spacing_width = if is_replier_name_spacing_width_odd {
        replier_name_right_spacing_width + 1
    } else {
        replier_name_right_spacing_width
    };

    let replier_name_left_spacing = (0..replier_name_left_spacing_width)
                                        .map(|_| "─")
                                        .collect::<Vec<_>>()
                                        .join("");

    let replier_name_right_spacing = (0..replier_name_right_spacing_width)
                                         .map(|_| "─")
                                         .collect::<Vec<_>>()
                                         .join("");

    let separator_replier = format!("{}{}{}{}{}",
                                    "╭",
                                    replier_name_left_spacing,
                                    replier_name,
                                    replier_name_right_spacing,
                                    "╮");
    let separator_replier_width = jks_len(&separator_replier);

    let separator_top_middle_width = if separator_width > separator_replier_width {
        separator_width - separator_replier_width
    } else {
        0
    };
    let separator_top_middle = (0..separator_top_middle_width)
                                   .map(|_| " ")
                                   .collect::<Vec<_>>()
                                   .join("");

    let separator_top = format!("{}{}{}{}",
                                separator_padding,
                                separator_top_middle,
                                separator_replier,
                                separator_padding);
    return separator_top;
}
fn make_separator_bottom(separator_width: usize, separator_padding: &str) -> String {
    let style_box_width = 1;
    let separator_bottom_middle_width = if separator_width > style_box_width {
        separator_width - style_box_width
    } else {
        0
    };
    let separator_bottom_middle = (0..separator_bottom_middle_width)
                                      .map(|_| "─")
                                      .collect::<Vec<_>>()
                                      .join("");
    let separator_bottom = format!("{}{}{}{}",
                                   separator_padding,
                                   separator_bottom_middle,
                                   "╯",
                                   separator_padding);
    return separator_bottom;
}

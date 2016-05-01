extern crate rustbox;
extern crate chrono;

use rustbox::*;
use chrono::*;

use screen::common::*;
use utility::string::*;
use model::ShowItem;


pub struct Show<'a> {
    rustbox: &'a rustbox::RustBox,
    scrollY: usize,
}

impl<'a> Show<'a> {
    pub fn new(rustbox: &'a rustbox::RustBox) -> Self {
        Show {
            rustbox: &rustbox,
            scrollY: 0,
        }
    }
    pub fn print(&mut self, title: &str, item: &ShowItem) {

        print_header(&self.rustbox,
                     self.rustbox.width(),
                     &format!("{} - {} [{}/{}]", item.title, title, item.page, item.max_page));
        print_body(&self.rustbox,
                   self.body_width(),
                   2,
                   self.body_height(),
                   &item,
                   self.scrollY);
    }

    pub fn resetY(&mut self) {
        self.scrollY = 0;
    }

    pub fn scrollUp(&mut self, value: usize) -> bool {
        let tmp = self.scrollY;
        if tmp > value {
            self.scrollY = tmp - value;
            true
        } else if tmp != 0 {
            self.scrollY = 0;
            true
        } else {
            false
        }
    }

    pub fn scrollDown(&mut self, value: usize) -> bool {
        let tmp = self.scrollY;
        if tmp < 10000 {
            self.scrollY = tmp + value;
            return true;
        }
        false
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
              item: &ShowItem,
              scrollY: usize) {

    let mut y = offset_y;
    let replier_max_width = 14;
    let time_max_width = 5;
    let now = Local::now();

    let separator_width = if rustbox.width() >= 2 {
        rustbox.width() - 2
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
            if scrollY + 1 < y + m {
                rustbox.print(0,
                              j + y - scrollY,
                              rustbox::RB_NORMAL,
                              Color::White,
                              Color::Black,
                              &format!(" {}", content));
            }
            m += 1;
        }

        if scrollY + 1 < y + m {

            let replier_name = reply.username.clone();

            let published_at = reply.published_at.clone();

            let published_at_dt = match Local.datetime_from_str(&published_at, "%d/%m/%Y %H:%M") {
                Ok(v) => v,
                Err(e) => now,
            };
            let time = published_at_format(&(now - published_at_dt));

            let separator_top = make_separator_top(separator_width,
                                                   &separator_padding,
                                                   replier_max_width,
                                                   &replier_name,
                                                   time_max_width,
                                                   &time);

            rustbox.print(0,
                          m + y - scrollY,
                          rustbox::RB_NORMAL,
                          Color::Green,
                          Color::Black,
                          &separator_top);
        }
        m += 1;

        if scrollY + 1 < y + m {
            rustbox.print(0,
                          m + y - scrollY,
                          rustbox::RB_NORMAL,
                          Color::Green,
                          Color::Black,
                          &separator_bottom);
        }
        m += 1;
        y += m;
    }
}

fn make_separator_replier_name(separator_width: usize,
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
                                    "");

    return separator_replier;
}

fn make_separator_time(separator_width: usize,
                       separator_padding: &str,
                       time_max_width: usize,
                       time: &str)
                       -> String {
    let time_len = jks_len(&time);
    let time_spacing_width = if time_max_width > time_len {
        time_max_width - time_len
    } else {
        0
    };

    let is_time_spacing_width_odd = time_spacing_width & 1 == 1;
    let time_right_spacing_width = time_spacing_width / 2;
    let time_left_spacing_width = if is_time_spacing_width_odd {
        time_right_spacing_width + 1
    } else {
        time_right_spacing_width
    };

    let time_left_spacing = (0..time_left_spacing_width)
                                .map(|_| "─")
                                .collect::<Vec<_>>()
                                .join("");

    let time_right_spacing = (0..time_right_spacing_width)
                                 .map(|_| "─")
                                 .collect::<Vec<_>>()
                                 .join("");

    let separator_time = format!("{}{}{}{}{}",
                                 "",
                                 time_left_spacing,
                                 time,
                                 time_right_spacing,
                                 "╮");


    return separator_time;
}

fn make_separator_top(separator_width: usize,
                      separator_padding: &str,
                      replier_max_width: usize,
                      replier_name: &str,
                      time_max_width: usize,
                      time: &str)
                      -> String {

    let separator_replier = make_separator_replier_name(separator_width,
                                                        &separator_padding,
                                                        replier_max_width,
                                                        &replier_name);

    let separator_replier_width = jks_len(&separator_replier);

    let separator_time = make_separator_time(separator_width,
                                             &separator_padding,
                                             time_max_width,
                                             &time);

    let separator_time_width = jks_len(&separator_time);

    let separator_top_middle_width = if separator_width >=
                                        (separator_replier_width + separator_time_width) {
        separator_width - separator_replier_width - separator_time_width
    } else {
        0
    };

    let separator_top_middle = (0..separator_top_middle_width)
                                   .map(|_| " ")
                                   .collect::<Vec<_>>()
                                   .join("");

    let separator_top = format!("{}{}{}{}{}",
                                separator_padding,
                                separator_top_middle,
                                separator_replier,
                                separator_time,
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


fn published_at_format(duration: &Duration) -> String {
    let weeks = duration.num_weeks();
    let days = duration.num_days();
    let hours = duration.num_hours();
    let minutes = duration.num_minutes();

    if weeks > 0 {
        format!("{}w", weeks)
    } else if days > 0 {
        format!("{}d", days)
    } else if hours > 0 {
        format!("{}h", hours)
    } else if minutes > 0 {
        format!("{}m", minutes)
    } else {
        String::from("1m")
    }
}

extern crate termion;
extern crate chrono;

use termion::{TermRead, TermWrite, IntoRawMode, Color, Style, Key};
use termion::terminal_size;
use std::io::{Read, Write, Stdout, Stdin};
use std::io::{stdout, stdin};
use std;

use chrono::*;
use utility::string::*;
use model::ShowReplyItem;
use model::ShowItem;
use reply_model::*;

pub struct Post {
    scrollY: usize,
    y: usize,
    replier_max_width: usize,
    time_max_width: usize,
    is_scroll_to_end: bool
}

impl Post {
    pub fn new() -> Self {
        Post {
            scrollY: 0,
            y: 0,
            replier_max_width: 14,
            time_max_width: 5,
            is_scroll_to_end: false
        }
    }
    pub fn print(&mut self, stdout: &mut termion::RawTerminal<std::io::StdoutLock>, title: &str, item: &ShowItem) {

        self.y = 2;

        self.print_header(stdout, &format!("{} - {} [{}/{}]",
                                   item.title,
                                   title,
                                   item.page,
                                   item.max_page));
        self.print_body(stdout, &item);
    }

    fn print_separator_top(&mut self, stdout: &mut termion::RawTerminal<std::io::StdoutLock>, reply: &ShowReplyItem) {
        if self.can_print() {
            let (replier_name, time) = make_separator_content(&reply);
            let s = self.build_separator_top(&replier_name, &time);
            self.print_separator_line(stdout, &s);
        }
    }

    fn print_separator_bottom(&mut self, stdout: &mut termion::RawTerminal<std::io::StdoutLock>) {
        if self.can_print() {
            let s = self.build_separator_bottom();
            self.print_separator_line(stdout, &s);
        }
    }

    fn print_separator_line(&mut self, stdout: &mut termion::RawTerminal<std::io::StdoutLock>, s: &str) {
        // self.rustbox.print(0,
        //                    self.scrolledY(),
        //                    rustbox::RB_NORMAL,
        //                    Color::Green,
        //                    Color::Black,
        //                    &s);
        stdout.style(Style::Reset).unwrap();
        stdout.goto(0, self.scrolledY() as u16).unwrap();
        stdout.color(Color::Green).unwrap();
        stdout.bg_color(Color::Black).unwrap();
        stdout.write(s.as_bytes()).unwrap();

        stdout.hide_cursor().unwrap();
        stdout.reset().unwrap();
    }

    fn print_header(&mut self, stdout: &mut termion::RawTerminal<std::io::StdoutLock>, text: &str) {
        let title_len = jks_len(text);
        let w = terminal_size().unwrap().0 as usize;
        let padding = ((if w >= title_len {
            w - title_len
        } else {
            0
        }) / 2) as u16;

        let header_bottom = seq_str_gen(0, w, "─", "");

        // clearline(&self.rustbox, self.rustbox.width(), 0, 0);
        // self.rustbox.print(padding,
        //                    0,
        //                    rustbox::RB_BOLD,
        //                    Color::White,
        //                    Color::Black,
        //                    text);
        // self.rustbox.print(0,
        //                    1,
        //                    rustbox::RB_BOLD,
        //                    Color::Yellow,
        //                    Color::Black,
        //                    &header_bottom);

        stdout.goto(padding, 0).unwrap();
        stdout.color(Color::White).unwrap();
        stdout.bg_color(Color::Black).unwrap();
        stdout.style(Style::Bold).unwrap();
        stdout.write(text.as_bytes()).unwrap();

        stdout.goto(0, 1).unwrap();
        stdout.color(Color::Yellow).unwrap();
        stdout.bg_color(Color::Black).unwrap();
        stdout.style(Style::Bold).unwrap();
        stdout.write(header_bottom.as_bytes()).unwrap();

        stdout.hide_cursor().unwrap();
        stdout.reset().unwrap();
    }

    pub fn print_body(&mut self, stdout: &mut termion::RawTerminal<std::io::StdoutLock>, item: &ShowItem) {
        let width = self.body_width();
        let rows = self.body_height();

        for (i, reply) in item.replies.iter().take(rows).enumerate() {

            self.print_reply(stdout, &reply.body, 0);

            self.print_separator_top(stdout, &reply);
            self.y += 1;

            self.print_separator_bottom(stdout);
            self.y += 1;
        }

        self.is_scroll_to_end = self.scrolledY() < self.body_height();
    }

    fn print_reply_line(&mut self, stdout: &mut termion::RawTerminal<std::io::StdoutLock>, s: String) {
        // self.rustbox.print(0,
        //                    self.scrolledY(),
        //                    rustbox::RB_NORMAL,
        //                    Color::White,
        //                    Color::Black,
        //                    &s);

       stdout.style(Style::Reset).unwrap();
       stdout.goto(0, self.scrolledY() as u16).unwrap();
       stdout.color(Color::White).unwrap();
       stdout.bg_color(Color::Black).unwrap();
       stdout.write(s.as_bytes()).unwrap();

       stdout.hide_cursor().unwrap();
       stdout.reset().unwrap();
    }

    fn print_reply(&mut self, stdout: &mut termion::RawTerminal<std::io::StdoutLock>, vec: &Vec<NodeType>, depth: usize) {

        let padding = seq_str_gen(0, depth, "├─", "");
        let mut line = String::new();
        let mut is_first = true;

        let vec_clean = clean_reply_body(vec);
        for (j, node) in vec_clean.iter().enumerate() {
            match node.clone() {
                NodeType::Text(n) => {
                    if n.data != "" {
                        line = format!("{}{}", line, n.data);
                    }
                }
                NodeType::Image(n) => {
                    if n.data != "" {
                        line = format!("{}[img {}]", line, n.data);
                    }
                }
                NodeType::BlockQuote(n) => {
                    self.print_reply(stdout, &n.data, depth + 1);
                    is_first = false;
                }
                NodeType::Br(n) => {
                    if !line.is_empty() {
                        if self.can_print() {
                            self.print_reply_line(stdout, format!(" {}{}", padding, line));
                        }
                        line = String::new();
                        is_first = false;
                    }

                    // prevent first line empty
                    if !is_first {
                        self.y += 1;
                    }

                }
            }
        }

        if !line.is_empty() {
            if self.can_print() {
                self.print_reply_line(stdout, format!(" {}{}  ", padding, line));
            }
            line = String::new();
            self.y += 1;
        }

    }

    fn build_separator_arguments(&mut self) -> (usize, usize, String) {
        let separator_width = self.body_width();
        let w = terminal_size().unwrap().0 as usize;

        let separator_padding_width = if w > separator_width {
            w - separator_width
        } else {
            0
        } / 2;

        let separator_padding = seq_str_gen(0, separator_padding_width, " ", "");

        (separator_width, separator_padding_width, separator_padding)
    }

    fn build_separator_top(&mut self, replier_name: &str, time: &str) -> String {
        let (separator_width, separator_padding_width, separator_padding) =
            self.build_separator_arguments();
        make_separator_top(separator_width,
                           &separator_padding,
                           self.replier_max_width,
                           &replier_name,
                           self.time_max_width,
                           &time)
    }

    fn build_separator_bottom(&mut self) -> String {
        let (separator_width, separator_padding_width, separator_padding) =
            self.build_separator_arguments();
        make_separator_bottom(separator_width, &separator_padding)
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

        if !self.is_scroll_to_end {
            let min_to_scroll = self.scrollY + self.body_height();
            if self.y >= min_to_scroll + value {
                self.scrollY += value;
                return true;
            } else if self.y >= min_to_scroll {
                self.scrollY += self.y - min_to_scroll;
                return true;
            }
        }

        false
    }

    pub fn body_height(&self) -> usize {

        let h = terminal_size().unwrap().1;

        if h >= 3 {
            h as usize - 3
        } else {
            0
        }
    }

    pub fn body_width(&self) -> usize {

        let w = terminal_size().unwrap().0;

        if w >= 2 {
            w as usize - 2
        } else {
            0
        }
    }

    fn can_print(&self) -> bool {
        self.y > self.scrollY + 1 && self.y < self.scrollY + 1 + self.body_height()
    }

    fn scrolledY(&self) -> usize {
        self.y - self.scrollY
    }

}



fn make_separator_content(reply: &ShowReplyItem) -> (String, String) {
    let now = Local::now();

    let replier_name = reply.username.clone();

    let published_at = reply.published_at.clone();

    let published_at_dt = match Local.datetime_from_str(&published_at, "%d/%m/%Y %H:%M") {
        Ok(v) => v,
        Err(e) => now,
    };
    let time = published_at_format(&(now - published_at_dt));
    (replier_name, time)
}

fn clean_reply_body(vec: &Vec<NodeType>) -> Vec<NodeType> {
    // clean up lines (end)
    let vec2 = {
        let vec_length = vec.len();
        let vec_check_cleanup = vec.clone();

        // check if last 4 elements match the EMPTY PATTERN
        let is_last4_empty = vec_check_cleanup.iter()
                                              .rev()
                                              .take(4)
                                              .enumerate()
                                              .all(|(j, node)| match node.clone() {
                                                  NodeType::Br(n) => j == 1 || j == 2 || j == 3,
                                                  NodeType::Text(n) => j == 0 && n.data.is_empty(),
                                                  _ => false,
                                              });

        let vec_short_length = if vec_length > 4 && is_last4_empty {
            vec_length - 4
        } else {
            vec_length
        };

        vec.iter().take(vec_short_length)
    };

    // clean up lines (start)
    let vec3 = {
        let vec2_cloned = vec2.clone();
        let mut result: Vec<NodeType> = Vec::new();
        for (j, node) in vec2_cloned.enumerate() {
            let node2 = node.clone();
            let node3 = node.clone();
            match node2 {
                NodeType::Br(n) => {
                    if !result.is_empty() {
                        result.push(node3);
                    }
                }
                _ => result.push(node3),
            }
        }
        result.clone()
    };

    vec3
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

    let replier_name_left_spacing = seq_str_gen(0, replier_name_left_spacing_width, "─", "");
    let replier_name_right_spacing = seq_str_gen(0, replier_name_right_spacing_width, "─", "");

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

    let time_left_spacing = seq_str_gen(0, time_left_spacing_width, "─", "");
    let time_right_spacing = seq_str_gen(0, time_right_spacing_width, "─", "");

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

    let separator_top_middle = seq_str_gen(0, separator_top_middle_width, " ", "");
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
    let separator_bottom_middle = seq_str_gen(0, separator_bottom_middle_width, "─", "");

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

fn seq_str_gen(start: usize, end: usize, sym: &str, join_sym: &str) -> String {
    (start..end).map(|_| sym.clone()).collect::<Vec<_>>().join(&join_sym)
}

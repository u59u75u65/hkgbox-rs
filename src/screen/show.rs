use log::{info, error};
use std::io::Write;
use std;

use chrono::*;
use crate::utility::string::*;
use crate::model::IconItem;
use crate::model::ShowReplyItem;
use crate::model::ShowItem;
use crate::reply_model::*;
use crate::screen::common::*;

pub struct Show {
    title: String,
    scroll_y: usize,
    y: usize,
    replier_max_width: usize,
    time_max_width: usize,
    is_scroll_to_end: bool,
    icon_collection: Box<Vec<IconItem>>
}

impl Show {
    pub fn new (icon_collection: Box<Vec<IconItem>>) -> Self {
        Show {
            title: String::from("高登"),
            scroll_y: 0,
            y: 0,
            replier_max_width: 14,
            time_max_width: 5,
            is_scroll_to_end: false,
            icon_collection: icon_collection
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn print(&mut self, stdout: &mut ::termion::raw::RawTerminal<std::io::StdoutLock>, item: &ShowItem) {

        self.y = 2;
        let title = self.title.clone();
        self.print_header(stdout, &format!("{} - {} [{}/{}]",
                                   item.title,
                                   title,
                                   item.page,
                                   item.max_page));
        self.print_body(stdout, &item);
    }

    fn print_separator_top(&mut self, stdout: &mut ::termion::raw::RawTerminal<std::io::StdoutLock>, reply: &ShowReplyItem) {
        if self.can_print() {
            let (replier_name, time) = make_separator_content(&reply);
            let s = self.build_separator_top(&replier_name, &time);
            self.print_separator_line(stdout, &s);
        }
    }

    fn print_separator_bottom(&mut self, stdout: &mut ::termion::raw::RawTerminal<std::io::StdoutLock>) {
        if self.can_print() {
            let s = self.build_separator_bottom();
            self.print_separator_line(stdout, &s);
        }
    }

    fn print_separator_line(&mut self, stdout: &mut ::termion::raw::RawTerminal<std::io::StdoutLock>, s: &str) {

        write!(stdout, "{}{}{}{}{}{}",
                ::termion::cursor::Goto(1, (self.scrolled_y() + 1) as u16),
                ::termion::color::Fg(::termion::color::Green),
                ::termion::style::Bold,
                s,
                ::termion::style::Reset,
                ::termion::cursor::Hide).expect("fail to write to shell");
    }

    fn print_header(&mut self, stdout: &mut ::termion::raw::RawTerminal<std::io::StdoutLock>, text: &str) {
        let title_len = jks_len(text);
        let w = ::termion::terminal_size().expect("fail to get terminal size").0 as usize;
        let padding = ((if w >= title_len {
            w - title_len
        } else {
            0
        }) / 2) as u16;

        let header_bottom = seq_str_gen(0, w, "─", "");

        write!(stdout, "{}{}{}{}{}{}",
                ::termion::cursor::Goto(padding + 1, 1),
                ::termion::color::Fg(::termion::color::White),
                ::termion::style::Bold,
                text, ::termion::style::Reset,
                ::termion::cursor::Hide).expect("fail to write to shell");

        write!(stdout, "{}{}{}{}{}{}",
                ::termion::cursor::Goto(1, 2),
                ::termion::color::Fg(::termion::color::Yellow),
                ::termion::style::Bold,
                header_bottom,
                ::termion::style::Reset,
                ::termion::cursor::Hide).expect("fail to write to shell");
    }

    pub fn print_body(&mut self, stdout: &mut ::termion::raw::RawTerminal<std::io::StdoutLock>, item: &ShowItem) {
        let _width = self.body_width();
        let rows = self.body_height();
        let max_y = self.scroll_y + 1 + rows;

        // Print main content first if exists
        if let Some(main_content) = &item.main_content {
            // Check if we have space before printing
            if self.y + 2 < max_y {
                self.print_reply(stdout, &main_content.body, 0);

                self.print_separator_top(stdout, &main_content);
                self.y += 1;

                self.print_separator_bottom(stdout);
                self.y += 1;
            }
        }

        // Then print replies
        for (_i, reply) in item.replies.iter().enumerate() {
            // Stop if we've exceeded the available space
            if self.y + 2 >= max_y {
                break;
            }

            self.print_reply(stdout, &reply.body, 0);

            self.print_separator_top(stdout, &reply);
            self.y += 1;

            self.print_separator_bottom(stdout);
            self.y += 1;
        }

        info!("[print_body] y: {} scroll_y: {} body_height: {}", self.y, self.scroll_y,  self.body_height());

        self.is_scroll_to_end = self.scrolled_y() < self.body_height();
    }

    fn print_reply_line(&mut self, stdout: &mut ::termion::raw::RawTerminal<std::io::StdoutLock>, s: String) {

       write!(stdout, "{}{}{}{}{}",
               ::termion::cursor::Goto(1, (self.scrolled_y() + 1) as u16),
               ::termion::color::Fg(::termion::color::White),
               s,
               ::termion::style::Reset,
               ::termion::cursor::Hide).expect("fail to write to shell");
    }

    fn print_reply(&mut self, stdout: &mut ::termion::raw::RawTerminal<std::io::StdoutLock>, vec: &Vec<NodeType>, depth: usize) {

        let icon_width = 2;
        let img_height = 10;
        let padding = seq_str_gen(0, depth, "├─", "");
        let mut line = String::new();
        let mut is_first = true;

        let vec_clean = clean_reply_body(vec);
        let mut img_offset = 0;
        let mut text_y_offset = 0;
        let w = ::termion::terminal_size().expect("fail to get terminal size").0 as usize;

        for (_j, node) in vec_clean.iter().enumerate() {
            match node.clone() {
                NodeType::Text(n) => {
                    if n.data != "" {
                        let text = &n.data;
                        let len = jks_len(&text);
                        text_y_offset = (if w > 0 { len / w } else { 0 }) + 1;
                        line = format!("{}{}", line, text);
                    }
                }
                NodeType::Link(n) => {
                    if !n.text.is_empty() {
                        let text = &n.text;
                        let len = jks_len(&text);
                        text_y_offset = (if w > 0 { len / w } else { 0 }) + 1;
                        line = format!("{}{}", line, text);
                    }
                }
                NodeType::Image(n) => {
                    if n.data != "" {
                        info!("Processing image: url={}, alt={}, is_external={}",
                              n.data, n.alt, n.data.starts_with("http") || n.data.starts_with("https"));

                        // Check if this is an external image (HTTP/HTTPS URL) or local emotion icon
                        let is_external_image = n.data.starts_with("http") || n.data.starts_with("https");

                        if is_external_image {
                            if self.can_still_print(img_offset + text_y_offset + img_height) {
                                if self.can_print() {
                                    match imgcat_from_url(&n.data, img_height) {
                                        Ok(img) => {
                                            img_offset += img_height + 2;
                                            line = format!("{}\n\r {}{}", line, padding, img);
                                        }
                                        Err(e) => {
                                            error!("Failed to load image: URL={}, Error={}", n.data, e);
                                            img_offset += img_height;
                                            line = format!("{}\n\r {}[x]", line, padding);
                                        }
                                    }
                                } else {
                                    img_offset += 1;
                                    line = format!("{}\n", line);
                                }
                            } else {
                                img_offset += 2;
                                line = format!("{}\n\r {}[-]", padding, line);
                            }
                        } else {
                            // Local emotion icon
                            match self.get_icon_reference(&n.alt, &n.data) {
                                Some(icon_reference) => {
                                    if line.is_empty() {
                                        img_offset += 1;
                                    }
                                    line = format!("{}{}", line, imgcat_from_path(&icon_reference, icon_width));
                                },
                                None => {
                                    error!("Local icon not found: alt={}, url={}", n.alt, n.data);
                                    line = format!("{}[:(]", line);
                                }
                            }
                        }
                    }
                }
                NodeType::BlockQuote(n) => {
                    if self.can_still_print(img_offset + text_y_offset) {
                        self.print_reply(stdout, &n.data, depth + 1);
                        is_first = false;                
                    }
                }
                NodeType::Br(_n) => {
                    if !line.is_empty() {
                        if self.can_print() {
                            self.print_reply_line(stdout, format!(" {}{}", padding, line));
                        }
                        line = String::new();
                        is_first = false;

                        if text_y_offset > 0 {
                            self.y += text_y_offset;
                            text_y_offset = 0;
                        }

                        if img_offset > 0 {
                            self.y += img_offset;
                            img_offset = 0;
                        }
                    } else {
                        // prevent first line empty + prevent empty lines in quotes
                        if !is_first && depth == 0 {
                            self.y += 1;
                        }
                    }
                }
            }
        }

        if !line.is_empty() {

            if self.can_print() {
                self.print_reply_line(stdout, format!(" {}{}", padding, line));

                if text_y_offset > 0 {
                    self.y += text_y_offset;
                }

                if img_offset > 0 {
                    self.y += img_offset;
                }
            }

        }
    }

    fn get_icon_reference(&mut self, alt: &str, url: &str) -> Option<String> {
        // First try to match by alt text (for HKGolden emojis)
        if !alt.is_empty() {
            if let Some(item) = self.icon_collection.iter().find(|icon_item| icon_item.alt.contains(&alt)) {
                return Some(format!("data/icon/{}", &item.src));
            }
        }

        // For LIHKG emojis, match by URL path
        // Extract emoji name from URL like "/assets/faces/lm2/tear.gif"
        if url.starts_with("/assets/faces/") || url.starts_with("/assets/stickers/") {
            if let Some(filename) = url.split('/').last() {
                let emoji_name = filename.trim_end_matches(".gif").trim_end_matches(".png");

                // Extract category from path like "/assets/faces/lm2/tear.gif"
                // Split: ["", "assets", "faces", "lm2", "tear.gif"]
                let parts: Vec<&str> = url.split('/').collect();
                let category = if parts.len() >= 4 { parts[3] } else { "unknown" };

                let possible_names = vec![
                    format!("lihkg_{}_{}.png", category, emoji_name),
                    format!("lihkg_{}_{}", category, emoji_name),
                ];

                for possible_name in possible_names {
                    let icon_path = format!("data/icon/lihkg/{}", possible_name);
                    if std::path::Path::new(&icon_path).exists() {
                        return Some(icon_path);
                    }
                }
            }
        }

        None
    }

    fn build_separator_arguments(&mut self) -> (usize, usize, String) {
        let separator_width = self.body_width();
        let w = ::termion::terminal_size().expect("fail to get terminal size").0 as usize;

        let separator_padding_width = if w > separator_width {
            w - separator_width
        } else {
            0
        } / 2;

        let separator_padding = seq_str_gen(0, separator_padding_width, " ", "");

        (separator_width, separator_padding_width, separator_padding)
    }

    fn build_separator_top(&mut self, replier_name: &str, time: &str) -> String {
        let (separator_width, _separator_padding_width, separator_padding) =
            self.build_separator_arguments();
        make_separator_top(separator_width,
                           &separator_padding,
                           self.replier_max_width,
                           &replier_name,
                           self.time_max_width,
                           &time)
    }

    fn build_separator_bottom(&mut self) -> String {
        let (separator_width, _separator_padding_width, separator_padding) =
            self.build_separator_arguments();
        make_separator_bottom(separator_width, &separator_padding)
    }

    pub fn reset_y(&mut self) {
        self.scroll_y = 0;
    }

    pub fn scroll_up(&mut self, value: usize) -> bool {
        let tmp = self.scroll_y;
        if tmp > value {
            self.scroll_y = tmp - value;
            true
        } else if tmp != 0 {
            self.scroll_y = 0;
            true
        } else {
            false
        }
    }

    pub fn scroll_down(&mut self, value: usize) -> bool {

        if !self.is_scroll_to_end {
            let min_to_scroll = self.scroll_y + self.body_height();
            if self.y >= min_to_scroll + value {
                self.scroll_y += value;
                return true;
            } else if self.y >= min_to_scroll {
                self.scroll_y += self.y - min_to_scroll;
                return true;
            }
        }

        false
    }

    pub fn body_height(&self) -> usize {

        let h = ::termion::terminal_size().expect("fail to get terminal size").1;

        if h >= 3 {
            h as usize - 3
        } else {
            0
        }
    }

    pub fn body_width(&self) -> usize {

        let w = ::termion::terminal_size().expect("fail to get terminal size").0;

        if w >= 2 {
            w as usize - 2
        } else {
            0
        }
    }

    fn can_print(&self) -> bool {
        self.y > self.scroll_y + 1 && self.y < self.scroll_y + 1 + self.body_height()
    }

    fn can_still_print(&self, i: usize) -> bool {
        // info!("[can_still_print] y: {} lower bound: {} upper bound: {}", self.y + i , self.scroll_y, self.scroll_y + self.body_height());
        self.y + i >= self.scroll_y && self.y + i <= self.scroll_y + self.body_height()
    }

    fn scrolled_y(&self) -> usize {
        // info!("[scrolled_y] y: {} scroll_y: {} body_height: {}", self.y, self.scroll_y,  self.body_height());
        if self.y >= self.scroll_y { self.y - self.scroll_y  } else { 0 }
    }

}



fn make_separator_content(reply: &ShowReplyItem) -> (String, String) {
    let now = Local::now();

    let replier_name = reply.username.clone();

    let published_at = reply.published_at.clone();

    // Parse the published_at time (now in local timezone thanks to timestamp_to_strings fix)
    let published_at_dt = match NaiveDateTime::parse_from_str(&published_at, "%d/%m/%Y %H:%M") {
        Ok(v) => v,
        Err(_e) => now.naive_local(),
    };

    // Compare with current local time
    let duration = now.naive_local() - published_at_dt;
    let time = published_at_format(&duration);
    (replier_name, time)
}

fn clean_reply_body(vec: &[NodeType]) -> Vec<NodeType> {
    // clean up lines (end)
    let vec_length = vec.len();

    // check if last 4 elements match the EMPTY PATTERN
    let is_last4_empty = vec.iter()
        .rev()
        .take(4)
        .enumerate()
        .all(|(j, node)| match node {
            NodeType::Br(_n) => j == 1 || j == 2 || j == 3,
            NodeType::Text(n) => j == 0 && n.data.is_empty(),
            _ => false,
        });

    let vec_short_length = if vec_length > 4 && is_last4_empty {
        vec_length - 4
    } else {
        vec_length
    };

    // clean up lines (start)
    let mut result: Vec<NodeType> = Vec::new();
    for node in vec.iter().take(vec_short_length) {
        match node {
            NodeType::Br(_) => {
                if !result.is_empty() {
                    result.push(node.clone());
                }
            }
            _ => result.push(node.clone()),
        }
    }
    result
}

fn make_separator_replier_name(_separator_width: usize,
                               _separator_padding: &str,
                               replier_max_width: usize,
                               replier_name: &str)
                               -> String {
    let replier_name_len = jks_len(&replier_name);
    let replier_name_spacing_width = if replier_max_width > replier_name_len {
        replier_max_width - replier_name_len
    } else {
        0
    };
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

fn make_separator_time(_separator_width: usize,
                       _separator_padding: &str,
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
    let total_seconds = duration.num_seconds();
    let weeks = total_seconds / 604800;
    let days = total_seconds / 86400;
    let hours = total_seconds / 3600;
    let minutes = total_seconds / 60;

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
    (start..end).map(|_| sym).collect::<Vec<_>>().join(&join_sym)
}

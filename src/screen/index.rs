use std::io::Write;
use std;

use utility::string::*;
use model::ListTopicItem;

pub struct Index {
    title: String,
    selected_topic_index: usize,
}

impl Index {
    pub fn new() -> Self {
        Index {
            title: String::from("高登"),
            selected_topic_index: 0,
        }
    }

    pub fn select_topic(&mut self, index: usize) {
        self.selected_topic_index = index;
    }

    pub fn get_selected_topic(&self) -> usize {
        self.selected_topic_index
    }

    pub fn print(&mut self, stdout: &mut ::termion::raw::RawTerminal<std::io::StdoutLock>, collection: &Vec<ListTopicItem>) {

        if self.selected_topic_index > self.body_height() {
            self.selected_topic_index = self.body_height();
        }

        let width = ::termion::terminal_size().expect("fail to get terminal size").0 as usize;

        print_header(stdout, width as usize, &self.title);
        print_body(stdout,
                   self.body_width(),
                   2,
                   self.body_height(),
                   &collection,
                   self.selected_topic_index);
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

}

fn print_header(stdout: &mut ::termion::raw::RawTerminal<std::io::StdoutLock>, width: usize, text: &str) {
    // print header
    let padding = ((width - text.len()) / 2) as u16;
    let header_bottom = (0..width).map(|_| "─").collect::<Vec<_>>().join("");
    let header_top_padding = seq_str_gen(0, width, " ", "");

    write!(stdout, "{}{}{}{}{}",
            ::termion::cursor::Goto(1, 1),
            ::termion::color::Fg(::termion::color::White),
            header_top_padding,
            ::termion::style::Reset,
           	::termion::cursor::Hide).expect("fail to write to shell");

    write!(stdout, "{}{}{}{}{}{}",
            ::termion::cursor::Goto(padding + 1, 1),
            ::termion::color::Fg(::termion::color::White),
            ::termion::style::Bold,
            text,
            ::termion::style::Reset,
            ::termion::cursor::Hide).expect("fail to write to shell");;

    write!(stdout, "{}{}{}{}{}{}",
            ::termion::cursor::Goto(1, 2),
            ::termion::color::Fg(::termion::color::Yellow),
            ::termion::style::Bold,
            header_bottom,
            ::termion::style::Reset,
            ::termion::cursor::Hide).expect("fail to write to shell");;
}

fn print_body(stdout: &mut ::termion::raw::RawTerminal<std::io::StdoutLock>,
    width: usize,
    offset_y: usize,
    rows: usize,
    collection: &Vec<ListTopicItem>,
    selected_topic_index: usize){

    let right_offset = 3;
    let author_max_width = 12;
    let no_max_width = 5;
    let title_max_width = width - no_max_width - author_max_width - right_offset;

    for (i, item) in collection.iter().take(rows).enumerate() {

        let original_title = item.title.text.clone();
        let title: String = substring(&original_title, title_max_width);
        let title_len = jks_len(&title);

        let title_spacin_minus = no_max_width + title_len + author_max_width + right_offset;
        let title_spacing_width = if width > title_spacin_minus {
            width - title_spacin_minus
        } else {
            0
        };
        let title_spacing = (0..title_spacing_width).map(|_| " ").collect::<Vec<_>>().join("");

        let author = item.author.name.clone();
        let author_spacing_width = author_max_width - jks_len(&author) + right_offset;
        let author_spacing = (0..author_spacing_width).map(|_| " ").collect::<Vec<_>>().join("");

        if selected_topic_index == i + 1 {
             write!(stdout, "{}{}{}{}{}{}",
                     ::termion::cursor::Goto(1, (i + offset_y + 1) as u16),
                     ::termion::color::Fg(::termion::color::Black),
                     ::termion::color::Bg(::termion::color::Yellow),
                      format!("[{no:0>2}] {title}{title_spacing}| {author}{author_spacing}",
                              no = i + 1,
                              title = title,
                              title_spacing = title_spacing,
                              author = &author,
                              author_spacing = author_spacing),
                      ::termion::style::Reset,
                      ::termion::cursor::Hide).expect("fail to write to shell");

        } else {
             write!(stdout, "{}{}{}{}{}",
                     ::termion::cursor::Goto(1, (i + offset_y + 1) as u16),
                     ::termion::color::Fg(::termion::color::White),
                     format!("[{no:0>2}] {title}{title_spacing}| {author}{author_spacing}",
                              no = i + 1,
                              title = title,
                              title_spacing = title_spacing,
                              author = &author,
                              author_spacing = author_spacing),
                     ::termion::style::Reset,
                     ::termion::cursor::Hide).expect("fail to write to shell");

        }

    }
}

fn seq_str_gen(start: usize, end: usize, sym: &str, join_sym: &str) -> String {
    (start..end).map(|_| sym.clone()).collect::<Vec<_>>().join(&join_sym)
}

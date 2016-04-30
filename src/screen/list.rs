extern crate rustbox;
use rustbox::*;
use screen::common::*;
use utility::string::*;
use model::TopicItem;

pub struct List<'a> {
    rustbox: &'a rustbox::RustBox,
    selected_topic_index: usize,
}

impl<'a> List<'a> {
    pub fn new(rustbox: &'a rustbox::RustBox) -> Self {
        List {
            rustbox: &rustbox,
            selected_topic_index: 0,
        }
    }

    pub fn select_topic(&mut self, index: usize) {
        self.selected_topic_index = index;
    }
    pub fn get_selected_topic(&self) -> usize {
        self.selected_topic_index
    }
    pub fn print(&self,
                 title: &str,
                 screen_width: usize,
                 body_width: usize,
                 body_height: usize,
                 collection: &Vec<TopicItem>) {
        print_header(&self.rustbox, screen_width, &title);
        print_body(&self.rustbox,
                   body_width,
                   2,
                   body_height,
                   &collection,
                   self.selected_topic_index);

    }
}

fn print_header(rustbox: &rustbox::RustBox, width: usize, text: &str) {
    let padding = (width - text.len()) / 2;
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
              collection: &Vec<TopicItem>,
              selected_topic_index: usize) {

    let right_offset = 3;
    let author_max_width = 12;
    let no_max_width = 5;
    let title_max_width = width - no_max_width - author_max_width - right_offset;

    for (i, item) in collection.iter().take(rows).enumerate() {

        let original_title = item.titles[0].text.clone();
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
            rustbox.print(0,
                          i + offset_y,
                          rustbox::RB_NORMAL,
                          Color::Black,
                          Color::Yellow,
                          &format!("[{no:0>2}] {title}{title_spacing}| {author}{author_spacing}",
                                   no = i + 1,
                                   title = title,
                                   title_spacing = title_spacing,
                                   author = &author,
                                   author_spacing = author_spacing));
        } else {
            rustbox.print(0,
                          i + offset_y,
                          rustbox::RB_NORMAL,
                          Color::White,
                          Color::Black,
                          &format!("[{no:0>2}] {title}{title_spacing}| {author}{author_spacing}",
                                   no = i + 1,
                                   title = title,
                                   title_spacing = title_spacing,
                                   author = &author,
                                   author_spacing = author_spacing));
        }

    }
}

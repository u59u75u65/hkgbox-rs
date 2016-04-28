extern crate hkg;
extern crate rustbox;
extern crate rustc_serialize;

use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

use rustc_serialize::json;

use hkg::utility::cache;
use hkg::model::TopicItem;

fn main() {

    loop {
        // GUI init
        let rustbox = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("{}", e),
        };

        let w = rustbox.width();
        let h = rustbox.height();
        let title = String::from("高登");
        print_header(&rustbox, w, title);

        let s = cache::readfile(String::from("topics.json"));
        let collection: Vec<TopicItem> = json::decode(&s).unwrap();

        print_body(&rustbox, w, 2, h - 3, collection);

        rustbox.print(1,
                      h - 1,
                      rustbox::RB_BOLD,
                      Color::White,
                      Color::Black,
                      "Press 'q' to quit.");


        rustbox.present();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}

fn print_header(rustbox: &rustbox::RustBox, width: usize, text: String) {
    let padding = (width - text.len()) / 2;
    let header_bottom = (0..width).map(|_| "─").collect::<Vec<_>>().join("");
    rustbox.print(padding,
                  0,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  &text);
    rustbox.print(0,
                  1,
                  rustbox::RB_BOLD,
                  Color::Yellow,
                  Color::Black,
                  &header_bottom);
}

fn print_body(rustbox: &rustbox::RustBox,
              width: usize,
              offset_x: usize,
              rows: usize,
              collection: Vec<TopicItem>) {

    let right_offset = 3;
    let author_max_width = 12;
    let no_max_width = 5;
    let title_max_width = width - no_max_width - author_max_width - right_offset;
    let author_position = width - author_max_width - right_offset;

    for (i, item) in collection.iter().take(rows).enumerate() {

        let title: String = item.titles[0].text.chars().take(title_max_width/2).collect();

        rustbox.print(0,
                      i + offset_x,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Black,
                      &format!("[{no:0>2}] {title}",
                               no = i + 1,
                               title = title));

        rustbox.print(author_position,
                      i + offset_x,
                      rustbox::RB_NORMAL,
                      Color::White,
                      Color::Black,
                      &format!("| {author}", author = &item.author.name));
    }

    rustbox.print(0,
                  rows + 2,
                  rustbox::RB_NORMAL,
                  Color::White,
                  Color::Black,
                  &format!("{} {}", title_max_width, author_position));

}

// fn debug_load_and_print_topics() {
//     let s = cache::readfile(String::from("topics.json"));
//     let collection: Vec<TopicItem> = json::decode(&s).unwrap();
//
//     println!("topics {:?}", collection.len());
//     debug_print_topics(collection);
// }
//
// fn debug_print_topics(collection: Vec<TopicItem>) {
//     for (i, item) in collection.iter().enumerate() {
//
//         println!("item[{}]= {title} {author_name} {last_replied_date} {last_replied_time} \
//                   {reply_count} {rating}",
//                  i,
//                  title = item.titles[0].text,
//                  author_name = item.author.name,
//                  last_replied_date = item.last_replied_date,
//                  last_replied_time = item.last_replied_time,
//                  reply_count = item.reply_count,
//                  rating = item.rating);
//     }
// }

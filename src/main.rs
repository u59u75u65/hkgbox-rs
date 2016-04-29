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

    // GUI init
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let title = String::from("高登");
    let s = cache::readfile(String::from("topics.json"));
    let collection: Vec<TopicItem> = json::decode(&s).unwrap();

    let mut status = String::from(">");
    loop {

        let w = rustbox.width();
        let h = rustbox.height();

        print_header(&rustbox, w, &title);
        print_body(&rustbox, w, 2, h - 3, &collection);

        let status_width = if w > status.len() {
            w - status.len()
        } else {
            0
        };
        let status_spacing = (0..status_width).map(|_| " ").collect::<Vec<_>>().join("");

        rustbox.print(1,
                      h - 1,
                      rustbox::RB_BOLD,
                      Color::White,
                      Color::Black,
                      &format!("{status}{status_spacing}",
                               status = status,
                               status_spacing = status_spacing));

        rustbox.present();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => {
                        break;
                    }

                    Key::Up => {
                        if status.len() >= w {
                            status = String::from("U");
                        } else {
                            status = String::from(format!("{}{}", &status, &"U"));
                        }

                    }
                    Key::Down => {
                        if status.len() >= w {
                            status = String::from("D");
                        } else {
                            status = String::from(format!("{}{}", &status, &"D"));
                        }

                    }

                    _ => {}
                }
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }
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
              collection: &Vec<TopicItem>) {

    let right_offset = 3;
    let author_max_width = 12;
    let no_max_width = 5;
    let title_max_width = width - no_max_width - author_max_width - right_offset;

    for (i, item) in collection.iter().take(rows).enumerate() {

        let original_title = item.titles[0].text.clone();
        let title: String = substring(&original_title, title_max_width);
        let title_len = string_jks_len(&title);

        let title_spacin_minus = no_max_width + title_len + author_max_width + right_offset;
        let title_spacing_width = if width > title_spacin_minus {
            width - title_spacin_minus
        } else {
            0
        };
        let title_spacing = (0..title_spacing_width).map(|_| " ").collect::<Vec<_>>().join("");

        let author = item.author.name.clone();
        let author_spacing_width = author_max_width - string_jks_len(&author) + right_offset;
        let author_spacing = (0..author_spacing_width).map(|_| " ").collect::<Vec<_>>().join("");

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
                               author_spacing = author_spacing
                           ));
    }


}

fn clearline(rustbox: &rustbox::RustBox, width: usize, x: usize, y: usize) {
    let s = (0..width).map(|_| "  ").collect::<Vec<_>>().join("");

    rustbox.print(x, y, rustbox::RB_NORMAL, Color::White, Color::Black, &s);
}

fn substring(s: &str, length: usize) -> String {
    let mut v = Vec::new();
    let mut c = 0;

    for x in s.chars() {
        if contains(x) {
            if c < length - 2 {
                v.push(x);
                c = c + 2;
            } else {
                break;
            }

        } else {
            if c < length - 1 {
                v.push(x);
                c = c + 1;
            } else {
                break;
            }
        }
    }

    let s: String = v.iter().cloned().collect();
    return s;
}

fn contains(c: char) -> bool {
    let cjks = vec![(0x4E00..0xA000),
                    (0x3400..0x4DC0),
                    (0x20000..0x2A6E0),
                    (0x2A700..0x2B740),
                    (0x2B740..0x2B820),
                    (0xF900..0xFB00),
                    (0x2F800..0x2FA20),
                    (0x9FA6..0x9FCC),

                    (0x3000..0x303F), // CJK Symbols and Punctuation
                    (0xff00..0xffef) /* Halfwidth and Fullwidth Forms */];

    for cjk in cjks {
        let h = c as u32;
        if cjk.start <= h && h < cjk.end {
            return true;
        }
    }
    return false;
}

fn string_jks_len(s: &str) -> usize {
    return s.chars()
            .map(|x| if contains(x) {
                2
            } else {
                1
            })
            .collect::<Vec<usize>>()
            .iter()
            .fold(0, |acc, &x| acc + x);
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

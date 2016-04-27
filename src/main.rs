extern crate hkg;

extern crate rustbox;

use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

use std::io::prelude::*;
use std::fs::File;
use hkg::utility::cache;

extern crate rustc_serialize;
use rustc_serialize::json;

#[derive(RustcDecodable)]
pub struct TopicTitleItem {
    url: String,
    text: String,
}

#[derive(RustcDecodable)]
pub struct TopicAuthorItem {
    url: String,
    name: String,
}


#[derive(RustcDecodable)]
pub struct TopicItem {
    titles: Vec<TopicTitleItem>,
    author: TopicAuthorItem,
    last_replied_date: String,
    last_replied_time: String,
    reply_count: String,
    rating: String,
}

fn main() {

    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let w = rustbox.width();
    let h = rustbox.height();
    let title = String::from("高登");
    print_header(&rustbox, w, h, title);
    print_body(&rustbox, w, h, 2, 2);
    rustbox.print(1,
                  23,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  "Press 'q' to quit.");

    // print_cjk_count(&rustbox);
    //
    // let s2 = String::from("<<100%成功率>>如何成為成功?香港Youtuber");
    //
    // rustbox.print(1,
    //               3,
    //               rustbox::RB_BOLD,
    //               Color::White,
    //               Color::Black,
    //               &format!("{}", real_count(s2)));


    loop {
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

fn contains(c: char) -> bool {
    let cjks = vec![(0x4E00..0xA000),
                    (0x3400..0x4DC0),
                    (0x20000..0x2A6E0),
                    (0x2A700..0x2B740),
                    (0x2B740..0x2B820),
                    (0xF900..0xFB00),
                    (0x2F800..0x2FA20),
                    (0x9FA6..0x9FCC)];

    for cjk in cjks {
        let h = c as u32;
        if cjk.start <= h && h < cjk.end {
            return true;
        }
    }
    return false;
}

fn real_count(s: &str) -> usize {
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

fn print_header(rustbox: &rustbox::RustBox, width: usize, height: usize, text: String) {
    let padding = (width - text.len()) / 2;
    let header_bottom = (0..width).map(|i| "─").collect::<Vec<_>>().join("");
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
              height: usize,
              offset_x: usize,
              rows: usize) {
    let titles = vec!["紅魔英超睇敢帥　十分之高招",
                      "發覺好多後生仔女搭火車地鐵 有位都唔坐"];
    let authors = vec!["電超", "程詠樂"];

    let mut title_spacing = 0;
    for i in (0..rows) {

        rustbox.print(0,
                      i + offset_x,
                      rustbox::RB_BOLD,
                      Color::White,
                      Color::Black,
                      &format!("{no:>2}| {title}", no = i + 1, title = &titles[i]));

        rustbox.print(60,
                      i + offset_x,
                      rustbox::RB_BOLD,
                      Color::White,
                      Color::Black,
                      &format!("| {author}", author = &authors[i]));
    }
}



fn print_cjk_count(rustbox: &rustbox::RustBox) {

    let mut offset_y = 2;

    let s1 = String::from("紅魔英超睇敢帥　十分之高招");
    for (b, c) in s1.chars().enumerate() {
        rustbox.print(1,
                      b + 2,
                      rustbox::RB_BOLD,
                      Color::White,
                      Color::Black,
                      &format!("{} 0x{:X}", c, c as u32));
    }

    offset_y = 16;

    let s2 = String::from("<<100%成功率>>如何成為成功?香港Youtuber");
    let mut s2count = 0;
    for (d, c) in s2.chars().enumerate() {
        if contains(c) {
            rustbox.print(1,
                          d + offset_y,
                          rustbox::RB_BOLD,
                          Color::White,
                          Color::Black,
                          &format!("[{:<2}] {:>2} 0x{:X} {}", d + offset_y, c, c as u32, &"YES"));
            s2count = s2count + 2;
        } else {
            rustbox.print(1,
                          d + offset_y,
                          rustbox::RB_BOLD,
                          Color::White,
                          Color::Black,
                          &format!("[{:<2}] {:>2} 0x{:X} {}", d + offset_y, c, c as u32, &"NO"));
            s2count = s2count + 1;
        }
    }

    let sum = s2.chars()
                .map(|x| if contains(x) {
                    2
                } else {
                    1
                })
                .collect::<Vec<u32>>()
                .iter()
                .fold(0, |acc, &x| acc + x);

    rustbox.print(1,
                  45,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  &format!("{} {}", sum, s2count));

}

fn debug_load_and_print_topics()
{
    let s = cache::readfile(String::from("topics.json"));
    let collection: Vec<TopicItem> = json::decode(&s).unwrap();

    println!("topics {:?}", collection.len());
    debug_print_topics(collection);
}

fn debug_print_topics(collection: Vec<TopicItem>) {
    for (i, item) in collection.iter().enumerate() {

        println!("item[{}]= {title} {author_name} {last_replied_date} {last_replied_time} \
                  {reply_count} {rating}",
                 i,
                 title = item.titles[0].text,
                 author_name = item.author.name,
                 last_replied_date = item.last_replied_date,
                 last_replied_time = item.last_replied_time,
                 reply_count = item.reply_count,
                 rating = item.rating);
    }
}

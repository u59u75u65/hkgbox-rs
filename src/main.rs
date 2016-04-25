extern crate hkg;

extern crate rustbox;

use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

fn main() {

    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let w = rustbox.width();
    let h = rustbox.height();
    let title = String::from("高登");
    print_header(&rustbox, w, h, title);
    // print_body(&rustbox, w, h, 2, 2);
    // rustbox.print(1, 23, rustbox::RB_BOLD, Color::White, Color::Black, "Press 'q' to quit.");

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

    let sum = s2.chars().map(|x| if contains(x) { 2 } else { 1 } ).collect::<Vec<u32>>().iter().fold(0, |acc, &x| acc + x);

    rustbox.print(1,
                  45,
                  rustbox::RB_BOLD,
                  Color::White,
                  Color::Black,
                  &format!("{} {}", sum, s2count));


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

    for i in (0..rows) {
        rustbox.print(0,
                      i + offset_x,
                      rustbox::RB_BOLD,
                      Color::White,
                      Color::Black,
                      &format!("{no:>2}|{title:<50}|{author}",
                               no = i + 1,
                               title = &titles[i],
                               author = &authors[i]));
    }
}

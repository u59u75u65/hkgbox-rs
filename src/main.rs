extern crate hkg;

extern crate rustbox;

use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

fn main(){

    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let w = rustbox.width();
    let h = rustbox.height();
    let title = String::from("高登");
    print_header(&rustbox, w, h, title);
    print_body(&rustbox, w, h, 2, 2);
    rustbox.print(1, 23, rustbox::RB_BOLD, Color::White, Color::Black, "Press 'q' to quit.");


    loop {
        rustbox.present();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}


fn print_header(rustbox: &rustbox::RustBox, width: usize, height: usize, text: String)
{
    let padding = (width - text.len())/2;
    let header_bottom = (0..width).map(|i|"─").collect::<Vec<_>>().join("");
    rustbox.print(padding, 0, rustbox::RB_BOLD, Color::White, Color::Black, &text);
    rustbox.print(0, 1, rustbox::RB_BOLD, Color::Yellow, Color::Black, &header_bottom);
}

fn print_body(rustbox: &rustbox::RustBox, width: usize, height: usize, offset_x: usize, rows: usize)
{
    let titles = vec!["紅魔英超睇敢帥　十分之高招", "發覺好多後生仔女搭火車地鐵 有位都唔坐"];
    let authors = vec!["電超","程詠樂"];

    for i in (0..rows)
    {
        rustbox.print(0, i  + offset_x, rustbox::RB_BOLD, Color::White, Color::Black, &format!("{no:>2}|{title:<50}|{author}",
                                                                                                                        no = i+1,
                                                                                                                        title = &titles[i],
                                                                                                                        author = &authors[i]));
    }
}

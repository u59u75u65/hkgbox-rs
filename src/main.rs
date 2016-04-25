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
    print_header(&rustbox, title, w, h);

    rustbox.print(1, 5, rustbox::RB_BOLD, Color::White, Color::Black, "Press 'q' to quit.");

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

fn print_header(rustbox: &rustbox::RustBox, text: String, width: usize, height: usize)
{
    let padding = (width - text.len())/2;
    let header_bottom = (0..width).map(|i|"─").collect::<Vec<_>>().join("");
    rustbox.print(padding, 0, rustbox::RB_BOLD, Color::White, Color::Black, &text);
    rustbox.print(0, 1, rustbox::RB_BOLD, Color::Yellow, Color::Black, &header_bottom);
}

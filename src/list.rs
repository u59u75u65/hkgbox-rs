extern crate rustbox;
use rustbox::{Color, RustBox};
use rustbox::Key;

pub fn print_header(rustbox: &rustbox::RustBox, width: usize, text: &str) {
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

pub fn clearline(rustbox: &rustbox::RustBox, width: usize, x: usize, y: usize) {
    let s = (0..width).map(|_| "  ").collect::<Vec<_>>().join("");

    rustbox.print(x, y, rustbox::RB_NORMAL, Color::White, Color::Black, &s);
}

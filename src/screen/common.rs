extern crate rustbox;
use rustbox::*;

pub fn clearline(rustbox: &rustbox::RustBox, width: usize, x: usize, y: usize) {
    let s = (0..width).map(|_| "  ").collect::<Vec<_>>().join("");

    rustbox.print(x, y, rustbox::RB_NORMAL, Color::White, Color::Black, &s);
}

extern crate rustbox;
use rustbox::*;

pub fn clearline(rustbox: &rustbox::RustBox, width: usize, x: usize, y: usize) {
    let s = (0..width).map(|_| "  ").collect::<Vec<_>>().join("");

    rustbox.print(x, y, rustbox::RB_NORMAL, Color::White, Color::Black, &s);
}

// not sure why the rustbox.clear() can not clear screen properly.
pub fn clear(rustbox: &rustbox::RustBox)
{
    for i in (0..rustbox.height())
    {
        clearline(&rustbox, rustbox.width(), 0, i);
    }

    rustbox.present();
    rustbox.clear();
    rustbox.present();
}

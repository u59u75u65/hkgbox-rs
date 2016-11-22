extern crate termion;
extern crate rustc_serialize;
extern crate hyper;

use rustc_serialize::base64::{self, ToBase64};
use rustc_serialize::hex::FromHex;

use std::fs::File;
use std::fs;
use std::io::{Error, ErrorKind};
use std::io::Read;

use termion::style;

use self::hyper::Client;
use self::hyper::header::Connection;

pub fn imgcat(path: &str, width: usize) -> String {
    let mut f = match File::open(path) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer);

    let e = buffer.as_slice().to_base64(base64::STANDARD);
    return String::from(format!("\x1b]1337;File=inline=1;width={width};:{code}\x07", width = width,  code = e));
}

pub fn imgcatUrl(url: &str, height: usize) -> String {
    let mut c = Client::new();
    return match c.get(url).send() {
        Ok(mut resp) => {
                let mut buffer = Vec::new();
                resp.read_to_end(&mut buffer);
                let e = buffer.as_slice().to_base64(base64::STANDARD);
                return String::from(format!("\x1b]1337;File=inline=1;height={height};:{code}\x07", height = height,  code = e));
            }
        Err(e) => String::from("[IMG FAIL]")
    }
}

pub fn reset_screen() {
    print!("{}{}{}", termion::clear::All, style::Reset, termion::cursor::Show);
}

pub fn clear_screen () {
    print!("{}", termion::clear::All);
}

extern crate rustc_serialize;

use rustc_serialize::base64::{self, ToBase64};
use rustc_serialize::hex::FromHex;

use std::fs::File;
use std::fs;
use std::io::{Error, ErrorKind};
use std::io::Read;

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

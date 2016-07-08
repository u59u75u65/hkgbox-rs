extern crate termion;

use termion::{TermRead, TermWrite, IntoRawMode, Color, Style, Key};
use termion::terminal_size;
use std::io::{Read, Write, Stdout, Stdin};
use std::io::{stdout, stdin};
use std;

pub struct Index<'a, W: 'a> {
    stdout: &'a mut W
}

impl<'a, W: Write> Index<'a, W> {
    pub fn new(stdout: &'a mut W) -> Self {
        Index {
            stdout: stdout
        }
    }

    pub fn print(&mut self) {
        self.stdout.goto(10 as u16, 3 as u16).unwrap();
        self.stdout.write("hello world".as_bytes()).unwrap();
    }
}

fn screen_goto(stdout: &Stdout, x: usize, y: usize) {
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    stdout.goto(x as u16,y as u16).unwrap();
}

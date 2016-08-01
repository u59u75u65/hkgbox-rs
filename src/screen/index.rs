extern crate termion;

use termion::{TermRead, TermWrite, IntoRawMode, Color, Style, Key};
use termion::terminal_size;
use std::io::{Read, Write, Stdout, Stdin};
use std::io::{stdout, stdin};
use std;
use model::ListTopicItem;


pub struct Index {
    title: String,
    selected_topic_index: usize,
}

impl Index {
    pub fn new() -> Self {
        Index {
            title: String::from("高登"),
            selected_topic_index: 0,
        }
    }

    pub fn select_topic(&mut self, index: usize) {
        self.selected_topic_index = index;
    }

    pub fn get_selected_topic(&self) -> usize {
        self.selected_topic_index
    }

    pub fn print(&mut self, stdout: &mut termion::RawTerminal<std::io::StdoutLock>, collection: &Vec<ListTopicItem>) {

        if self.selected_topic_index > self.body_height() {
            self.selected_topic_index = self.body_height();
        }

        let width = terminal_size().unwrap().0 as usize;

        print_header(stdout, width as usize, &self.title);
    }

    pub fn body_height(&self) -> usize {

        let h = terminal_size().unwrap().1;

        if h >= 3 {
            h as usize - 3
        } else {
            0
        }
    }

    pub fn body_width(&self) -> usize {

        let w = terminal_size().unwrap().0;

        if w >= 2 {
            w as usize - 2
        } else {
            0
        }
    }

}

fn print_header(stdout: &mut termion::RawTerminal<std::io::StdoutLock>, width: usize, text: &str) {
    // print header
    let padding = ((width - text.len()) / 2) as u16;
    let header_bottom = (0..width).map(|_| "─").collect::<Vec<_>>().join("");

    stdout.goto(padding, 0).unwrap();
    stdout.color(Color::White).unwrap();
    stdout.bg_color(Color::Black).unwrap();
    stdout.style(Style::Bold).unwrap();
    stdout.write(text.as_bytes()).unwrap();

    stdout.goto(0, 1).unwrap();
    stdout.color(Color::Yellow).unwrap();
    stdout.bg_color(Color::Black).unwrap();
    stdout.style(Style::Bold).unwrap();
    stdout.write(header_bottom.as_bytes()).unwrap();

    stdout.hide_cursor().unwrap();
    stdout.reset().unwrap();
}

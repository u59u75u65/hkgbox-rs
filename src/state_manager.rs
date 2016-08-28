use status::*;
use termion::terminal_size;

pub struct StateManager {
    current_state: Status,
    prev_state: Status,
    current_width: usize,
    prev_width: usize,
    is_web_requesting: bool
}

impl StateManager {
    pub fn new () -> Self {
        StateManager {
            current_state: Status::Startup,
            prev_state: Status::Startup,
            current_width: 0,
            prev_width: 0,
            is_web_requesting: false
        }
    }
    pub fn updateWidth(&mut self) -> usize {
        let w = terminal_size().unwrap().0 as usize;
        self.prev_width = self.current_width;
        self.current_width = w;
        w
    }
    pub fn isWidthChanged(&self) -> bool {
        self.current_width != self.prev_width
    }
    pub fn isWebRequest (&self) -> bool {
        self.is_web_requesting
    }
    pub fn setWebRequest(&mut self, value: bool) {
        self.is_web_requesting = value;
    }
    pub fn updateState(&mut self, value: Status) {
        // if self.current_state != value {
        self.prev_state = self.current_state;
        self.current_state = value;
        // }
    }
    pub fn getState(&self) -> Status {
        self.current_state
    }
    pub fn isStateChanged(&self) -> bool {
        self.current_state != self.prev_state
    }
}

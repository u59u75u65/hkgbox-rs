use status::*;

#[derive(Clone)]
pub struct StateManager {
    current_state: Status,
    prev_state: Status,
    is_web_requesting: bool
}

impl StateManager {
    pub fn new () -> Self {
        StateManager {
            current_state: Status::Startup,
            prev_state: Status::Startup,
            is_web_requesting: false
        }
    }
    pub fn isWebRequest (&self) -> bool {
        self.is_web_requesting
    }
    pub fn setWebRequest(&mut self, value: bool) {
        self.is_web_requesting = value;
    }
    pub fn updateState(&mut self, value: Status) {
        if self.current_state != value {
            self.prev_state = self.current_state;
            self.current_state = value;
        }
    }
    pub fn getState(&self) -> Status {
        self.current_state
    }
}

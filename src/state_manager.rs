use status::*;

use std::sync::mpsc::Sender;

#[derive(Clone)]
pub struct StateManager {
    current_state: Status,
    prev_state: Status,
    is_web_requesting: bool,
    is_bg_requesting: bool,
    tx_state: Sender<(Status,Status)>
}

impl StateManager {
    pub fn new (tx_state: Sender<(Status,Status)>) -> Self {
        StateManager {
            current_state: Status::Startup,
            prev_state: Status::Startup,
            is_web_requesting: false,
            is_bg_requesting: false,
            tx_state: tx_state
        }
    }
    pub fn is_web_request (&self) -> bool {
        self.is_web_requesting
    }
    pub fn set_web_request(&mut self, value: bool) {
        self.is_web_requesting = value;
    }

    pub fn is_bg_request (&self) -> bool {
        self.is_bg_requesting
    }
    pub fn set_bg_request(&mut self, value: bool) {
        self.is_bg_requesting = value;
    }

    pub fn update_state(&mut self, value: Status) {
        if self.current_state != value {
            self.tx_state.send((self.prev_state, self.current_state));
            self.prev_state = self.current_state;
            self.current_state = value;
        }
    }

    pub fn get_state(&self) -> Status {
        self.current_state
    }
}
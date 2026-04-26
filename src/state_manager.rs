use crate::status::*;

use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Represents the current request state of the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestState {
    Idle,
    WebRequest,
    BackgroundRequest,
}

impl Default for RequestState {
    fn default() -> Self {
        RequestState::Idle
    }
}

#[derive(Clone)]
pub struct StateManager {
    current_state: Status,
    prev_state: Status,
    request_state: RequestState,
    tx_state: Sender<(Status,Status)>,
    to_print_screen: Arc<AtomicBool>
}

impl StateManager {
    pub fn new (tx_state: Sender<(Status,Status)>) -> Self {
        StateManager {
            current_state: Status::Startup,
            prev_state: Status::Startup,
            request_state: RequestState::Idle,
            tx_state: tx_state,
            to_print_screen: Arc::new(AtomicBool::new(false))
        }
    }

    pub fn is_web_request (&self) -> bool {
        matches!(self.request_state, RequestState::WebRequest)
    }

    pub fn is_bg_request (&self) -> bool {
        matches!(self.request_state, RequestState::BackgroundRequest)
    }

    pub fn set_request_state(&mut self, state: RequestState) {
        self.request_state = state;
    }

    pub fn get_request_state(&self) -> RequestState {
        self.request_state
    }

    pub fn set_web_request(&mut self, value: bool) {
        self.request_state = if value {
            RequestState::WebRequest
        } else {
            RequestState::Idle
        };
    }

    pub fn set_bg_request(&mut self, value: bool) {
        self.request_state = if value {
            RequestState::BackgroundRequest
        } else {
            RequestState::Idle
        };
    }

    pub fn update_state(&mut self, value: Status) {
        if self.current_state != value {
            let prev_state_tmp = self.prev_state.clone();
            let current_state_tmp = self.current_state.clone();
            self.prev_state = self.current_state;
            self.current_state = value;

            let _ = self.tx_state.send((prev_state_tmp, current_state_tmp));
        }
    }

    pub fn get_state(&self) -> Status {
        self.current_state
    }

    pub fn is_to_print_screen(&self) -> bool {
        (*self.to_print_screen.clone()).load(Ordering::Relaxed)
    }

    pub fn set_to_print_screen(&mut self, value: bool) {
        (*self.to_print_screen).store(value, Ordering::Relaxed)
    }
}

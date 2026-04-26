use log::info;
use termion::event::Key;
use crate::control::common as control_common;

pub struct Dialog {

}

impl Dialog {
    pub fn new() -> Self { Dialog {} }

    pub fn handle(&mut self, c: ::termion::event::Key, app: &mut crate::App) -> Option<i32> {
        match c {
            Key::Esc => {
                info!("[Dialog] ESC pressed, canceling dialog");
                app.dialog.hide();
                app.state_manager.update_state(app.prev_state);
                Some(1)
            }
            Key::Char('\n') => {
                let thread_id = app.dialog.get_input();
                info!("[Dialog] Enter pressed, thread_id: {}", thread_id);

                if thread_id.is_empty() {
                    app.dialog.hide();
                    app.state_manager.update_state(app.prev_state);
                    return Some(1);
                }

                // Send page request with the thread ID
                let status_message = control_common::send_page_request(thread_id, 1, &mut app.state_manager, &app.tx_req);
                app.status_bar.append(&app.screen_manager,
                                      &control_common::format_page_status(thread_id, 1, &status_message));

                app.dialog.hide();
                // State will transition to Show when response arrives
                Some(1)
            }
            Key::Char(c) => {
                // Only allow numeric input
                if c.is_ascii_digit() {
                    app.dialog.add_char(c);
                    Some(1)
                } else {
                    None
                }
            }
            Key::Backspace => {
                app.dialog.backspace();
                Some(1)
            }
            Key::Ctrl('c') => {
                // Allow Ctrl-C to exit
                Some(0)
            }
            _ => None,
        }
    }
}

use log::info;
use termion::event::Key;
use crate::control::common as control_common;

pub struct PageDialog {

}

impl PageDialog {
    pub fn new() -> Self { PageDialog {} }

    pub fn handle(&mut self, c: ::termion::event::Key, app: &mut crate::App) -> Option<i32> {
        match c {
            Key::Esc => {
                info!("[PageDialog] ESC pressed, canceling dialog");
                app.page_dialog.hide();
                app.state_manager.update_state(app.prev_state);
                Some(1)
            }
            Key::Char('\n') => {
                // Validate page number
                if let Some(page) = app.page_dialog.get_valid_page() {
                    info!("[PageDialog] Enter pressed, navigating to page: {}", page);

                    let postid = app.show_item.url_query.message.as_str();
                    let status_message = control_common::send_page_request(
                        postid,
                        page,
                        &mut app.state_manager,
                        &app.tx_req
                    );
                    app.status_bar.append(&app.screen_manager,
                                          &control_common::format_page_status(postid, page, &status_message));

                    app.page_dialog.hide();
                    app.state_manager.update_state(crate::status::Status::Show);
                    Some(1)
                } else {
                    // Invalid input, just hide dialog
                    app.page_dialog.hide();
                    app.state_manager.update_state(app.prev_state);
                    Some(1)
                }
            }
            Key::Char(c) => {
                app.page_dialog.add_char(c);
                Some(1)
            }
            Key::Backspace => {
                app.page_dialog.backspace();
                Some(1)
            }
            Key::Ctrl('c') => {
                Some(0)
            }
            _ => None,
        }
    }
}

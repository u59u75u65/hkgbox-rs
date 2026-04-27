//! Show view input handling
//!
//! This module handles keyboard input processing for the show (thread view) screen.
//! It manages navigation, pagination, and refresh operations.

use termion::event::Key;
use crate::status::*;
use crate::control::common as control_common;

/// Input handler for the show view
///
/// The `Show` handler processes keyboard events when the application is in
/// the `Status::Show` state, managing thread content navigation and interaction.
///
/// # Key Bindings
/// - `c` - Open channel selection dialog
/// - `Ctrl+O` - Open thread by ID dialog
/// - `q` - Quit application
/// - `r` / `Ctrl+R` - Refresh current page (fetch from API)
/// - `←/→` - Navigate pages
/// - `↑/↓` / `PageUp/PageDown` - Scroll content
/// - `Backspace` - Return to index
pub struct Show {

}

impl Show {

    pub fn new () -> Self { Show {} }
    pub fn handle(&mut self, c: ::termion::event::Key, app: &mut crate::App) -> Option<i32> {
        match c {
            Key::Char('c') => {
                app.prev_state = app.state_manager.get_state();
                app.channel_dialog.show();
                app.state_manager.update_state(Status::ChannelDialog);
                Some(1)
            }
            Key::Ctrl('o') => {
                app.prev_state = app.state_manager.get_state();
                app.dialog.show();
                app.state_manager.update_state(Status::Dialog);
                Some(1)
            }
            Key::Char('q') => {
                crate::screen::common::reset_screen(); // print!("{}{}{}", termion::clear::All, style::Reset, termion::cursor::Show);
                Some(0)
            }
            Key::Char('r') | Key::Ctrl('r') => {
                if !app.state_manager.is_web_request() {
                    // Refresh current show page
                    let postid = app.show_item.url_query.message.as_str();
                    let page = app.show_item.page;
                    let status_message = control_common::send_page_request(postid, page, &mut app.state_manager, &app.tx_req);
                    app.status_bar.append(&app.screen_manager,
                                          &control_common::format_page_status(postid, page, &status_message));
                } else {
                    app.status_bar.append(&app.screen_manager, "[r][BUSY]");
                }
                Some(1)
            }
            Key::Left => {
                app.status_bar.append(&app.screen_manager, &format!("←"));
                if app.show_item.page > 1 {
                    let postid = app.show_item.url_query.message.as_str();
                    let page = &app.show_item.page - 1;
                    let status_message = control_common::send_page_request(postid, page, &mut app.state_manager, &app.tx_req);

                    app.status_bar.append(&app.screen_manager,
                                          &control_common::format_page_status(postid, page, &status_message));
                }
                Some(1)
            }
            Key::Right => {
                app.status_bar.append(&app.screen_manager, &format!("→"));
                if app.show_item.max_page > app.show_item.page {
                    let postid = app.show_item.url_query.message.as_str();
                    let page = &app.show_item.page + 1;
                    let status_message = control_common::send_page_request(postid, page, &mut app.state_manager, &app.tx_req);

                    app.status_bar.append(&app.screen_manager,
                                          &control_common::format_page_status(postid, page, &status_message));
                }
                Some(1)
            }
            Key::PageUp => {
                app.status_bar.append(&app.screen_manager, "↑");
                let bh = app.show.body_height();
                if app.show.scroll_up(bh) {
                    crate::screen::common::clear_screen();
                }
                Some(1)
            }
            Key::PageDown => {
                app.status_bar.append(&app.screen_manager, "↓");
                let bh = app.show.body_height();
                if app.show.scroll_down(bh) {
                    crate::screen::common::clear_screen();
                }
                Some(1)
            }
            Key::Up => {
                app.status_bar.append(&app.screen_manager, "↑");
                if app.show.scroll_up(2) {
                    crate::screen::common::clear_screen();
                }
                Some(1)
            }
            Key::Down => {
                app.status_bar.append(&app.screen_manager, "↓");
                if app.show.scroll_down(2) {
                    crate::screen::common::clear_screen();
                }
                Some(1)
            }
            Key::Backspace => {
                app.status_bar.append(&app.screen_manager, "B");
                app.state_manager.update_state(Status::List); // state = Status::List;
                crate::screen::common::clear_screen();
                Some(1)
            }
            _ => None,
        }
    }
}

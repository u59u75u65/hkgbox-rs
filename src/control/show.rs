use std::sync::mpsc::Sender;
use termion::event::Key;
use crate::status::*;
use crate::state_manager::*;
use crate::resources::*;
use crate::control::common as control_common;

pub struct Show {

}

impl Show {

    pub fn new () -> Self { Show {} }
    pub fn handle(&mut self, c: ::termion::event::Key, app: &mut crate::App) -> Option<i32> {
        match c {
            Key::Char('q') => {
                crate::screen::common::reset_screen(); // print!("{}{}{}", termion::clear::All, style::Reset, termion::cursor::Show);
                Some(0)
            }
            Key::Char('r') => {
                crate::screen::common::clear_screen();
                app.status_bar.append(&app.screen_manager, &format!("r"));
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

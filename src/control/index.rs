use log::info;
use std::sync::mpsc::Sender;
use termion::event::Key;
use crate::state_manager::*;
use crate::resources::*;
use crate::control::common as control_common;

use std::default::Default;

pub struct Index {

}

impl Index {

    pub fn new () -> Self { Index {} }
    pub fn handle(&mut self, c: ::termion::event::Key, app: &mut crate::App) -> Option<i32> {
        match c {
            Key::Char('q') => {
                crate::screen::common::reset_screen();
                Some(0)
            }
            Key::Char('r') => {
                crate::screen::common::clear_screen();
                app.status_bar.append(&app.screen_manager, &format!("r"));
                Some(1)
            }
            Key::Char('\n') => {
                if !app.state_manager.is_web_request() {
                    app.status_bar.append(&app.screen_manager, "[ENTER]");
                    let i = app.index.get_selected_topic();
                    if i > 0 {
                        info!("select topic: {}", i);
                        let topic_item = &app.list_topic_items[i - 1];
                        let postid = topic_item.title.url_query.message.as_str();
                        let page = 1;
                        let status_message = control_common::send_page_request(postid, page, &mut app.state_manager, &app.tx_req);

                        app.status_bar.append(&app.screen_manager,
                                              &control_common::format_page_status(postid, page, &status_message));
                    }
                } else {
                    app.status_bar.append(&app.screen_manager, "[ENTER][BUSY]");
                }
                Some(1)
            }
            Key::PageUp => {
                app.status_bar.append(&app.screen_manager, "↑");
                let tmp = app.index.get_selected_topic();
                app.status_bar.append(&app.screen_manager, &format!("{}", tmp));

                if tmp > 1 {
                    app.index.select_topic(tmp - 1);
                }
                Some(1)
            }
            Key::Up => {
                app.status_bar.append(&app.screen_manager, "↑");
                let tmp = app.index.get_selected_topic();
                app.status_bar.append(&app.screen_manager, &format!("{}", tmp));

                if tmp > 1 {
                    app.index.select_topic(tmp - 1);
                }
                Some(1)
            }
            Key::Down => {
                app.status_bar.append(&app.screen_manager, "↓");
                let tmp = app.index.get_selected_topic();
                app.status_bar.append(&app.screen_manager, &format!("{}", tmp));

                if tmp < app.index.body_height() {
                    app.index.select_topic(tmp + 1);
                }
                Some(1)
            }
            _ => None,
        }
    }

}


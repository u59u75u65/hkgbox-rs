
use std::io::{stdout, stdin, Write};
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use cancellation::{CancellationToken, CancellationTokenSource, OperationCanceled};
use kuchiki::traits::*;
use rustc_serialize::json;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::event::Key;
use status::*;
use model::IconItem;
use model::ListTopicItem;
use state_manager::*;
use screen_manager::*;
use caches::file_cache::*;
use resources::*;
use resources::web_resource::*;
use resources::common::*;
use web::*;
use responser::*;
use log4rs::*;

pub struct Show {

}

impl Show {

    pub fn new () -> Self { Show {} }
    pub fn handle(&mut self, c: ::termion::event::Key,app: &mut ::App)-> Option<i32> {
        match c {
            Key::Char('q') => {
                ::screen::common::reset_screen(); // print!("{}{}{}", termion::clear::All, style::Reset, termion::cursor::Show);
                Some(0)
            }
            Key::Left => {
                app.status_bar.append(&app.screen_manager, &format!("←"));
                if app.show_item.page > 1 {
                    let postid = &app.show_item.url_query.message;
                    let page = &app.show_item.page - 1;
                    let status_message = show_page(&postid, page, &mut app.state_manager, &app.tx_req);

                    app.status_bar.append(&app.screen_manager,
                                          &get_show_page_status_message(postid, page, &status_message));
                }
                Some(1)
            }
            Key::Right => {
                app.status_bar.append(&app.screen_manager, &format!("→"));
                if app.show_item.max_page > app.show_item.page {
                    let postid = &app.show_item.url_query.message;
                    let page = &app.show_item.page + 1;
                    let status_message = show_page(&postid, page, &mut app.state_manager, &app.tx_req);

                    app.status_bar.append(&app.screen_manager,
                                          &get_show_page_status_message(postid, page, &status_message));
                }
                Some(1)
            }
            Key::PageUp => {
                app.status_bar.append(&app.screen_manager, "↑");
                let bh = app.show.body_height();
                if app.show.scrollUp(bh) {
                    ::screen::common::clear_screen();
                }
                Some(1)
            }
            Key::PageDown => {
                app.status_bar.append(&app.screen_manager, "↓");
                let bh = app.show.body_height();
                if app.show.scrollDown(bh) {
                    ::screen::common::clear_screen();
                }
                Some(1)
            }
            Key::Up => {
                app.status_bar.append(&app.screen_manager, "↑");
                if app.show.scrollUp(2) {
                    ::screen::common::clear_screen();
                }
                Some(1)
            }
            Key::Down => {
                app.status_bar.append(&app.screen_manager, "↓");
                if app.show.scrollDown(2) {
                    ::screen::common::clear_screen();
                }
                Some(1)
            }
            Key::Backspace => {
                app.status_bar.append(&app.screen_manager, "B");
                app.state_manager.updateState(Status::List); // state = Status::List;
                ::screen::common::clear_screen();
                Some(1)
            }
            _ => None,
        }
    }
}


fn show_page(postid: &String, page: usize, state_manager: &mut StateManager, tx_req: &Sender<ChannelItem>) -> String {

    let ci = ChannelItem {
        extra: ChannelItemType::Show(ChannelShowItem {
                                         postid: postid.clone(),
                                         page: page,
                                     }),
        result: String::from(""),
    };

    let status_message = match tx_req.send(ci) {
        Ok(()) => {
            state_manager.setWebRequest(true); // *is_web_requesting = true;
            "SOK".to_string()
        }
        Err(e) => format!("{}:{}", "SFAIL", e).to_string(),
    };

    status_message
}

fn get_show_page_status_message(postid: &String, page: usize, status_message: &String) -> String {
    format!("[{}-{}:{}]", postid, page, status_message)
}

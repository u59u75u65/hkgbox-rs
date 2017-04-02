use std::sync::mpsc::Sender;
use termion::event::Key;
use state_manager::*;
use resources::*;

use std::default::Default;

pub struct Index {

}

impl Index {

    pub fn new () -> Self { Index {} }
    pub fn handle(&mut self, c: ::termion::event::Key,app: &mut ::App)-> Option<i32> {
        match c {
            Key::Char('q') => {
                ::screen::common::reset_screen();
                Some(0)
            }
            Key::Char('\n') => {
                app.status_bar.append(&app.screen_manager, "ENTER");
                let i = app.index.get_selected_topic();
                if i > 0 {
                    let topic_item = &app.list_topic_items[i - 1];
                    let postid = &topic_item.title.url_query.message;
                    let page = 1;
                    let status_message = show_page(&postid, page, &mut app.state_manager, &app.tx_req);

                    app.status_bar.append(&app.screen_manager,
                                          &get_show_page_status_message(postid, page, &status_message));
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

fn show_page(postid: &String, page: usize, state_manager: &mut StateManager, tx_req: &Sender<ChannelItem>) -> String {

    let ci = ChannelItem {
        extra: Some( ChannelItemType::Show(ChannelShowItem {
                                         postid: postid.clone(),
                                         page: page,
                                     })),
        result: Default::default(),
    };

    let status_message = match tx_req.send(ci) {
        Ok(()) => {
            state_manager.set_web_request(true); // *is_web_requesting = true;
            "SOK".to_string()
        }
        Err(e) => format!("{}:{}", "SFAIL", e).to_string(),
    };

    status_message
}

fn get_show_page_status_message(postid: &String, page: usize, status_message: &String) -> String {
    format!("[{}-{}:{}]", postid, page, status_message)
}

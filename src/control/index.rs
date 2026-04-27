//! Index view input handling
//!
//! This module handles keyboard input processing for the index (topic list) view.
//! It manages navigation, topic selection, pagination, and mode switching.

use log::info;
use termion::event::Key;
use crate::control::common as control_common;
use crate::status::Status;

/// Input handler for the index view
///
/// The `Index` handler processes keyboard events when the application is in
/// the `Status::List` state, managing topic list navigation and interaction.
///
/// # Key Bindings
/// - `c` - Open channel selection dialog
/// - `Ctrl+O` - Open thread by ID dialog
/// - `q` - Quit application
/// - `r` - Refresh screen
/// - `Enter` - Open selected topic
/// - `↑/↓` - Navigate topics
/// - `←/→` - Navigate pages
pub struct Index {

}

impl Index {
    /// Create a new index input handler
    ///
    /// # Examples
    /// ```
    /// use hkg::control::index::Index;
    ///
    /// let handler = Index::new();
    /// ```
    #[must_use]
    pub fn new() -> Self { Index {} }

    /// Handle a keyboard event in the index view
    ///
    /// # Arguments
    /// * `c` - The keyboard event to handle
    /// * `app` - Mutable reference to the application state
    ///
    /// # Returns
    /// * `Some(1)` - Event was handled, screen should be redrawn
    /// * `Some(0)` - Quit requested
    /// * `None` - Event not handled or no screen update needed
    ///
    /// # Examples
    /// ```no_run
    /// # use hkg::control::index::Index;
    /// # use hkg::App;
    /// let mut handler = Index::new();
    /// # let mut app: App = unsafe { std::mem::zeroed() };
    /// use termion::event::Key;
    ///
    /// match handler.handle(Key::Char('q'), &mut app) {
    ///     Some(0) => println!("Quit requested"),
    ///     Some(1) => println!("Screen updated"),
    ///     None => println!("No change"),
    /// }
    /// ```
    #[allow(clippy::too_many_lines)]
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

                let body_height = app.index.body_height();
                let max_items = body_height.min(app.list_topic_items.len());
                if tmp < max_items {
                    app.index.select_topic(tmp + 1);
                }
                Some(1)
            }
            Key::Left => {
                app.status_bar.append(&app.screen_manager, "←");
                if app.index_page > 1 {
                    let new_page = app.index_page - 1;
                    app.index_page = new_page;
                    app.index.select_topic(1);
                    let body_height = app.index.body_height();
                    let page_count = calculate_page_count(body_height);
                    let status_message = control_common::send_index_page_request_with_count(
                        new_page,
                        page_count,
                        &mut app.state_manager,
                        &app.tx_req,
                        &app.current_channel
                    );
                    app.status_bar.append(&app.screen_manager,
                                          &control_common::format_index_page_status(new_page, &status_message));
                }
                Some(1)
            }
            Key::Right => {
                app.status_bar.append(&app.screen_manager, "→");
                if app.index_page < app.index_max_page {
                    let new_page = app.index_page + 1;
                    app.index_page = new_page;
                    app.index.select_topic(1);
                    let body_height = app.index.body_height();
                    let page_count = calculate_page_count(body_height);
                    let status_message = control_common::send_index_page_request_with_count(
                        new_page,
                        page_count,
                        &mut app.state_manager,
                        &app.tx_req,
                        &app.current_channel
                    );
                    app.status_bar.append(&app.screen_manager,
                                          &control_common::format_index_page_status(new_page, &status_message));
                }
                Some(1)
            }
            _ => None,
        }
    }

}

/// Calculate how many API pages to fetch based on display capacity
/// API returns max 30 items per page
fn calculate_page_count(body_height: usize) -> usize {
    const API_MAX_PER_PAGE: usize = 30;
    if body_height <= API_MAX_PER_PAGE {
        1
    } else {
        (body_height + API_MAX_PER_PAGE - 1) / API_MAX_PER_PAGE
    }
}


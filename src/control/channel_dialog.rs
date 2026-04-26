use log::info;
use termion::event::Key;
use crate::control::common as control_common;
use crate::status::Status;

pub struct ChannelDialog {

}

impl ChannelDialog {
    pub fn new() -> Self { ChannelDialog {} }

    pub fn handle(&mut self, c: ::termion::event::Key, app: &mut crate::App) -> Option<i32> {
        match c {
            Key::Esc => {
                if app.channel_dialog.is_filter_mode() {
                    // Exit filter mode
                    app.channel_dialog.exit_filter_mode();
                    Some(1)
                } else {
                    // Close dialog
                    info!("[ChannelDialog] ESC pressed, canceling dialog");
                    app.channel_dialog.hide();
                    app.state_manager.update_state(app.prev_state);
                    Some(1)
                }
            }
            Key::Char('/') => {
                if !app.channel_dialog.is_filter_mode() {
                    // Enter filter mode
                    app.channel_dialog.enter_filter_mode();
                }
                Some(1)
            }
            Key::Up => {
                app.channel_dialog.move_up();
                Some(1)
            }
            Key::Down => {
                app.channel_dialog.move_down();
                Some(1)
            }
            Key::Char('\n') => {
                // Enter confirms selection
                let index = app.channel_dialog.get_selected_index() + 1;
                self.select_channel(app, index)
            }
            Key::Backspace => {
                if app.channel_dialog.is_filter_mode() {
                    app.channel_dialog.backspace_filter();
                    Some(1)
                } else {
                    None
                }
            }
            Key::Char(c) => {
                if app.channel_dialog.is_filter_mode() {
                    // Only allow alphanumeric characters and space
                    if c.is_alphanumeric() || c == ' ' {
                        app.channel_dialog.add_filter_char(c);
                        Some(1)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Key::Ctrl('c') => {
                Some(0)
            }
            _ => None,
        }
    }

    fn select_channel(&mut self, app: &mut crate::App, index: usize) -> Option<i32> {
        if let Some(channel_info) = app.channel_dialog.get_channel(index) {
            let channel_info = channel_info.clone();
            info!("[ChannelDialog] Selected channel {}: {}", channel_info.channel, channel_info.title);

            // Update current channel with both code and title
            app.current_channel = channel_info.channel.clone();
            app.current_channel_title = channel_info.title.clone();
            app.index.set_channel(channel_info.channel.clone(), channel_info.title.clone());

            // Reset to page 1 when switching channels
            app.index_page = 1;
            app.index_max_page = 1;
            app.index.set_page(1, 1);

            // Hide dialog
            app.channel_dialog.hide();

            // Request topics from new channel
            // Calculate how many API pages to fetch based on terminal height
            const API_MAX_PER_PAGE: usize = 30;
            let body_height = app.index.body_height();
            let page_count = if body_height <= API_MAX_PER_PAGE {
                1
            } else {
                (body_height + API_MAX_PER_PAGE - 1) / API_MAX_PER_PAGE
            };

            let status_message = control_common::send_index_page_request_with_count(
                1,
                page_count,
                &mut app.state_manager,
                &app.tx_req,
                &app.current_channel
            );
            app.status_bar.append(&app.screen_manager,
                                  &format!("[Channel: {} {}]", channel_info.channel, channel_info.title));
            app.status_bar.append(&app.screen_manager,
                                  &control_common::format_index_page_status(1, &status_message));

            app.state_manager.update_state(Status::List);
            Some(1)
        } else {
            None
        }
    }
}

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
                info!("[ChannelDialog] ESC pressed, canceling dialog");
                app.channel_dialog.hide();
                app.state_manager.update_state(app.prev_state);
                Some(1)
            }
            Key::Char('1') | Key::Char('2') | Key::Char('3') | Key::Char('4') |
            Key::Char('5') | Key::Char('6') | Key::Char('7') | Key::Char('8') |
            Key::Char('9') => {
                let digit = match c {
                    Key::Char('1') => 1,
                    Key::Char('2') => 2,
                    Key::Char('3') => 3,
                    Key::Char('4') => 4,
                    Key::Char('5') => 5,
                    Key::Char('6') => 6,
                    Key::Char('7') => 7,
                    Key::Char('8') => 8,
                    Key::Char('9') => 9,
                    _ => 1,
                };
                self.select_channel(app, digit)
            }
            Key::Char('\n') => {
                // Enter without selection - just close
                app.channel_dialog.hide();
                app.state_manager.update_state(app.prev_state);
                Some(1)
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

            // Update current channel
            app.current_channel = channel_info.channel.clone();
            app.index.set_channel(channel_info.channel.clone());

            // Reset to page 1 when switching channels
            app.index_page = 1;
            app.index_max_page = 1;
            app.index.set_page(1, 1);

            // Hide dialog
            app.channel_dialog.hide();

            // Request topics from new channel
            let status_message = control_common::send_index_page_request(
                1,
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

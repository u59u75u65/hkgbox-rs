use std::io::Write;
use crate::utility::string::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelInfo {
    pub title: String,
    pub channel: String,
}

pub struct ChannelDialog {
    title: String,
    channels: Vec<ChannelInfo>,
    visible: bool,
}

impl ChannelDialog {
    pub fn new() -> Self {
        let channels = vec![
            ChannelInfo { title: "吹水台".to_string(), channel: "BW".to_string() },
            ChannelInfo { title: "高登熱".to_string(), channel: "HT".to_string() },
            ChannelInfo { title: "最　新".to_string(), channel: "NW".to_string() },
            ChannelInfo { title: "時事台".to_string(), channel: "CA".to_string() },
            ChannelInfo { title: "娛樂台".to_string(), channel: "ET".to_string() },
            ChannelInfo { title: "體育台".to_string(), channel: "SP".to_string() },
            ChannelInfo { title: "財經台".to_string(), channel: "FN".to_string() },
            ChannelInfo { title: "學術台".to_string(), channel: "ST".to_string() },
            ChannelInfo { title: "講故台".to_string(), channel: "SY".to_string() },
            ChannelInfo { title: "創意台".to_string(), channel: "EP".to_string() },
            ChannelInfo { title: "超自然台".to_string(), channel: "SN".to_string() },
            ChannelInfo { title: "優惠台".to_string(), channel: "CP".to_string() },
        ];

        ChannelDialog {
            title: String::from("Select Channel"),
            channels,
            visible: false,
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn get_channel(&self, index: usize) -> Option<&ChannelInfo> {
        if index > 0 && index <= self.channels.len() {
            self.channels.get(index - 1)
        } else {
            None
        }
    }

    pub fn get_channels(&self) -> &[ChannelInfo] {
        &self.channels
    }

    pub fn print(&mut self, stdout: &mut ::termion::raw::RawTerminal<std::io::StdoutLock>) {
        let width = ::termion::terminal_size().expect("fail to get terminal size").0 as usize;
        let height = ::termion::terminal_size().expect("fail to get terminal size").1 as usize;

        let dialog_width = 35.min(width - 4);
        let dialog_height = 18;

        let dialog_x = ((width - dialog_width) / 2) as u16;
        let dialog_y = ((height - dialog_height) / 2) as u16;

        // Draw corners and edges by position
        // Top-left corner
        write!(stdout, "{}{}┌{}",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + 1),
               ::termion::color::Fg(::termion::color::White),
               ::termion::style::Reset).expect("fail to write to shell");

        // Top border
        for i in 0..dialog_width as u16 - 2 {
            write!(stdout, "{}─",
                   ::termion::cursor::Goto(dialog_x + 2 + i, dialog_y + 1)).expect("fail to write to shell");
        }

        // Top-right corner
        write!(stdout, "{}┐",
               ::termion::cursor::Goto(dialog_x + dialog_width as u16, dialog_y + 1)).expect("fail to write to shell");

        // Title line with sides
        let title_width = jks_len(&self.title);
        let title_padding = dialog_width.saturating_sub(title_width + 2);
        let title_left = " ".repeat(title_padding / 2);
        let title_right = " ".repeat(title_padding - title_padding / 2);
        let title_line = format!("│{}{}{}│", title_left, self.title, title_right);

        write!(stdout, "{}{}{}{}{}",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + 2),
               ::termion::color::Fg(::termion::color::White),
               ::termion::style::Bold,
               title_line,
               ::termion::style::Reset).expect("fail to write to shell");

        // Channel list
        for (i, channel) in self.channels.iter().enumerate() {
            let line_num = i + 3;
            if line_num >= dialog_height - 2 {
                break;
            }

            let item_text = format!("[{:2}] {} ({})", i + 1, channel.title, channel.channel);
            let item_text_width = jks_len(&item_text);
            let item_padding = dialog_width.saturating_sub(item_text_width + 2);
            let item_line = format!("│{}{}│", item_text, " ".repeat(item_padding));

            write!(stdout, "{}{}{}",
                   ::termion::cursor::Goto(dialog_x + 1, dialog_y + line_num as u16),
                   ::termion::color::Fg(::termion::color::White),
                   item_line).expect("fail to write to shell");
        }

        // Bottom-left corner
        write!(stdout, "{}└",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + dialog_height as u16 - 1)).expect("fail to write to shell");

        // Bottom border
        for i in 0..dialog_width as u16 - 2 {
            write!(stdout, "{}─",
                   ::termion::cursor::Goto(dialog_x + 2 + i, dialog_y + dialog_height as u16 - 1)).expect("fail to write to shell");
        }

        // Bottom-right corner
        write!(stdout, "{}┘",
               ::termion::cursor::Goto(dialog_x + dialog_width as u16, dialog_y + dialog_height as u16 - 1)).expect("fail to write to shell");

        // Instructions
        let instructions = "1-9: Select | ESC: Cancel";
        let instructions_width = jks_len(&instructions);
        let instructions_left = " ".repeat((dialog_width.saturating_sub(instructions_width)) / 2);
        write!(stdout, "{}{}{}{}{}",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + dialog_height as u16),
               ::termion::color::Fg(::termion::color::Yellow),
               ::termion::style::Bold,
               instructions_left,
               instructions).expect("fail to write to shell");
    }
}

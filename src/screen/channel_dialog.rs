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
    selected_index: usize,
    scroll_offset: usize,
    items_per_page: usize,
}

impl ChannelDialog {
    pub fn new() -> Self {
        let channels = vec![
            // 推薦
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
            // 科技
            ChannelInfo { title: "硬件台".to_string(), channel: "HW".to_string() },
            ChannelInfo { title: "電訊台".to_string(), channel: "IN".to_string() },
            ChannelInfo { title: "軟件台".to_string(), channel: "SW".to_string() },
            ChannelInfo { title: "手機台".to_string(), channel: "MP".to_string() },
            ChannelInfo { title: "Apps台".to_string(), channel: "AP".to_string() },
            ChannelInfo { title: "Crypto台".to_string(), channel: "blockchain".to_string() },
            ChannelInfo { title: "AI技術台".to_string(), channel: "AI".to_string() },
            // 消閒
            ChannelInfo { title: "遊戲台".to_string(), channel: "GM".to_string() },
            ChannelInfo { title: "飲食台".to_string(), channel: "ED".to_string() },
            ChannelInfo { title: "旅遊台".to_string(), channel: "TR".to_string() },
            ChannelInfo { title: "潮流台".to_string(), channel: "CO".to_string() },
            ChannelInfo { title: "動漫台".to_string(), channel: "AN".to_string() },
            ChannelInfo { title: "玩具台".to_string(), channel: "TO".to_string() },
            ChannelInfo { title: "音樂台".to_string(), channel: "MU".to_string() },
            ChannelInfo { title: "影視台".to_string(), channel: "VI".to_string() },
            ChannelInfo { title: "攝影台".to_string(), channel: "DC".to_string() },
            ChannelInfo { title: "汽車台".to_string(), channel: "TS".to_string() },
            // 生活
            ChannelInfo { title: "上班台".to_string(), channel: "WK".to_string() },
            ChannelInfo { title: "感情台".to_string(), channel: "LV".to_string() },
            ChannelInfo { title: "校園台".to_string(), channel: "SC".to_string() },
            ChannelInfo { title: "親子台".to_string(), channel: "BB".to_string() },
            ChannelInfo { title: "寵物台".to_string(), channel: "PT".to_string() },
            ChannelInfo { title: "健康台".to_string(), channel: "HL".to_string() },
            // 其他
            ChannelInfo { title: "站務台".to_string(), channel: "MB".to_string() },
            ChannelInfo { title: "電　台".to_string(), channel: "RA".to_string() },
            ChannelInfo { title: "活動台".to_string(), channel: "AC".to_string() },
            ChannelInfo { title: "買賣台".to_string(), channel: "BS".to_string() },
            ChannelInfo { title: "直播台".to_string(), channel: "JT".to_string() },
            ChannelInfo { title: "成人台".to_string(), channel: "AU".to_string() },
            ChannelInfo { title: "考古台".to_string(), channel: "OP".to_string() },
        ];

        ChannelDialog {
            title: String::from("Select Channel"),
            channels,
            visible: false,
            selected_index: 0,
            scroll_offset: 0,
            items_per_page: 12, // Show 12 channels at a time
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn move_up(&mut self) {
        if self.selected_index == 0 {
            self.selected_index = self.channels.len() - 1;
            self.scroll_offset = self.channels.len().saturating_sub(self.items_per_page);
        } else {
            self.selected_index -= 1;
            if self.selected_index < self.scroll_offset {
                self.scroll_offset = self.selected_index;
            }
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index >= self.channels.len() - 1 {
            self.selected_index = 0;
            self.scroll_offset = 0;
        } else {
            self.selected_index += 1;
            if self.selected_index >= self.scroll_offset + self.items_per_page {
                self.scroll_offset = self.selected_index - self.items_per_page + 1;
            }
        }
    }

    pub fn get_selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn get_selected_channel(&self) -> Option<&ChannelInfo> {
        self.channels.get(self.selected_index)
    }

    pub fn set_selected_index(&mut self, index: usize) {
        if index < self.channels.len() {
            self.selected_index = index;
        }
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
        let dialog_height = 16;

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

        // Title line with sides - aligned left, show page indicator
        let page_indicator = if self.channels.len() > self.items_per_page {
            let current_page = self.scroll_offset / self.items_per_page + 1;
            let total_pages = (self.channels.len() + self.items_per_page - 1) / self.items_per_page;
            format!(" [{}/{}]", current_page, total_pages)
        } else {
            String::new()
        };
        let title_line = format!("│{}{}{}│", self.title, " ".repeat(dialog_width.saturating_sub(self.title.len() + page_indicator.len() + 2)), page_indicator);

        write!(stdout, "{}{}{}{}{}",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + 2),
               ::termion::color::Fg(::termion::color::White),
               ::termion::style::Bold,
               title_line,
               ::termion::style::Reset).expect("fail to write to shell");

        // Channel list - show only visible channels
        let visible_channels = self.channels.iter()
            .skip(self.scroll_offset)
            .take(self.items_per_page)
            .enumerate();

        for (i, channel) in visible_channels {
            let line_num = i + 3;
            let actual_index = self.scroll_offset + i;
            if line_num >= dialog_height - 1 {
                break;
            }

            let item_text = format!("[{:2}] {} ({})", actual_index + 1, channel.title, channel.channel);
            let item_text_width = jks_len(&item_text);
            let left_padding = "   "; // 3 spaces of left padding
            let item_padding = dialog_width.saturating_sub(item_text_width + 2 + left_padding.len());
            let inner_content = format!("{}{}{}", left_padding, item_text, " ".repeat(item_padding));

            if actual_index == self.selected_index {
                // Highlight selected channel - reduced by 1 on each side
                let highlight_left = &left_padding[1..]; // Skip first space
                let highlight_right = " ".repeat(item_padding.saturating_sub(1)); // One less space
                let highlight_content = format!("{}{}{}", highlight_left, item_text, highlight_right);
                let full_line = format!("│ {}{}{}{} │",
                    ::termion::color::Fg(::termion::color::Black),
                    ::termion::color::Bg(::termion::color::Yellow),
                    highlight_content,
                    ::termion::style::Reset
                );
                write!(stdout, "{}{}",
                       ::termion::cursor::Goto(dialog_x + 1, dialog_y + line_num as u16),
                       full_line).expect("fail to write to shell");
            } else {
                write!(stdout, "{}{}│{}│",
                       ::termion::cursor::Goto(dialog_x + 1, dialog_y + line_num as u16),
                       ::termion::color::Fg(::termion::color::White),
                       inner_content).expect("fail to write to shell");
            }
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

        // Instructions below bottom border
        let instructions = format!("↑↓: Navigate | Enter: Select | ESC: Cancel");
        write!(stdout, "{}{}{}{}{}",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + dialog_height as u16),
               ::termion::color::Fg(::termion::color::Yellow),
               ::termion::style::Bold,
               instructions,
               " ".repeat(dialog_width.saturating_sub(jks_len(&instructions)))).expect("fail to write to shell");
    }
}

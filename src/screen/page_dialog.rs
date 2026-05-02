use std::io::Write;

pub struct PageDialog {
    title: String,
    prompt: String,
    input: String,
    visible: bool,
    min_page: usize,
    max_page: usize,
}

impl PageDialog {
    pub fn new() -> Self {
        PageDialog {
            title: String::from("Go to Page"),
            prompt: String::from("Enter page number:"),
            input: String::new(),
            visible: false,
            min_page: 1,
            max_page: 1,
        }
    }

    pub fn show(&mut self, current_page: usize, max_page: usize) {
        self.visible = true;
        self.input.clear();
        self.min_page = 1;
        self.max_page = max_page;

        // Pre-fill with current page for convenience
        self.input = current_page.to_string();
    }

    pub fn hide(&mut self) {
        self.visible = false;
        self.input.clear();
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn add_char(&mut self, c: char) {
        // Only allow numeric input
        if c.is_ascii_digit() {
            // Limit input length to prevent overflow
            if self.input.len() < 10 {
                self.input.push(c);
            }
        }
    }

    pub fn backspace(&mut self) {
        self.input.pop();
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    /// Validate and get the page number
    ///
    /// Returns None if input is empty or not a valid number
    /// Returns Some(page) if input is valid, clamped to [min_page, max_page]
    pub fn get_valid_page(&self) -> Option<usize> {
        if self.input.is_empty() {
            return None;
        }

        self.input.parse::<usize>().ok().map(|page| {
            // Clamp to valid range
            page.clamp(self.min_page, self.max_page)
        })
    }

    pub fn clear(&mut self) {
        self.input.clear();
    }

    pub fn print(&mut self, stdout: &mut ::termion::raw::RawTerminal<std::io::StdoutLock>) {
        let width = ::termion::terminal_size().expect("fail to get terminal size").0 as usize;
        let height = ::termion::terminal_size().expect("fail to get terminal size").1 as usize;

        let dialog_width = 40.min(width - 4);
        let dialog_height = 7;

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
        let title_padding = dialog_width.saturating_sub(self.title.len() + 2);
        let title_left = " ".repeat(title_padding / 2);
        let title_right = " ".repeat(title_padding - title_padding / 2);
        let title_line = format!("│{}{}{}│", title_left, self.title, title_right);

        write!(stdout, "{}{}{}{}{}",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + 2),
               ::termion::color::Fg(::termion::color::White),
               ::termion::style::Bold,
               title_line,
               ::termion::style::Reset).expect("fail to write to shell");

        // Page range hint
        let range_hint = format!("Page: 1-{}", self.max_page);
        let range_padding = dialog_width.saturating_sub(range_hint.len() + 2);
        let range_line = format!("│{}{}│", " ".repeat(range_padding / 2), range_hint);
        let range_line_full = format!("{}{}{}",
                                       range_line,
                                       " ".repeat(dialog_width.saturating_sub(range_line.len())),
                                       "│");

        write!(stdout, "{}{}{}",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + 3),
               ::termion::color::Fg(::termion::color::Cyan),
               range_line_full).expect("fail to write to shell");

        // Input line with sides
        let input_text = format!("{} {}", self.prompt, self.input);
        let input_display = if input_text.len() > dialog_width - 2 {
            format!("...{}", &input_text[input_text.len() - dialog_width + 5..])
        } else {
            input_text.clone()
        };
        let input_padding = dialog_width.saturating_sub(input_display.len() + 2);
        let input_line = format!("│{}{}{}│",
                                 ::termion::style::Bold,
                                 format!("{}{}", input_display, " ".repeat(input_padding)),
                                 ::termion::style::Reset);

        write!(stdout, "{}{}{}{}",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + 4),
               ::termion::color::Fg(::termion::color::White),
               input_line,
               ::termion::style::Reset).expect("fail to write to shell");

        // Bottom-left corner
        write!(stdout, "{}└",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + 5)).expect("fail to write to shell");

        // Bottom border
        for i in 0..dialog_width as u16 - 2 {
            write!(stdout, "{}─",
                   ::termion::cursor::Goto(dialog_x + 2 + i, dialog_y + 5)).expect("fail to write to shell");
        }

        // Bottom-right corner
        write!(stdout, "{}┘",
               ::termion::cursor::Goto(dialog_x + dialog_width as u16, dialog_y + 5)).expect("fail to write to shell");

        // Instructions
        let instructions = "ENTER: Go | ESC: Cancel";
        write!(stdout, "{}{}{}{}",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + 6),
               ::termion::color::Fg(::termion::color::Yellow),
               ::termion::style::Bold,
               instructions).expect("fail to write to shell");
    }
}

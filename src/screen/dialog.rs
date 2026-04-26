use std::io::Write;
use std;

pub struct Dialog {
    title: String,
    prompt: String,
    input: String,
    visible: bool,
}

impl Dialog {
    pub fn new() -> Self {
        Dialog {
            title: String::from("Open Thread by ID"),
            prompt: String::from("Enter thread ID:"),
            input: String::new(),
            visible: false,
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
        self.input.clear();
    }

    pub fn hide(&mut self) {
        self.visible = false;
        self.input.clear();
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn add_char(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn backspace(&mut self) {
        self.input.pop();
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn clear(&mut self) {
        self.input.clear();
    }

    pub fn print(&mut self, stdout: &mut ::termion::raw::RawTerminal<std::io::StdoutLock>) {
        let width = ::termion::terminal_size().expect("fail to get terminal size").0 as usize;
        let height = ::termion::terminal_size().expect("fail to get terminal size").1 as usize;

        let dialog_width = 40.min(width - 4);
        let dialog_height = 6;

        let dialog_x = ((width - dialog_width) / 2) as u16;
        let dialog_y = ((height - dialog_height) / 2) as u16;

        // Clear dialog area
        for y in dialog_y..dialog_y + dialog_height as u16 {
            write!(stdout, "{}{}",
                   ::termion::cursor::Goto(1, y + 1),
                   " ".repeat(width)).expect("fail to write to shell");
        }

        // Draw dialog border
        let border = "─".repeat(dialog_width);
        let side = "│";

        // Top border
        write!(stdout, "{}{}┌{}┐{}",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + 1),
               ::termion::color::Fg(::termion::color::White),
               border,
               ::termion::style::Reset).expect("fail to write to shell");

        // Title line
        let title_padding = dialog_width.saturating_sub(self.title.len() + 2);
        let title_left = " ".repeat(title_padding / 2);
        let title_right = " ".repeat(title_padding - title_padding / 2);
        write!(stdout, "{}{}{}{}{}{}{}{}",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + 2),
               ::termion::color::Fg(::termion::color::White),
               side,
               ::termion::style::Bold,
               format!("{}{}{}", title_left, self.title, title_right),
               ::termion::style::Reset,
               side,
               ::termion::cursor::Hide).expect("fail to write to shell");

        // Empty line
        write!(stdout, "{}{}{}{}{}",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + 3),
               ::termion::color::Fg(::termion::color::White),
               side,
               " ".repeat(dialog_width),
               side).expect("fail to write to shell");

        // Prompt line with input
        let input_text = format!("{} {}", self.prompt, self.input);
        let input_display = if input_text.len() > dialog_width - 2 {
            format!("...{}", &input_text[input_text.len() - dialog_width + 5..])
        } else {
            input_text.clone()
        };
        let input_padding = dialog_width.saturating_sub(input_display.len() + 2);
        write!(stdout, "{}{}{}{}{}{}{}{}",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + 4),
               ::termion::color::Fg(::termion::color::White),
               side,
               ::termion::style::Bold,
               format!("{}{}", input_display, " ".repeat(input_padding)),
               ::termion::style::Reset,
               side,
               ::termion::cursor::Hide).expect("fail to write to shell");

        // Bottom border
        write!(stdout, "{}{}└{}┘{}",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + 5),
               ::termion::color::Fg(::termion::color::White),
               border,
               ::termion::style::Reset).expect("fail to write to shell");

        // Instructions line
        let instructions = "ENTER: Open | ESC: Cancel";
        write!(stdout, "{}{}{}{}{}",
               ::termion::cursor::Goto(dialog_x + 1, dialog_y + 6),
               ::termion::color::Fg(::termion::color::Yellow),
               ::termion::style::Bold,
               instructions,
               ::termion::style::Reset).expect("fail to write to shell");
    }
}

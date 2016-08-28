use termion::terminal_size;

pub struct ScreenManager {
    current_width: usize,
    prev_width: usize
}

impl ScreenManager {
    pub fn new () -> Self {
        ScreenManager {
            current_width: 0,
            prev_width: 0,
        }
    }
    pub fn updateWidth(&mut self) -> usize {
        let w = terminal_size().unwrap().0 as usize;
        self.prev_width = self.current_width;
        self.current_width = w;
        w
    }
    pub fn isWidthChanged(&self) -> bool {
        self.current_width != self.prev_width
    }
}

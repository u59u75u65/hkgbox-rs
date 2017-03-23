use termion::terminal_size;

pub struct ScreenManager {
    current_width: usize,
    prev_width: usize
}

impl ScreenManager {
    pub fn new () -> Self {
        let w = terminal_size().expect("fail to get terminal size").0 as usize;
        ScreenManager {
            current_width: w,
            prev_width: w,
        }
    }
    pub fn getWidth(&self) -> usize {
        self.current_width
    }
    pub fn isWidthChanged(&mut self) -> bool {
        let w = terminal_size().expect("fail to get terminal size").0 as usize;
        if self.current_width != w {
            self.prev_width = self.current_width;
            self.current_width = w;
            return true
        }
        return false
    }
    pub fn getHeight(&self) -> usize {
        terminal_size().expect("fail to get terminal size").1 as usize
    }
}

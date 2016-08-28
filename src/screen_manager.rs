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
    pub fn isWidthChanged(&mut self) -> bool {
        let w = terminal_size().unwrap().0 as usize;
        if self.current_width != w {
            self.prev_width = self.current_width;
            self.current_width = w;
            return true
        }
        return false
    }    
}

use termion::terminal_size;

pub struct ScreenManager {
    current_width: usize,
    prev_width: usize,
    current_height: usize,
    prev_height: usize
}

impl ScreenManager {

    pub fn new () -> Self {
        let w = terminal_size().expect("fail to get terminal size").0 as usize;
        let h = terminal_size().expect("fail to get terminal size").1 as usize;

        ScreenManager {
            current_width: w,
            prev_width: w,
            current_height: h,
            prev_height: h
        }
    }

    pub fn get_width(&self) -> usize {
        self.current_width
    }
    pub fn get_height(&self) -> usize {
        self.current_height
    }

    pub fn is_width_changed(&mut self) -> bool {
        let w = terminal_size().expect("fail to get terminal size").0 as usize;
        if self.current_width != w {
            self.prev_width = self.current_width;
            self.current_width = w;
            return true
        }
        return false
    }

    pub fn is_height_changed(&mut self) -> bool {
        let h = terminal_size().expect("fail to get terminal size").1 as usize;
        if self.current_height != h {
            self.prev_height = self.current_height;
            self.current_height = h;
            return true
        }
        return false
    }

}

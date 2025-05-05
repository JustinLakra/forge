use crossterm::{cursor::MoveTo, execute};
use std::io::stdout;

pub struct Cursor {
    //using u16 since we only need to store positive integers
    x: u16,
    y: u16,
}
impl Cursor {
    pub fn default() -> Self {
        Cursor { x: 0, y: 0 }
    }
    pub fn move_left(&mut self) {
        if self.x != 0 {
            self.x -= 1;
            self.move_crossterm_cursor();
        }
    }
    pub fn move_right(&mut self) {
        self.x += 1;
        self.move_crossterm_cursor();
    }
    pub fn move_up(&mut self) {
        if self.y != 0 {
            self.y -= 1;
            self.move_crossterm_cursor();
        }
    }
    pub fn move_down(&mut self) {
        self.y += 1;
        self.move_crossterm_cursor();
    }
    pub fn move_crossterm_cursor(&mut self) {
        execute!(stdout(), MoveTo(self.x, self.y)).ok();
    }
}

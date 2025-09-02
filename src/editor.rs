use crate::cursor::Cursor;
use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event::Key, KeyCode::Backspace, KeyCode::Char};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdout, Write};

#[derive(PartialEq, Eq)]
pub enum Mode {
    Normal,
    Editing,
}

pub struct Editor {
    pub mode: Mode,
    pub buffer: Vec<char>,
    cursor: Cursor,
}
//NOTE fix the imports
impl Editor {
    pub fn default() -> Self {
        Editor {
            mode: Mode::Normal,
            buffer: Vec::new(),
            cursor: Cursor::default(),
        }
    }
    pub fn run(&mut self) {
        match read() {
            Ok(Key(event)) => {
                if let Char(c) = event.code {
                    if self.mode == Mode::Normal {
                        if c == 'q' {
                            return;
                        }
                        if c == 'i' {
                            self.mode = Mode::Editing;
                        }
                        //TODO use custom keybindings. create another directory and move these
                        //into a new function
                        if c == 'w' {
                            self.cursor.move_up();
                        }
                        if c == 'a' {
                            self.cursor.move_left();
                        }
                        if c == 's' {
                            self.cursor.move_down();
                        }
                        if c == 'd' {
                            self.cursor.move_right();
                        }
                    }
                    // condition for normal mode
                    else if self.mode == Mode::Editing {
                        //TODO implement functionality for insert mode(exiting insert mode)
                        if c == 'q' {
                            self.mode = Mode::Normal;
                        } else {
                            self.insert_char(c);
                        }
                    }
                } else if let Backspace = event.code {
                    if self.mode == Mode::Editing {
                        self.buffer.pop();
                    }
                }
            }
            Err(err) => println!("Error: {err}"),
            _ => (),
        }
    }
    pub fn insert_char(&mut self, c: char) {
        self.buffer.push(c);
        self.cursor.move_right();
    }
}

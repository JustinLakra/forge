use crate::cursor::Cursor;
use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event::Key, KeyCode::Backspace, KeyCode::Char};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdout, Write};

#[derive(PartialEq, Eq)]
enum Mode {
    Normal,
    Editing,
}

pub struct Editor {
    mode: Mode,
    buffer: Vec<char>,
    cursor: Cursor,
}
//TODO make a display function to first clear the terminal screen and then display the buffer
//elements
//NOTE fix the imports
impl Editor {
    pub fn default() -> Self {
        Editor {
            mode: Mode::Normal, //0 for normal, 1 for visual, 2 for insert
            buffer: Vec::new(),
            cursor: Cursor::default(),
        }
    }
    pub fn run(&mut self) {
        enable_raw_mode().unwrap();
        execute!(stdout(), Clear(ClearType::All)).ok();
        execute!(stdout(), MoveTo(0, 0)).ok();
        loop {
            match read() {
                Ok(Key(event)) => {
                    if let Char(c) = event.code {
                        if self.mode == Mode::Normal {
                            if c == 'q' {
                                break;
                            }
                            if c == 'i' {
                                self.mode = Mode::Editing;
                            }
                            //TODO use custom keybindings. create another directory
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
                    //TODO Move this into another function which is specifically built for display
                    execute!(stdout(), Clear(ClearType::All)).ok();
                    let line: String = self.buffer.iter().collect();
                    execute!(stdout(), Print(line)).ok();
                    //execute!(stdout(), Print("\r")).ok();
                    //println!("{:?}\r", self.buffer);
                    //execute!(stdout(), MoveTo(0, 0)).ok();
                }
                Err(err) => println!("Error: {err}"),
                _ => (),
            }
        }
        disable_raw_mode().unwrap();
    }
    pub fn insert_char(&mut self, c: char) {
        self.buffer.push(c);
        self.cursor.move_right();
    }
}

use crate::cursor::Cursor;
use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event::Key, KeyCode::Backspace, KeyCode::Char};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::env;
use std::fs;
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
    pub file_path: String,
}
//NOTE fix the imports
impl Editor {
    pub fn default() -> Self {
        Editor {
            mode: Mode::Normal,
            buffer: Vec::new(),
            cursor: Cursor::default(),
            file_path: String::from(""),
        }
    }
    pub fn run(&mut self) {
        self.read_file();
        match read() {
            Ok(Key(event)) => {
                if let Char(c) = event.code {
                    if self.mode == Mode::Normal {
                        if c == 'q' {
                            //TODO set flag and handle exit in main.rs
                            std::process::exit(0);
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
    pub fn get_line_from_buffer(&self) -> String {
        return self.buffer.iter().collect();
    }
    fn get_file_path(&mut self) {
        let args: Vec<String> = env::args().collect();
        //cloning because I need this
        self.file_path = args[1].clone();
    }
    pub fn read_file(&mut self) {
        self.get_file_path();
        let content = fs::read_to_string(&self.file_path).unwrap();
        self.buffer = content.chars().collect();
    }
}

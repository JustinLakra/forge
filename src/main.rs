#![warn(clippy::all, clippy::pedantic)]
mod buffer;
mod cursor;
mod editor;
use crate::editor::Mode;
use editor::Editor;
use ratatui::{
    layout::{Constraint, Layout, Position},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

pub struct App {
    editor: Editor,
}

impl App {
    pub fn default() -> Self {
        App {
            editor: Editor::default(),
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            self.editor.run();
        }
    }

    fn draw(&self, frame: &mut Frame) {
        //let layout = Layout::default();
        //use frame.area() to use the whole terminal
        let line: String = self.editor.buffer.iter().collect();
        let output = match self.editor.mode {
            Mode::Normal => "Normal".to_string(),
            Mode::Editing => "Editing".to_string(),
        };
        frame.render_widget(Paragraph::new(line), frame.area())
    }
}

#![warn(clippy::all, clippy::pedantic)]
mod buffer;
mod cursor;
mod editor;
use crate::editor::Mode;
use editor::Editor;
use ratatui::{
    layout::{Constraint, Direction, Layout, Position},
    style::{Color, Style, Stylize},
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
        let _ = terminal.clear();
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            self.editor.run();
        }
    }

    fn draw(&self, frame: &mut Frame) {
        //use frame.area() to use the whole terminal
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(5),
                Constraint::Percentage(90),
                Constraint::Percentage(5),
            ])
            .split(frame.area());
        let line: String = self.editor.get_line_from_buffer();
        let mode = match self.editor.mode {
            Mode::Normal => "Normal".to_string(),
            Mode::Editing => "Editing".to_string(),
        };
        let file_path = self.editor.file_path.clone();
        frame.render_widget(Paragraph::new(file_path).centered(), layout[0]);
        frame.render_widget(Paragraph::new(line).block(Block::bordered()), layout[1]);
        frame.render_widget(Paragraph::new(mode), layout[2]);
    }
}

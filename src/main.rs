#![warn(clippy::all, clippy::pedantic)]
mod buffer;
mod cursor;
mod editor;
use editor::Editor;
fn main() {
    let mut editor = Editor::default();
    editor.run();
}

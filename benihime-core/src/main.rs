use editor::Editor;

mod application;
mod buffer;
mod buffer_manager;
mod chars;
mod command;
mod editor;
mod editor_state;
mod keymap;
mod macros;
mod mini_buffer;
mod movement;
mod position;
mod project;
mod render;
mod theme;
mod ui;
mod undotree;

fn main() {
    let mut editor = Editor::new();
    let _ = editor.run();
}

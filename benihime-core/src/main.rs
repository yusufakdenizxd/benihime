use editor::Editor;

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
mod theme;
mod undotree;

fn main() {
    let mut editor = Editor::new();
}

use application::Application;

mod application;
mod buffer;
mod buffer_manager;
mod chars;
mod command;
mod editor;
mod graphics;
mod keymap;
mod macros;
mod mini_buffer;
mod movement;
mod position;
mod project;
mod theme;
mod ui;
mod undotree;

fn main() {
    let mut editor = Application::new();
}

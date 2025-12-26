use render::gui_render;

mod buffer;
mod buffer_manager;
mod chars;
mod command;
mod editor;
mod keymap;
mod macros;
mod mini_buffer;
mod movement;
mod position;
mod render;
mod themes;

fn main() {
    let _ = gui_render::run();
}

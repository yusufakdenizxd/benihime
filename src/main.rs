use render::gui_render;

mod buffer;
mod buffer_manager;
mod command;
mod editor;
mod keymap;
mod mini_buffer;
mod movement;
mod position;
mod render;

fn main() {
    let _ = gui_render::run();
}

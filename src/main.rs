mod buffer;
mod buffer_manager;
mod command;
mod editor;
mod gui_render;
mod keymap;

fn main() {
    let _ = gui_render::run();
}

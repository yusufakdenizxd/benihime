mod buffer;
mod buffer_manager;
mod command;
mod editor;
mod keymap;
mod tui_render;

fn main() {
    let _ = tui_render::run();
}

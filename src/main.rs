mod buffer;
mod commands;
mod editor;
mod keymap;
mod tui_render;

fn main() {
    let _ = tui_render::run();
}

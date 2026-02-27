use application::Application;

mod application;
mod buffer;
mod buffer_manager;
mod chars;
mod command;
mod editor;
mod graphics;
mod input_handler;
mod keymap;
mod macros;
mod mini_buffer;
mod movement;
mod position;
mod project;
mod theme;
mod ui;
mod undotree;

fn main() -> anyhow::Result<()> {
    let window_config = benihime_renderer::WindowConfig::new("The Editor", false);

    let app = Application::new();

    

    benihime_renderer::run(window_config, app)
        .map_err(|e| anyhow::anyhow!("Failed to run renderer: {}", e))
}

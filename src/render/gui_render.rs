use crate::buffer::Mode;
use crate::editor::Editor;
use crate::keymap::key_chord::{KeyCode, KeyModifiers};
use eframe::egui;

use egui::Key;

use super::buffer::render_buffer;
use super::bufferline::render_bufferline;
use super::minibuffer::render_minibuffer;
use super::statusline::render_statusline;

pub struct EditorApp {
    pub editor: Editor,
}

impl EditorApp {
    pub fn new() -> Self {
        let editor = Editor::new();
        Self { editor }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            for event in &i.events {
                if let egui::Event::Key {
                    key,
                    pressed,
                    modifiers,
                    ..
                } = event
                {
                    let mut pressed = pressed;
                    //EGUI doesnt give pressed true when it triggers cut copy or paste
                    if modifiers.command_only()
                        && (*key == Key::X || *key == Key::C || *key == Key::V)
                    {
                        pressed = &true;
                    }

                    if *pressed {
                        let key_code = KeyCode::from_egui(*key);
                        let key_modifiers = KeyModifiers::from_egui(*modifiers);
                        self.editor.handle_key(key_code, key_modifiers);
                    }
                }
            }
        });

        let state = &mut self.editor.state.lock().unwrap();

        render_bufferline(ctx, state.buffer_line());

        render_statusline(
            ctx,
            state.status_line(),
            state.focused_buf().mode,
            &state.command_buffer,
            &state.message,
            &state.error_message,
        );

        let buf = state.focused_buf_mut();
        if buf.mode == Mode::Minibuffer {
            let minibuffer = &state.minibuffer_manager.current.as_ref().unwrap();

            render_minibuffer(ctx, minibuffer);
        }

        render_buffer(ctx, state);
    }
}

pub fn run() -> eframe::Result<()> {
    eframe::run_native(
        "Benihime Editor",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(EditorApp::new()))),
    )
}

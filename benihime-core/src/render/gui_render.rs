use crate::editor::Editor;
use crate::keymap::key_chord::{KeyCode, KeyModifiers};
use eframe::egui;

use egui::Key;

use super::buffer::render_buffer;
use super::bufferline::render_bufferline;
use super::minibuffer::render_minibuffer;
use super::statusline::render_statusline;

impl eframe::App for &mut Editor {
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
                        self.handle_key(key_code, key_modifiers);
                    }
                }
            }
        });

        let state = &mut self.state.lock().unwrap();

        render_bufferline(ctx, state);

        render_statusline(ctx, state);

        render_minibuffer(ctx, state);

        render_buffer(ctx, state);
    }
}

use crate::editor::Editor;
use eframe::egui;

use super::buffer::render_buffer;
use super::bufferline::render_bufferline;
use super::minibuffer::render_minibuffer;
use super::statusline::render_statusline;

impl eframe::App for &mut Editor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let state = &mut self.state.lock().unwrap();

        render_bufferline(ctx, state);

        render_statusline(ctx, state);

        render_minibuffer(ctx, state);

        render_buffer(ctx, state);
    }
}

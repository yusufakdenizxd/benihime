use egui::{Context, RichText};

use crate::{buffer::Mode, editor_state::EditorState, mini_buffer::MiniBufferTrait};

pub fn render_minibuffer(ctx: &Context, state: &EditorState) {
    let buf = state.focused_buf();
    if buf.mode != Mode::Minibuffer {
        return;
    }
    let minibuffer = &state.minibuffer_manager.current.as_ref().unwrap();

    let max_count = 10;
    let offset = minibuffer.offset();
    let index = minibuffer.index();
    let len = minibuffer.len();

    let end = (offset + max_count).min(len);

    egui::TopBottomPanel::bottom("minibuffer").show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.label(format!(
                "({}/{}) {} {}",
                index,
                len,
                minibuffer.prompt(),
                minibuffer.input(),
            ));
            for (i, c) in minibuffer.render_candidates()[offset..end]
                .iter()
                .enumerate()
            {
                if (index - offset) == i {
                    ui.label(RichText::new(c).underline().strong());
                } else {
                    ui.label(c);
                }
            }
        });
    });
}

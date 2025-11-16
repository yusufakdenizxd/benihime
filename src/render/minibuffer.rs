use egui::{Context, RichText};

use crate::mini_buffer::MiniBufferTrait;

pub fn render_minibuffer(ctx: &Context, minibuffer: &&Box<dyn MiniBufferTrait + Send>) {
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

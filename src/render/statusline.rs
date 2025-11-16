use egui::{Align, Context, Layout};

use crate::buffer::Mode;

pub fn render_statusline(
    ctx: &Context,
    statusline: String,
    mode: Mode,
    command_buffer: &String,
    message: &Option<String>,
    error_message: &Option<String>,
) {
    egui::TopBottomPanel::bottom("statusline").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(statusline);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if mode == Mode::Command {
                    ui.label(format!("{}:", command_buffer));
                } else if error_message.is_some() {
                    ui.colored_label(egui::Color32::RED, error_message.clone().unwrap());
                } else if message.is_some() {
                    ui.label(message.clone().unwrap());
                } else {
                    ui.label("");
                }
            });
        });
    });
}

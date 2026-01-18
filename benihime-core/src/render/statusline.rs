use egui::{Align, Context, Layout};

use crate::{buffer::Mode, editor_state::EditorState};

pub fn render_statusline(ctx: &Context, state: &EditorState) {
    egui::TopBottomPanel::bottom("statusline").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(state.status_line());
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if let Some(name) = state.project_manager.current_name() {
                    ui.label(name);
                }
                if state.focused_buf().mode == Mode::Command {
                    ui.label(format!("{}:", state.command_buffer));
                } else if state.error_message.is_some() {
                    ui.colored_label(egui::Color32::RED, state.error_message.clone().unwrap());
                } else if state.message.is_some() {
                    ui.label(state.message.clone().unwrap());
                }
            });
        });
    });
}

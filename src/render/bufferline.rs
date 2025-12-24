use egui::{Color32, Context};

pub fn render_bufferline(ctx: &Context, buffer_line: Vec<(i32, String, bool)>) {
    egui::TopBottomPanel::top("bufferline")
        .frame(egui::Frame {
            inner_margin: egui::Margin::ZERO,
            outer_margin: egui::Margin::ZERO,
            ..Default::default()
        })
        .show_separator_line(false)
        .exact_height(30.0)
        .show(ctx, |ui| {
            let text_color = ui.visuals().text_color();
            let strong_text = ui.visuals().strong_text_color();

            ui.painter()
                .rect_filled(ui.max_rect(), 0.0, Color32::from_gray(30));

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;

                for (_, name, active) in buffer_line {
                    let padding_x = 14.0;
                    let tab_height = 30.0;

                    let text_width = ui.fonts(|f| {
                        f.layout_no_wrap(name.clone(), egui::FontId::default(), text_color)
                            .size()
                            .x
                    });

                    let tab_width = text_width + padding_x * 2.0;

                    let rect = ui.allocate_space(egui::vec2(tab_width, tab_height)).1;

                    let tab_bg = if active {
                        Color32::from_rgba_premultiplied(60, 60, 60, 255)
                    } else {
                        Color32::from_rgba_premultiplied(40, 40, 40, 255)
                    };

                    ui.painter().rect_filled(rect, 0.0, tab_bg);

                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        name,
                        egui::FontId::default(),
                        if active { strong_text } else { text_color },
                    );
                }
            });
        });
}

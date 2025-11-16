use egui::{Context, RichText};

pub fn render_bufferline(ctx: &Context, buffer_line: Vec<(i32, String, bool)>) {
    egui::TopBottomPanel::top("bufferline")
        .show_separator_line(false)
        .show(ctx, |ui| {
            egui::Frame::NONE
                .inner_margin(egui::Margin {
                    top: 4,
                    ..Default::default()
                })
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for (_, name, active) in buffer_line {
                            let frame = egui::Frame::group(ui.style())
                                .corner_radius(egui::CornerRadius {
                                    nw: 4,
                                    ne: 4,
                                    sw: 0,
                                    se: 0,
                                })
                                .inner_margin(egui::Margin::symmetric(8, 4));

                            let tab_color = if active {
                                ui.visuals().selection.bg_fill
                            } else {
                                ui.visuals().widgets.inactive.bg_fill
                            };

                            let text_color = if active {
                                ui.visuals().strong_text_color()
                            } else {
                                ui.visuals().text_color()
                            };

                            frame.fill(tab_color).show(ui, |ui| {
                                let text = RichText::new(name).color(text_color);
                                ui.add(egui::Label::new(text).sense(egui::Sense::click()));
                            });
                        }
                    });
                });
        });
}

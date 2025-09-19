use crate::editor::{Editor, HandleKeyError};
use eframe::egui;

use egui::{Pos2, Rect};

use egui::Color32;

use egui::FontId;

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
        let state = &mut self.editor.state.lock().unwrap();

        ctx.input(|i| {
            for event in &i.events {
                if let egui::Event::Key {
                    key,
                    pressed,
                    modifiers,
                    ..
                } = event
                {
                    if *pressed {
                        //TODO: Custom Key Parse and then Handle
                        let executed = self.editor.handle_key(state, *key, *modifiers);
                        if executed.is_err_and(|e| e == HandleKeyError::KeyNotFound) {
                            let buf = state.focused_buf_mut();
                            match buf.mode {
                                crate::buffer::Mode::Insert => {
                                    buf.insert_str(key.name());
                                }
                                crate::buffer::Mode::Command => {}
                                _ => {}
                            }
                        }
                    }
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let buf = state.focused_buf();
            let font_size = 16.0;
            let font_id = FontId::monospace(font_size);

            let char_width = ui.fonts(|f| f.glyph_width(&font_id, 'W'));
            let char_height = ui.fonts(|f| f.row_height(&font_id));

            let text_rect = ui
                .allocate_space(egui::vec2(
                    char_width * buf.lines.iter().map(|l| l.len()).max().unwrap_or(1) as f32,
                    char_height * buf.lines.len() as f32,
                ))
                .1;

            for (row, line) in buf.lines.iter().enumerate() {
                for (col, ch) in line.chars().enumerate() {
                    let pos = Pos2 {
                        x: text_rect.min.x + col as f32 * char_width,
                        y: text_rect.min.y + row as f32 * char_height,
                    };

                    ui.painter().text(
                        pos,
                        egui::Align2::LEFT_TOP,
                        ch.to_string(),
                        font_id.clone(),
                        Color32::WHITE,
                    );
                }
            }

            let cursor_row = buf.cursor.row.min(buf.lines.len().saturating_sub(1));
            let cursor_col = buf.cursor.col.min(buf.lines[cursor_row].len());

            let cursor_pos = Pos2 {
                x: text_rect.min.x + cursor_col as f32 * char_width,
                y: text_rect.min.y + cursor_row as f32 * char_height,
            };
            match buf.mode {
                crate::buffer::Mode::Normal => {
                    ui.painter().rect_filled(
                        Rect::from_min_size(cursor_pos, egui::vec2(char_width, char_height)),
                        0.0,
                        Color32::WHITE,
                    );
                }
                crate::buffer::Mode::Insert => {
                    ui.painter().rect_filled(
                        Rect::from_min_size(cursor_pos, egui::vec2(2.0, char_height)),
                        0.0,
                        Color32::LIGHT_BLUE,
                    );
                }
                _ => {}
            }
        });

        egui::TopBottomPanel::bottom("statusline").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(state.status_line());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!(":{}", state.command_buffer));
                });
            });

            if let Some(msg) = &state.message {
                ui.colored_label(egui::Color32::YELLOW, msg);
            }
        });
    }
}

pub fn run() -> eframe::Result<()> {
    eframe::run_native(
        "Benihime Editor",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(EditorApp::new()))),
    )
}

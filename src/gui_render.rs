use crate::buffer::Mode;
use crate::editor::Editor;
use crate::keymap::key_chord::{KeyCode, KeyModifiers};
use eframe::egui;

use egui::{Align, Key, Layout, Pos2, Rect, RichText};

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

        egui::CentralPanel::default().show(ctx, |ui| {
            let font_size = 16.0;

            let font_id = FontId::monospace(font_size);

            let char_width = ui.fonts(|f| f.glyph_width(&font_id, 'W'));
            let char_height = ui.fonts(|f| f.row_height(&font_id));

            let screen_height = (ui.available_height() / char_height) as usize;

            state.screen_height = screen_height;
            let buf = state.focused_buf_mut();
            buf.update_scroll(screen_height, 8);

            let text_rect = ui
                .allocate_space(egui::vec2(
                    char_width * buf.lines.iter().map(|l| l.len()).max().unwrap_or(1) as f32,
                    char_height * buf.lines.len() as f32,
                ))
                .1;

            let start = buf.scroll_offset;
            let end = (start + screen_height).min(buf.lines.len());

            for (row, line) in buf.lines[start..end].iter().enumerate() {
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

            let screen_row = cursor_row.saturating_sub(buf.scroll_offset);

            let cursor_pos = Pos2 {
                x: text_rect.min.x + cursor_col as f32 * char_width,
                y: text_rect.min.y + screen_row as f32 * char_height,
            };
            match buf.mode {
                Mode::Normal => {
                    ui.painter().rect_filled(
                        Rect::from_min_size(cursor_pos, egui::vec2(char_width, char_height)),
                        0.0,
                        Color32::WHITE,
                    );
                }
                Mode::Insert => {
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
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if state.focused_buf().mode == Mode::Command {
                        ui.label(format!("{}:", state.command_buffer));
                    } else if state.error_message.is_some() {
                        ui.colored_label(egui::Color32::RED, state.error_message.clone().unwrap());
                    } else if state.message.is_some() {
                        ui.label(state.message.clone().unwrap());
                    } else {
                        ui.label("");
                    }
                });
            });
        });

        let buf = state.focused_buf_mut();
        if buf.mode == Mode::Minibuffer {
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
                            ui.label(RichText::new(c).underline());
                        } else {
                            ui.label(c);
                        }
                    }
                });
            });
        }
    }
}

pub fn run() -> eframe::Result<()> {
    eframe::run_native(
        "Benihime Editor",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(EditorApp::new()))),
    )
}

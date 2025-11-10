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
        let buffer_line_height = 10.0;
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
            ui.add_space(buffer_line_height + 5.0);
            let font_size = 16.0;
            let font_id = FontId::monospace(font_size);

            let char_width = ui.fonts(|f| f.glyph_width(&font_id, 'W'));
            let char_height = ui.fonts(|f| f.row_height(&font_id));

            let screen_height = (ui.available_height() / char_height) as usize;

            state.screen_height = screen_height;
            let buf = state.focused_buf_mut();
            buf.update_scroll(screen_height, 8);

            let gutter_width = ((buf.lines.len() as f32).log10().ceil() as usize).max(2) + 2;
            let gutter_padding = 1.0;

            let text_rect = ui
                .allocate_space(egui::vec2(
                    (gutter_width as f32 + gutter_padding) * char_width
                        + char_width * buf.lines.iter().map(|l| l.len()).max().unwrap_or(1) as f32,
                    char_height * buf.lines.len() as f32,
                ))
                .1;

            ui.painter().rect_filled(
                Rect::from_min_size(
                    text_rect.min,
                    egui::vec2(
                        (gutter_width as f32 + gutter_padding) * char_width,
                        text_rect.height(),
                    ),
                ),
                0.0,
                Color32::from_gray(30),
            );

            let start = buf.scroll_offset;
            let end = (start + screen_height).min(buf.lines.len());

            for (row, line) in buf.lines[start..end].iter().enumerate() {
                let y = text_rect.min.y + row as f32 * char_height;
                let line_number = start + row + 1;

                let line_number_color = if buf.cursor.row == start + row {
                    Color32::LIGHT_BLUE
                } else {
                    Color32::GRAY
                };

                let line_number_text = format!("{:>width$}", line_number, width = gutter_width);
                //TODO: Relative Number
                ui.painter().text(
                    Pos2 {
                        x: text_rect.min.x,
                        y,
                    },
                    egui::Align2::LEFT_TOP,
                    line_number_text,
                    font_id.clone(),
                    line_number_color,
                );

                if buf.mode == Mode::Visual {
                    if let Some(selection) = &buf.selection {
                        let (start_cursor, end_cursor) = selection.normalized(&buf.cursor);
                        let current_row_abs = start + row;

                        if current_row_abs >= start_cursor.row && current_row_abs <= end_cursor.row
                        {
                            let start_col = if current_row_abs == start_cursor.row {
                                start_cursor.col
                            } else {
                                0
                            };
                            let end_col = if current_row_abs == end_cursor.row {
                                end_cursor.col + 1
                            } else {
                                line.len()
                            };

                            if start_col < end_col {
                                let x_start = text_rect.min.x
                                    + (gutter_width as f32 + gutter_padding) * char_width
                                    + start_col as f32 * char_width;
                                let x_end = text_rect.min.x
                                    + (gutter_width as f32 + gutter_padding) * char_width
                                    + end_col as f32 * char_width;

                                let highlight_rect = Rect::from_min_max(
                                    Pos2 { x: x_start, y },
                                    Pos2 {
                                        x: x_end,
                                        y: y + char_height,
                                    },
                                );

                                ui.painter().rect_filled(
                                    highlight_rect,
                                    0.0,
                                    Color32::from_gray(80),
                                );
                            }
                        }
                    }
                }

                for (col, ch) in line.chars().enumerate() {
                    let pos = Pos2 {
                        x: text_rect.min.x
                            + (gutter_width as f32 + gutter_padding) * char_width
                            + col as f32 * char_width,
                        y,
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
                x: text_rect.min.x
                    + (gutter_width as f32 + gutter_padding) * char_width
                    + cursor_col as f32 * char_width,
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
                Mode::Visual => {
                    ui.painter().rect_filled(
                        Rect::from_min_size(cursor_pos, egui::vec2(char_width, char_height)),
                        0.0,
                        Color32::LIGHT_BLUE,
                    );
                }
                _ => {}
            }
        });

        egui::TopBottomPanel::top("bufferline").show(ctx, |ui| {
            ui.horizontal(|ui| {
                for (id, name, active) in state.buffer_line() {
                    let render = format!("{}{}", id, name);
                    let text = if active {
                        RichText::new(render)
                            .color(Color32::BLACK)
                            .background_color(Color32::LIGHT_BLUE)
                            .strong()
                    } else {
                        RichText::new(render).color(Color32::GRAY)
                    };

                    if ui
                        .add(egui::Label::new(text).sense(egui::Sense::click()))
                        .clicked()
                    {
                        state.focused_buf_id = id;
                    }
                    ui.set_height(buffer_line_height);
                }
            });
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
                            ui.label(RichText::new(c).underline().strong());
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

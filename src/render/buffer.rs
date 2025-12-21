use std::sync::MutexGuard;

use eframe::egui;
use egui::{Color32, Context, LayerId, Rect};

use egui::Pos2;

use egui::FontId;

use crate::buffer::Mode;
use crate::editor::EditorState;

pub fn render_buffer(ctx: &Context, state: &mut MutexGuard<'_, EditorState>) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let font_size = 16.0;
        let font_id = FontId::monospace(font_size);

        let char_width = ui.fonts(|f| f.glyph_width(&font_id, 'W'));
        let char_height = ui.fonts(|f| f.row_height(&font_id));

        let screen_height = (ui.available_height() / char_height) as usize;

        state.screen_height = screen_height;
        let buf = state.focused_buf_mut();
        buf.update_scroll(screen_height, 8);

        let gutter_width = ((buf.line_count() as f32).log10().ceil() as usize).max(2) + 2;
        let gutter_padding = 1.0;

        let max_line_len = (0..buf.line_count())
            .map(|i| buf.line_len(i))
            .max()
            .unwrap_or(1);

        let text_rect = ui
            .allocate_space(egui::vec2(
                (gutter_width as f32 + gutter_padding) * char_width
                    + char_width * max_line_len as f32,
                char_height * buf.line_count() as f32,
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
        let end = (start + screen_height).min(buf.line_count());

        for (row_idx, line) in buf.lines.lines_at(start).take(end - start).enumerate() {
            let row = start + row_idx;
            let y = text_rect.min.y + row_idx as f32 * char_height;
            let line_number = row + 1;

            let line_number_color = if buf.cursor.row == row {
                Color32::LIGHT_BLUE
            } else {
                Color32::GRAY
            };

            let line_number_text = if buf.cursor.row == row {
                format!("{:>width$}", line_number, width = gutter_width)
            } else {
                let relative_line_number = row.abs_diff(buf.cursor.row);
                format!("{:>width$}", relative_line_number, width = gutter_width)
            };
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

                    if row >= start_cursor.row && row <= end_cursor.row {
                        let start_col = if row == start_cursor.row {
                            start_cursor.col
                        } else {
                            0
                        };
                        let end_col = if row == end_cursor.row {
                            end_cursor.col + 1
                        } else {
                            buf.line_len(row)
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

                            ui.painter()
                                .rect_filled(highlight_rect, 0.0, Color32::from_gray(80));
                        }
                    }
                }
            }

            // Highlight last motion range
            if let Some(range) = &buf.range {
                let row = buf.cursor.row; // single-line range assumed at cursor row
                if row >= start && row < end {
                    let y = text_rect.min.y + (row - start) as f32 * char_height;

                    let start_col = range.anchor.min(range.head);
                    let end_col = range.anchor.max(range.head);

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
                            Color32::from_rgba_unmultiplied(100, 100, 100, 50),
                        );
                    }
                }
            }

            for (col, ch) in line.chars().enumerate() {
                if ch == '\n' || ch == '\r' {
                    continue;
                }
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

        let cursor_row = buf.cursor.row.min(buf.line_count().saturating_sub(1));
        let cursor_col = buf.cursor.col.min(buf.line_len(cursor_row));
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
}

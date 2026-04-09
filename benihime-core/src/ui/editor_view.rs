use benihime_renderer::{color::Color, Renderer};

use crate::{
    graphics::Rect,
    ui::composer::{Component, Context},
};

pub struct EditorView {
    scroll_offset: usize,
}

impl EditorView {
    pub fn new() -> Self {
        Self { scroll_offset: 0 }
    }
}

impl Default for EditorView {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for EditorView {
    fn render(&mut self, area: Rect, surface: &mut Renderer, ctx: &mut Context) {
        let editor = &ctx.editor;
        let buffer = editor.focused_buf();
        let line_count = buffer.line_count();

        let cell_width = surface.cell_width();
        let cell_height = surface.cell_height();
        let cell_height_u16 = cell_height as u16;

        let buffer_line_height = cell_height_u16;
        let status_line_height = cell_height_u16;

        let minibuffer_height = if editor.minibuffer_manager.current.is_some() {
            cell_height_u16
        } else {
            0
        };

        let gutter_width_chars = 4;
        let gutter_width = (gutter_width_chars as f32 * cell_width).ceil();
        let gutter_width_u16 = gutter_width as u16;
        let editor_start_x = area.x + gutter_width_u16;

        let y_offset = area.y + buffer_line_height;
        let editor_area_height = area
            .height
            .saturating_sub(buffer_line_height + status_line_height + minibuffer_height);

        surface.draw_rect(
            area.x as f32,
            y_offset as f32,
            area.width as f32,
            editor_area_height as f32,
            Color::rgb(0.1, 0.1, 0.15),
        );

        surface.draw_rect(
            area.x as f32,
            y_offset as f32,
            gutter_width,
            editor_area_height as f32,
            Color::rgb(0.08, 0.08, 0.12),
        );

        let visible_lines = (editor_area_height as f32 / cell_height).floor() as usize;

        let start_line = buffer.scroll_offset;
        let end_line = (start_line + visible_lines).min(line_count);
        let cursor_row = buffer.cursor.row;

        let scroll_left = buffer.scroll_left;
        let visible_cols = (area.width as f32 / cell_width).floor() as usize;

        for (row, line_idx) in (start_line..end_line).enumerate() {
            let line = buffer.line(line_idx);
            let line_str = line.to_string();

            let y = y_offset as f32 + (row as f32 * cell_height);

            let is_current_line = line_idx == cursor_row;
            let line_num_str = format!("{:>4}", line_idx + 1);

            let (text, color) = if is_current_line {
                (line_num_str.clone(), Color::rgb(0.9, 0.6, 0.4))
            } else if line_idx < cursor_row {
                (
                    format!("{:>4}", cursor_row - line_idx),
                    Color::rgb(0.4, 0.4, 0.45),
                )
            } else {
                (
                    format!("{:>4}", line_idx - cursor_row),
                    Color::rgb(0.4, 0.4, 0.45),
                )
            };

            let line_num_section = benihime_renderer::text::TextSection::simple(
                area.x as f32,
                y,
                text.as_str(),
                surface.font_size() * 0.8,
                color,
            );
            surface.draw_text(line_num_section);

            if let Some(range) = &buffer.range {
                if !range.is_empty() {
                    let anchor_col = range.anchor;
                    let head_col = range.head;
                    let range_from_col = anchor_col.min(head_col);
                    let range_to_col = anchor_col.max(head_col);

                    if line_idx == cursor_row {
                        let highlight_start_byte = buffer.char_to_byte(line_idx, range_from_col);
                        let highlight_end_byte = buffer.char_to_byte(line_idx, range_to_col);

                        let visual_start = highlight_start_byte;
                        let visual_end = highlight_end_byte;

                        let vis_start = visual_start.saturating_sub(scroll_left);
                        let vis_end = visual_end.saturating_sub(scroll_left);

                        if vis_start < visible_cols && vis_end > 0 {
                            let x1 = editor_start_x as f32 + (vis_start as f32 * cell_width);
                            let x2 = editor_start_x as f32 + (vis_end as f32 * cell_width);
                            let highlight_width = x2 - x1;

                            if highlight_width > 0.0 {
                                surface.draw_rect(
                                    x1,
                                    y,
                                    highlight_width,
                                    cell_height,
                                    Color::rgb(0.3, 0.25, 0.2),
                                );
                            }
                        }
                    }
                }
            }

            if !line_str.is_empty() {
                let start_col = scroll_left;
                if start_col < line_str.len() {
                    let end_col = (start_col + visible_cols).min(line_str.len());
                    let visible_text = &line_str[start_col..end_col];
                    let x_pos = editor_start_x as f32;

                    let section = benihime_renderer::text::TextSection::simple(
                        x_pos,
                        y,
                        visible_text,
                        surface.font_size(),
                        Color::WHITE,
                    );
                    surface.draw_text(section);
                }
            }
        }
    }

    fn cursor(
        &self,
        _area: Rect,
        _app: &crate::application::Application,
    ) -> (
        Option<crate::position::Position>,
        crate::graphics::CursorKind,
    ) {
        (None, crate::graphics::CursorKind::Hidden)
    }
}

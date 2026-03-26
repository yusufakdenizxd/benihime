use benihime_renderer::Renderer;

use crate::{
    application::Application,
    buffer::Mode,
    graphics::{CursorKind, Rect},
    position::Position,
    ui::composer::{Component, Context},
};

pub struct CursorComponent;

impl CursorComponent {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CursorComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for CursorComponent {
    fn render(&mut self, area: Rect, surface: &mut Renderer, ctx: &mut Context) {
        let editor = &ctx.editor;
        let buffer = editor.focused_buf();

        let cursor_row = buffer.cursor.row.saturating_sub(buffer.scroll_offset);
        let cursor_col = buffer.cursor.col.saturating_sub(buffer.scroll_left);

        let cell_width = surface.cell_width();
        let cell_height = surface.cell_height();

        let buffer_line_height = cell_height as u16;
        let status_line_height = cell_height as u16;
        let minibuffer_height = if editor.minibuffer_manager.current.is_some() {
            cell_height as u16
        } else {
            0
        };

        let gutter_width_chars = 4;
        let gutter_width = (gutter_width_chars as f32 * cell_width).ceil();
        let editor_start_x = area.x + gutter_width as u16;

        let y_offset = area.y + buffer_line_height;
        let visible_rows = area
            .height
            .saturating_sub(buffer_line_height + status_line_height + minibuffer_height)
            as usize;
        let visible_cols = ((area.width as f32 - gutter_width) / cell_width) as usize;

        let cursor_row = cursor_row.min(visible_rows.saturating_sub(1));
        let cursor_col = cursor_col.min(visible_cols.saturating_sub(1));

        let y = y_offset as f32 + (cursor_row as f32 * cell_height);
        let x = editor_start_x as f32 + (cursor_col as f32 * cell_width);

        let cursor_kind = match buffer.mode {
            Mode::Insert => CursorKind::Bar,
            _ => CursorKind::Block,
        };

        match cursor_kind {
            CursorKind::Bar => {
                let bar_width = 2.0;
                surface.draw_rect(
                    x,
                    y,
                    bar_width,
                    cell_height,
                    benihime_renderer::color::Color::WHITE,
                );
            }
            _ => {
                surface.draw_rect(
                    x,
                    y,
                    cell_width,
                    cell_height,
                    benihime_renderer::color::Color::WHITE,
                );
            }
        }
    }

    fn cursor(&self, _area: Rect, app: &Application) -> (Option<Position>, CursorKind) {
        let editor = &app.editor;
        let buffer = editor.focused_buf();

        let cursor_row = buffer.cursor.row.saturating_sub(buffer.scroll_offset);
        let cursor_col = buffer.cursor.col.saturating_sub(buffer.scroll_left);

        let cursor_kind = match buffer.mode {
            Mode::Insert => CursorKind::Bar,
            _ => CursorKind::Block,
        };

        (Some(Position::new(cursor_row, cursor_col)), cursor_kind)
    }
}

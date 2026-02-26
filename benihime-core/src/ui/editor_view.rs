use benihime_renderer::Renderer;

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

        let y_offset = area.y + buffer_line_height;
        let editor_area_height = area
            .height
            .saturating_sub(buffer_line_height + status_line_height + minibuffer_height);

        surface.draw_rect(
            area.x as f32,
            y_offset as f32,
            area.width as f32,
            editor_area_height as f32,
            benihime_renderer::color::Color::rgb(0.1, 0.1, 0.15),
        );

        let visible_lines = (editor_area_height as f32 / cell_height).floor() as usize;

        let start_line = buffer.scroll_offset;
        let end_line = (start_line + visible_lines).min(line_count);

        for (row, line_idx) in (start_line..end_line).enumerate() {
            let line = buffer.line(line_idx);
            let line_str = line.to_string();

            let y = y_offset as f32 + (row as f32 * cell_height);

            if !line_str.is_empty() {
                let section = benihime_renderer::text::TextSection::simple(
                    area.x as f32,
                    y,
                    line_str,
                    surface.font_size(),
                    benihime_renderer::color::Color::WHITE,
                );
                surface.draw_text(section);
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

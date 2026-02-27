use benihime_renderer::Renderer;

use crate::{
    graphics::Rect,
    ui::composer::{Component, Context},
};

pub struct BufferLine;

impl BufferLine {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BufferLine {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for BufferLine {
    fn render(&mut self, area: Rect, surface: &mut Renderer, ctx: &mut Context) {
        let editor = &ctx.editor;
        let buffers = editor.buffer_line();

        let cell_height = surface.cell_height() as u16;

        surface.draw_rect(
            area.x as f32,
            area.y as f32,
            area.width as f32,
            cell_height as f32,
            benihime_renderer::color::Color::rgb(0.2, 0.2, 0.25),
        );

        let mut x_offset = area.x as f32;

        for (_buf_id, name, is_active, _is_modified) in buffers {
            let label = format!(" {} ", name.as_str());

            let color = if is_active {
                benihime_renderer::color::Color::rgb(0.8, 0.4, 0.4)
            } else {
                benihime_renderer::color::Color::rgb(0.5, 0.5, 0.5)
            };

            let section = benihime_renderer::text::TextSection::simple(
                x_offset,
                area.y as f32,
                label.as_str(),
                surface.font_size(),
                color,
            );
            surface.draw_text(section);

            let label_width = label.len() as f32 * surface.cell_width();
            x_offset += label_width;
        }
    }
}

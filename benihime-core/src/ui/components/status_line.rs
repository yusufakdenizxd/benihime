use benihime_renderer::Renderer;

use crate::{
    graphics::Rect,
    ui::composer::{Component, Context},
};

pub struct StatusLine;

impl StatusLine {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StatusLine {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for StatusLine {
    fn render(&mut self, area: Rect, surface: &mut Renderer, ctx: &mut Context) {
        let editor = &ctx.editor;
        let status_text = editor.status_line();
        let status_width = (status_text.len() as f32 + 1.0) * surface.cell_width();

        let cell_height = surface.cell_height() as u16;
        let y = area.y + area.height.saturating_sub(cell_height);

        surface.draw_rect(
            area.x as f32,
            y as f32,
            area.width as f32,
            cell_height as f32,
            benihime_renderer::color::Color::rgb(0.15, 0.15, 0.2),
        );

        let section = benihime_renderer::text::TextSection::simple(
            area.x as f32,
            y as f32,
            status_text,
            surface.font_size(),
            benihime_renderer::color::Color::WHITE,
        );
        surface.draw_text(section);

        if !editor.command_buffer.is_empty() {
            let section = benihime_renderer::text::TextSection::simple(
                area.x as f32 + status_width,
                y as f32,
                editor.command_buffer.clone(),
                surface.font_size(),
                benihime_renderer::color::Color::WHITE,
            );
            surface.draw_text(section);
        }

        if let Some(ref err) = editor.error_message {
            let err_width = (err.len() as f32 + 1.0) * surface.cell_width();
            let x = area.x as f32 + area.width as f32 - err_width;
            let section = benihime_renderer::text::TextSection::simple(
                x,
                y as f32,
                err.clone(),
                surface.font_size(),
                benihime_renderer::color::Color::rgb(1.0, 0.3, 0.3),
            );
            surface.draw_text(section);
        }
    }
}

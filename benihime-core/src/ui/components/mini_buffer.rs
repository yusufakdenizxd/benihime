use benihime_renderer::Renderer;

use crate::{
    graphics::Rect,
    ui::composer::{Component, Context},
};

pub struct MiniBufferComponent;

impl MiniBufferComponent {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MiniBufferComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for MiniBufferComponent {
    fn render(&mut self, area: Rect, surface: &mut Renderer, ctx: &mut Context) {
        let minibuffer_manager = &ctx.editor.minibuffer_manager;

        let Some(mini) = &minibuffer_manager.current else {
            return;
        };

        let cell_height = surface.cell_height() as u16;
        let status_line_height = cell_height;

        let prompt = mini.prompt();
        let input = mini.input();

        let candidates = mini.render_candidates();
        let total_candidates = candidates.len();
        let max_visible = 10.min(total_candidates);
        let offset = mini.offset();
        let index = mini.index();

        let available_height = area.height.saturating_sub(status_line_height);
        let minibuffer_y = area.y + available_height.saturating_sub(cell_height);
        let minibuffer_height = cell_height + (max_visible as u16 * cell_height);

        surface.draw_rect(
            area.x as f32,
            minibuffer_y as f32,
            area.width as f32,
            minibuffer_height as f32,
            benihime_renderer::color::Color::rgb(0.15, 0.15, 0.2),
        );

        let input_y = minibuffer_y;

        if total_candidates > 0 {
            let candidate_area_height = max_visible as u16 * cell_height;
            let candidate_area_start = input_y.saturating_sub(candidate_area_height);

            surface.with_overlay_region(
                area.x as f32,
                candidate_area_start as f32,
                area.width as f32,
                candidate_area_height as f32,
                |f| {
                    f.draw_rect(
                        area.x as f32,
                        candidate_area_start as f32,
                        area.width as f32,
                        candidate_area_height as f32,
                        benihime_renderer::color::Color::rgb(0.1, 0.1, 0.15),
                    );

                    for i in 0..max_visible {
                        let candidate_idx = offset + i;
                        if candidate_idx >= total_candidates {
                            break;
                        }

                        let candidate_y = candidate_area_start + (i as u16 * cell_height);
                        let candidate_text = &candidates[candidate_idx];

                        let color = if candidate_idx == index {
                            benihime_renderer::color::Color::rgb(0.9, 0.6, 0.4)
                        } else {
                            benihime_renderer::color::Color::rgb(0.6, 0.6, 0.6)
                        };

                        let section = benihime_renderer::text::TextSection::simple(
                            area.x as f32,
                            candidate_y as f32,
                            candidate_text.as_str(),
                            f.font_size(),
                            color,
                        );
                        f.draw_text(section);
                    }

                    if total_candidates > max_visible {
                        let scroll_info = format!("[{}/{}]", index + 1, total_candidates);
                        let scroll_section = benihime_renderer::text::TextSection::simple(
                            (area.x + area.width - 10) as f32,
                            candidate_area_start as f32,
                            scroll_info.as_str(),
                            f.font_size(),
                            benihime_renderer::color::Color::rgb(0.4, 0.4, 0.4),
                        );
                        f.draw_text(scroll_section);
                    }
                },
            );
        }

        surface.draw_rect(
            area.x as f32,
            input_y as f32,
            area.width as f32,
            cell_height as f32,
            benihime_renderer::color::Color::rgb(0.2, 0.2, 0.25),
        );

        let prompt_section = benihime_renderer::text::TextSection::simple(
            area.x as f32,
            input_y as f32,
            prompt,
            surface.font_size(),
            benihime_renderer::color::Color::WHITE,
        );
        surface.draw_text(prompt_section);

        if !input.is_empty() {
            let prompt_width = prompt.len() as f32 * surface.cell_width();
            let input_section = benihime_renderer::text::TextSection::simple(
                area.x as f32 + prompt_width,
                input_y as f32,
                input,
                surface.font_size(),
                benihime_renderer::color::Color::rgb(0.5, 0.7, 0.5),
            );
            surface.draw_text(input_section);
        }
    }

    fn should_update(&self) -> bool {
        true
    }
}

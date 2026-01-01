use crate::graphics::{Color, FontId, Point, Rect};

#[derive(Debug, Clone)]
pub struct GlyphInstance {
    pub glyph_index: u32,
    pub position: Point,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub enum DrawCommand {
    Clear(Color),
    Rect {
        rect: Rect,
        color: Color,
    },
    GlyphRun {
        font: FontId,
        glyphs: Vec<GlyphInstance>,
    },
}

#[derive(Debug, Clone)]
pub struct DisplayList {
    pub commands: Vec<DrawCommand>,
}

impl DisplayList {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
}

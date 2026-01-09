use crate::graphics::{Color, CursorKind};

pub struct LineRender {
    pub text: String,
    pub style: Option<Color>,
}

pub struct CursorRender {
    pub row: usize,
    pub col: usize,
    pub kind: CursorKind,
}

#[derive(Debug, Copy, Clone)]
pub struct Viewport {
    pub width: usize,
    pub height: usize,
    pub scroll_row: usize,
    pub scroll_col: usize,
}

pub struct Frame {
    pub lines: Vec<LineRender>,
    pub cursor: CursorRender,
    pub viewport: Viewport,
}

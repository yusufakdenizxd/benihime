use crate::color::Color;

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub color: Color,
    pub size: f32,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            size: 16.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextSegment {
    pub content: String,
    pub style: TextStyle,
}

impl TextSegment {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: TextStyle::default(),
        }
    }

    pub fn with_style(mut self, style: TextStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.style.color = color;
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.style.size = size;
        self
    }
}

#[derive(Debug, Clone)]
pub struct TextSection {
    pub position: (f32, f32),
    pub texts: Vec<TextSegment>,
}

impl TextSection {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: (x, y),
            texts: Vec::new(),
        }
    }

    pub fn add_text(mut self, text: TextSegment) -> Self {
        self.texts.push(text);
        self
    }

    pub fn simple(x: f32, y: f32, content: impl Into<String>, size: f32, color: Color) -> Self {
        Self {
            position: (x, y),
            texts: vec![TextSegment {
                content: content.into(),
                style: TextStyle { color, size },
            }],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Font {
    pub data: Vec<u8>,
}

impl Font {
    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self { data }
    }
}

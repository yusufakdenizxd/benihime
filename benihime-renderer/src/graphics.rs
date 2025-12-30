use std::str::FromStr;

use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorKind {
    Block,
    Bar,
    Underline,
    Hidden,
}

impl Default for CursorKind {
    fn default() -> Self {
        Self::Block
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hex_string(s: &str) -> Self {
        if s.len() >= 7 {
            if let (Ok(red), Ok(green), Ok(blue)) = (
                u8::from_str_radix(&s[1..3], 16),
                u8::from_str_radix(&s[3..5], 16),
                u8::from_str_radix(&s[5..7], 16),
            ) {
                return Color::from_rgb(red, green, blue);
            }
        }
        Color::default()
    }
}

bitflags! {
    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    pub struct Modifier: u16 {
        const BOLD              = 0b0000_0000_0001;
        const ITALIC            = 0b0000_0000_0010;
        const CROSSED_OUT       = 0b0000_0000_0100;
    }
}

impl FromStr for Modifier {
    type Err = &'static str;

    fn from_str(modifier: &str) -> Result<Self, Self::Err> {
        match modifier {
            "bold" => Ok(Self::BOLD),
            "italic" => Ok(Self::ITALIC),
            "crossed_out" => Ok(Self::CROSSED_OUT),
            _ => Err("Invalid modifier"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HighlightGroup {
    pub fg: Option<Color>,
    pub bg: Option<Color>,

    pub modifier: Modifier,
}

impl Default for HighlightGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl HighlightGroup {
    pub const fn new() -> Self {
        HighlightGroup {
            fg: None,
            bg: None,
            modifier: Modifier::empty(),
        }
    }
    pub const fn fg(mut self, color: Color) -> HighlightGroup {
        self.fg = Some(color);
        self
    }

    pub const fn bg(mut self, color: Color) -> HighlightGroup {
        self.bg = Some(color);
        self
    }

    pub fn add_modifier(mut self, modifier: Modifier) -> HighlightGroup {
        self.modifier.insert(modifier);
        self
    }

    pub fn remove_modifier(mut self, modifier: Modifier) -> HighlightGroup {
        self.modifier.remove(modifier);
        self
    }

    pub fn patch(mut self, other: HighlightGroup) -> HighlightGroup {
        self.fg = other.fg.or(self.fg);
        self.bg = other.bg.or(self.bg);

        self.modifier.insert(other.modifier);

        self
    }
}

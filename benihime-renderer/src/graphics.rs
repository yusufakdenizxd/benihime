use std::{
    cmp::{max, min},
    str::FromStr,
};

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Margin {
    pub horizontal: u16,
    pub vertical: u16,
}

impl Margin {
    pub fn none() -> Self {
        Self {
            horizontal: 0,
            vertical: 0,
        }
    }

    pub const fn all(value: u16) -> Self {
        Self {
            horizontal: value,
            vertical: value,
        }
    }

    pub const fn horizontal(value: u16) -> Self {
        Self {
            horizontal: value,
            vertical: 0,
        }
    }

    pub const fn vertical(value: u16) -> Self {
        Self {
            horizontal: 0,
            vertical: value,
        }
    }

    pub const fn width(&self) -> u16 {
        self.horizontal * 2
    }

    pub const fn height(&self) -> u16 {
        self.vertical * 2
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
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,

    pub font_size: Option<usize>,

    pub modifier: Modifier,
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

impl Style {
    pub const fn new() -> Self {
        Style {
            fg: None,
            bg: None,
            font_size: None,
            modifier: Modifier::empty(),
        }
    }
    pub const fn fg(mut self, color: Color) -> Style {
        self.fg = Some(color);
        self
    }

    pub const fn bg(mut self, color: Color) -> Style {
        self.bg = Some(color);
        self
    }

    pub const fn font_size(mut self, font_size: usize) -> Style {
        self.font_size = Some(font_size);
        self
    }

    pub fn add_modifier(mut self, modifier: Modifier) -> Style {
        self.modifier.insert(modifier);
        self
    }

    pub fn remove_modifier(mut self, modifier: Modifier) -> Style {
        self.modifier.remove(modifier);
        self
    }

    pub fn patch(mut self, other: Style) -> Style {
        self.fg = other.fg.or(self.fg);
        self.bg = other.bg.or(self.bg);

        self.modifier.insert(other.modifier);

        self
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FontId(pub u32);

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn area(self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn left(self) -> u16 {
        self.x
    }

    pub fn right(self) -> u16 {
        self.x + self.width
    }

    pub fn top(self) -> u16 {
        self.y
    }

    pub fn bottom(self) -> u16 {
        self.y + self.height
    }

    pub fn intersects(self, other: Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    pub fn intersection(self, other: Rect) -> Rect {
        let x1 = max(self.x, other.x);
        let y1 = max(self.y, other.y);
        let x2 = min(self.right(), other.right());
        let y2 = min(self.bottom(), other.bottom());

        Rect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    pub fn union(self, other: Rect) -> Rect {
        let x1 = min(self.x, other.x);
        let y1 = min(self.y, other.y);
        let x2 = max(self.right(), other.right());
        let y2 = max(self.bottom(), other.bottom());

        Rect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_size_preservation() {
        for width in 0..256u16 {
            for height in 0..256u16 {
                let rect = Rect::new(0, 0, width, height);
                rect.area();
                assert_eq!(rect.width, width);
                assert_eq!(rect.height, height);
            }
        }

        let rect = Rect::new(0, 0, 300, 100);
        assert_eq!(rect.width, 300);
        assert_eq!(rect.height, 100);
    }

    #[test]
    fn test_rect_union() {
        let rect1 = Rect::new(0, 0, 5, 5);
        let rect2 = Rect::new(5, 0, 2, 2);
        assert_eq!(rect1.union(rect2), Rect::new(0, 0, 7, 5));
    }

    #[test]
    fn test_rect_intersect() {
        let rect1 = Rect::new(0, 0, 5, 5);
        let rect2 = Rect::new(5, 0, 2, 2);
        assert_eq!(rect1.intersection(rect2), Rect::new(5, 0, 0, 2));
    }
}

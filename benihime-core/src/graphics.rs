use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorKind {
    Block,
    Bar,
    Underline,
    Hollow,
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

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
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

    #[inline]
    pub fn area(self) -> usize {
        (self.width as usize) * (self.height as usize)
    }

    #[inline]
    pub fn left(self) -> u16 {
        self.x
    }

    #[inline]
    pub fn right(self) -> u16 {
        self.x.saturating_add(self.width)
    }

    #[inline]
    pub fn top(self) -> u16 {
        self.y
    }

    #[inline]
    pub fn bottom(self) -> u16 {
        self.y.saturating_add(self.height)
    }

    pub fn clip_left(self, width: u16) -> Rect {
        let width = std::cmp::min(width, self.width);
        Rect {
            x: self.x.saturating_add(width),
            width: self.width.saturating_sub(width),
            ..self
        }
    }

    pub fn clip_right(self, width: u16) -> Rect {
        Rect {
            width: self.width.saturating_sub(width),
            ..self
        }
    }

    pub fn clip_top(self, height: u16) -> Rect {
        let height = std::cmp::min(height, self.height);
        Rect {
            y: self.y.saturating_add(height),
            height: self.height.saturating_sub(height),
            ..self
        }
    }

    pub fn clip_bottom(self, height: u16) -> Rect {
        Rect {
            height: self.height.saturating_sub(height),
            ..self
        }
    }

    pub fn with_height(self, height: u16) -> Rect {
        Self::new(self.x, self.y, self.width, height)
    }

    pub fn with_width(self, width: u16) -> Rect {
        Self::new(self.x, self.y, width, self.height)
    }

    pub fn inner(self, margin: Margin) -> Rect {
        if self.width < margin.width() || self.height < margin.height() {
            Rect::default()
        } else {
            Rect {
                x: self.x + margin.horizontal,
                y: self.y + margin.vertical,
                width: self.width - margin.width(),
                height: self.height - margin.height(),
            }
        }
    }

    pub fn union(self, other: Rect) -> Rect {
        let x1 = min(self.x, other.x);
        let y1 = min(self.y, other.y);
        let x2 = max(self.x + self.width, other.x + other.width);
        let y2 = max(self.y + self.height, other.y + other.height);
        Rect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    pub fn intersection(self, other: Rect) -> Rect {
        let x1 = max(self.x, other.x);
        let y1 = max(self.y, other.y);
        let x2 = min(self.x + self.width, other.x + other.width);
        let y2 = min(self.y + self.height, other.y + other.height);
        Rect {
            x: x1,
            y: y1,
            width: x2.saturating_sub(x1),
            height: y2.saturating_sub(y1),
        }
    }

    pub fn intersects(self, other: Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

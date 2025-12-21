use crate::buffer::Cursor;

use super::movement::Direction;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Range {
    pub anchor: usize,
    pub head: usize,
}

impl Range {
    pub fn new(anchor: usize, head: usize) -> Self {
        Self { anchor, head }
    }

    pub fn point(head: usize) -> Self {
        Self::new(head, head)
    }

    /// `true` when head and anchor are at the same position.
    pub fn is_empty(&self) -> bool {
        self.anchor == self.head
    }

    /// Start of the range.
    pub fn from(&self) -> usize {
        std::cmp::min(self.anchor, self.head)
    }

    /// End of the range.
    pub fn to(&self) -> usize {
        std::cmp::max(self.anchor, self.head)
    }

    /// `Direction::Backward` when head < anchor.
    /// `Direction::Forward` otherwise.
    pub fn direction(&self) -> Direction {
        if self.head < self.anchor {
            Direction::Backward
        } else {
            Direction::Forward
        }
    }

    /// Flips the direction of the selection
    pub fn flip(&self) -> Self {
        Self {
            anchor: self.head,
            head: self.anchor,
        }
    }

    pub fn with_direction(self, direction: Direction) -> Self {
        if self.direction() == direction {
            self
        } else {
            self.flip()
        }
    }

    /// Check two ranges for overlap.
    pub fn overlaps(&self, other: &Self) -> bool {
        self.to() > other.from() && other.to() > self.from()
    }

    pub fn contains_range(&self, other: &Self) -> bool {
        self.from() <= other.from() && self.to() >= other.to()
    }

    pub fn contains(&self, pos: usize) -> bool {
        self.from() <= pos && pos < self.to()
    }

    pub fn merge(&self, other: Self) -> Self {
        let start = self.from().min(other.from());
        let end = self.to().max(other.to());
        Self {
            anchor: start,
            head: end,
        }
    }
}

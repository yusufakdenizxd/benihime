use std::str::Chars;

use super::selection::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Backward,
    Forward,
}

/// Possible targets of a word motion
#[derive(Copy, Clone, Debug)]
pub enum WordMotionTarget {
    NextWordStart,
    NextWordEnd,
    PrevWordStart,
    PrevWordEnd,
    NextLongWordStart,
    NextLongWordEnd,
    PrevLongWordStart,
    PrevLongWordEnd,
    NextSubWordStart,
    NextSubWordEnd,
    PrevSubWordStart,
    PrevSubWordEnd,
}

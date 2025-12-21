use ropey::{RopeSlice, iter::Chars};

use crate::chars::{CharCategory, categorize_char, char_is_line_ending};

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

fn is_word_boundary(a: char, b: char) -> bool {
    categorize_char(a) != categorize_char(b)
}

fn is_long_word_boundary(a: char, b: char) -> bool {
    match (categorize_char(a), categorize_char(b)) {
        (CharCategory::Word, CharCategory::Punctuation)
        | (CharCategory::Punctuation, CharCategory::Word) => false,
        (a, b) if a != b => true,
        _ => false,
    }
}

fn is_sub_word_boundary(a: char, b: char, dir: Direction) -> bool {
    match (categorize_char(a), categorize_char(b)) {
        (CharCategory::Word, CharCategory::Word) => {
            if (a == '_') != (b == '_') {
                return true;
            }

            match dir {
                Direction::Forward => a.is_lowercase() && b.is_uppercase(),
                Direction::Backward => a.is_uppercase() && b.is_lowercase(),
            }
        }
        (a, b) if a != b => true,
        _ => false,
    }
}

fn range_to_target(chars: &mut Chars<'_>, target: WordMotionTarget, origin: Range) -> Range {
    let prev = matches!(
        target,
        WordMotionTarget::PrevWordStart
            | WordMotionTarget::PrevLongWordStart
            | WordMotionTarget::PrevSubWordStart
            | WordMotionTarget::PrevWordEnd
            | WordMotionTarget::PrevLongWordEnd
            | WordMotionTarget::PrevSubWordEnd
    );

    // Reverse the iterator if needed for the motion direction.
    if prev {
        chars.reverse();
    }

    // Function to advance index in the appropriate motion direction.
    let advance: &dyn Fn(&mut usize) = if prev {
        &|idx| *idx = idx.saturating_sub(1)
    } else {
        &|idx| *idx += 1
    };

    // Initialize state variables.
    let mut anchor = origin.anchor;
    let mut head = origin.head;
    let mut prev_ch = {
        let ch = chars.prev();
        if ch.is_some() {
            chars.next();
        }
        ch
    };

    // Skip any initial newline characters.
    while let Some(ch) = chars.next() {
        if char_is_line_ending(ch) {
            prev_ch = Some(ch);
            advance(&mut head);
        } else {
            chars.prev();
            break;
        }
    }
    if prev_ch.map(char_is_line_ending).unwrap_or(false) {
        anchor = head;
    }

    let head_start = head;
    while let Some(next_ch) = chars.next() {
        if prev_ch.is_none() || reached_target(target, prev_ch.unwrap(), next_ch) {
            if head == head_start {
                anchor = head;
            } else {
                break;
            }
        }
        prev_ch = Some(next_ch);
        advance(&mut head);
    }

    if prev {
        chars.reverse();
    }

    Range::new(anchor, head)
}

fn reached_target(target: WordMotionTarget, prev_ch: char, next_ch: char) -> bool {
    match target {
        WordMotionTarget::NextWordStart | WordMotionTarget::PrevWordEnd => {
            is_word_boundary(prev_ch, next_ch)
                && (char_is_line_ending(next_ch) || !next_ch.is_whitespace())
        }
        WordMotionTarget::NextWordEnd | WordMotionTarget::PrevWordStart => {
            is_word_boundary(prev_ch, next_ch)
                && (!prev_ch.is_whitespace() || char_is_line_ending(next_ch))
        }
        WordMotionTarget::NextLongWordStart | WordMotionTarget::PrevLongWordEnd => {
            is_long_word_boundary(prev_ch, next_ch)
                && (char_is_line_ending(next_ch) || !next_ch.is_whitespace())
        }
        WordMotionTarget::NextLongWordEnd | WordMotionTarget::PrevLongWordStart => {
            is_long_word_boundary(prev_ch, next_ch)
                && (!prev_ch.is_whitespace() || char_is_line_ending(next_ch))
        }
        WordMotionTarget::NextSubWordStart => {
            is_sub_word_boundary(prev_ch, next_ch, Direction::Forward)
                && (char_is_line_ending(next_ch) || !(next_ch.is_whitespace() || next_ch == '_'))
        }
        WordMotionTarget::PrevSubWordEnd => {
            is_sub_word_boundary(prev_ch, next_ch, Direction::Backward)
                && (char_is_line_ending(next_ch) || !(next_ch.is_whitespace() || next_ch == '_'))
        }
        WordMotionTarget::NextSubWordEnd => {
            is_sub_word_boundary(prev_ch, next_ch, Direction::Forward)
                && (!(prev_ch.is_whitespace() || prev_ch == '_') || char_is_line_ending(next_ch))
        }
        WordMotionTarget::PrevSubWordStart => {
            is_sub_word_boundary(prev_ch, next_ch, Direction::Backward)
                && (!(prev_ch.is_whitespace() || prev_ch == '_') || char_is_line_ending(next_ch))
        }
    }
}

pub fn move_next_word_start(slice: RopeSlice, range: Range, count: usize) -> Range {
    word_move(slice, range, count, WordMotionTarget::NextWordStart)
}

pub fn move_next_word_end(slice: RopeSlice, range: Range, count: usize) -> Range {
    word_move(slice, range, count, WordMotionTarget::NextWordEnd)
}

pub fn move_prev_word_start(slice: RopeSlice, range: Range, count: usize) -> Range {
    word_move(slice, range, count, WordMotionTarget::PrevWordStart)
}

pub fn move_prev_word_end(slice: RopeSlice, range: Range, count: usize) -> Range {
    word_move(slice, range, count, WordMotionTarget::PrevWordEnd)
}

pub fn move_next_long_word_start(slice: RopeSlice, range: Range, count: usize) -> Range {
    word_move(slice, range, count, WordMotionTarget::NextLongWordStart)
}

pub fn move_next_long_word_end(slice: RopeSlice, range: Range, count: usize) -> Range {
    word_move(slice, range, count, WordMotionTarget::NextLongWordEnd)
}

pub fn move_prev_long_word_start(slice: RopeSlice, range: Range, count: usize) -> Range {
    word_move(slice, range, count, WordMotionTarget::PrevLongWordStart)
}

pub fn move_prev_long_word_end(slice: RopeSlice, range: Range, count: usize) -> Range {
    word_move(slice, range, count, WordMotionTarget::PrevLongWordEnd)
}

pub fn move_next_sub_word_start(slice: RopeSlice, range: Range, count: usize) -> Range {
    word_move(slice, range, count, WordMotionTarget::NextSubWordStart)
}

pub fn move_next_sub_word_end(slice: RopeSlice, range: Range, count: usize) -> Range {
    word_move(slice, range, count, WordMotionTarget::NextSubWordEnd)
}

pub fn move_prev_sub_word_start(slice: RopeSlice, range: Range, count: usize) -> Range {
    word_move(slice, range, count, WordMotionTarget::PrevSubWordStart)
}

pub fn move_prev_sub_word_end(slice: RopeSlice, range: Range, count: usize) -> Range {
    word_move(slice, range, count, WordMotionTarget::PrevSubWordEnd)
}

pub fn word_move(slice: RopeSlice, range: Range, count: usize, target: WordMotionTarget) -> Range {
    let prev = matches!(
        target,
        WordMotionTarget::PrevWordStart
            | WordMotionTarget::PrevWordEnd
            | WordMotionTarget::PrevLongWordStart
            | WordMotionTarget::PrevLongWordEnd
            | WordMotionTarget::PrevSubWordStart
            | WordMotionTarget::PrevSubWordEnd
    );
    let len = slice.len_chars();

    // Early exit if at the start/end.
    if (prev && range.head == 0) || (!prev && range.head == len) {
        return range;
    }

    let start_head = if prev {
        if range.anchor < range.head {
            range.head.saturating_sub(1)
        } else {
            range.head.min(len.saturating_sub(1))
        }
    } else {
        if range.anchor < range.head {
            range.head.saturating_sub(1)
        } else {
            range.head.saturating_add(1).min(len)
        }
    };

    let mut range = Range {
        anchor: range.anchor,
        head: start_head,
    };

    for _ in 0..count {
        range = range_to_target(&mut slice.chars_at(range.head), target, range);
    }
    return range;
}

use anyhow::{Ok, Result};
use std::cmp::min;

use crate::{buffer::Mode, editor::Editor};

pub fn move_left(ed: &mut Editor) -> Result<()> {
    ed.focused_buf.cursor.col = ed.focused_buf.cursor.col.saturating_sub(1);
    Ok(())
}

pub fn move_right(ed: &mut Editor) -> Result<()> {
    ed.focused_buf.cursor.col = min(
        ed.focused_buf.cursor.col + 1,
        ed.focused_buf.line_len(ed.focused_buf.cursor.row),
    );
    Ok(())
}

pub fn set_mode(ed: &mut Editor, mode: Mode) -> Result<()> {
    ed.focused_buf.mode = mode;
    Ok(())
}

pub fn insert_char(ed: &mut Editor, ch: char) -> Result<()> {
    ed.focused_buf.lines[ed.focused_buf.cursor.row].insert(ed.focused_buf.cursor.col, ch);
    ed.focused_buf.cursor.col += 1;
    Ok(())
}

pub fn move_up(ed: &mut Editor) -> Result<()> {
    ed.focused_buf.cursor.row = ed.focused_buf.cursor.row.saturating_sub(1);
    ed.focused_buf.cursor.col = min(
        ed.focused_buf.cursor.col,
        ed.focused_buf.line_len(ed.focused_buf.cursor.row),
    );
    Ok(())
}

pub fn move_down(ed: &mut Editor) -> Result<()> {
    ed.focused_buf.cursor.row = min(
        ed.focused_buf.cursor.row + 1,
        ed.focused_buf.line_count() - 1,
    );
    ed.focused_buf.cursor.col = min(
        ed.focused_buf.cursor.col,
        ed.focused_buf.line_len(ed.focused_buf.cursor.row),
    );
    Ok(())
}

pub fn beginning_of_line(ed: &mut Editor) -> Result<()> {
    ed.focused_buf.cursor.col = 0;
    Ok(())
}

pub fn start_of_line(ed: &mut Editor) -> Result<()> {
    let line = &ed.focused_buf.lines[ed.focused_buf.cursor.row];
    let mut i = 0;
    while i < line.len() && line.as_bytes()[i].is_ascii_whitespace() {
        i += 1;
    }
    ed.focused_buf.cursor.col = min(i, line.len());
    Ok(())
}

pub fn end_of_line(ed: &mut Editor) -> Result<()> {
    ed.focused_buf.cursor.col = ed.focused_buf.line_len(ed.focused_buf.cursor.row);
    Ok(())
}

pub fn word_forward(ed: &mut Editor) -> Result<()> {
    let line = &ed.focused_buf.lines[ed.focused_buf.cursor.row];
    let mut i = ed.focused_buf.cursor.col;
    if i < line.len() {
        i += 1;
    }
    while i < line.len() && line.as_bytes()[i].is_ascii_whitespace() {
        i += 1;
    }
    while i < line.len() && !line.as_bytes()[i].is_ascii_whitespace() {
        i += 1;
    }
    ed.focused_buf.cursor.col = min(i, line.len());
    Ok(())
}
pub fn new_line_below(ed: &mut Editor) -> Result<()> {
    let i = ed.focused_buf.lines[ed.focused_buf.cursor.row].len();
    let rest = ed.focused_buf.lines[ed.focused_buf.cursor.row].split_off(i);
    ed.focused_buf
        .lines
        .insert(ed.focused_buf.cursor.row + 1, rest);
    ed.focused_buf.cursor.row += 1;
    ed.focused_buf.cursor.col = 0;
    Ok(())
}

pub fn new_line_above(ed: &mut Editor) -> Result<()> {
    ed.focused_buf
        .lines
        .insert(ed.focused_buf.cursor.row, String::new());
    ed.focused_buf.cursor.col = 0;
    Ok(())
}

pub fn word_backward(ed: &mut Editor) -> Result<()> {
    let line = &ed.focused_buf.lines[ed.focused_buf.cursor.row];
    let mut i = ed.focused_buf.cursor.col;
    if i > 0 {
        i -= 1;
    }
    while i > 0 && line.as_bytes()[i].is_ascii_whitespace() {
        i -= 1;
    }
    while i > 0 && !line.as_bytes()[i - 1].is_ascii_whitespace() {
        i -= 1;
    }
    ed.focused_buf.cursor.col = i;
    Ok(())
}

pub fn word_end(ed: &mut Editor) -> Result<()> {
    let line = &ed.focused_buf.lines[ed.focused_buf.cursor.row];
    let mut i = ed.focused_buf.cursor.col;
    while i < line.len() && line.as_bytes()[i].is_ascii_whitespace() {
        i += 1;
    }
    while i < line.len() {
        if i + 1 >= line.len() || line.as_bytes()[i + 1].is_ascii_whitespace() {
            break;
        }
        i += 1;
    }
    ed.focused_buf.cursor.col = min(i, line.len());
    Ok(())
}

use anyhow::{Ok, Result};
use std::cmp::min;

use crate::{buffer::Mode, editor::Editor};

pub fn move_left(ed: &mut Editor) -> Result<()> {
    let buf = ed.focused_buf_mut();
    buf.cursor.col = buf.cursor.col.saturating_sub(1);
    Ok(())
}

pub fn move_right(ed: &mut Editor) -> Result<()> {
    let buf = ed.focused_buf_mut();
    buf.cursor.col = min(buf.cursor.col + 1, buf.line_len(buf.cursor.row));
    Ok(())
}

pub fn set_mode(ed: &mut Editor, mode: Mode) -> Result<()> {
    ed.focused_buf_mut().mode = mode;
    Ok(())
}

pub fn insert_char(ed: &mut Editor, ch: char) -> Result<()> {
    let buf = ed.focused_buf_mut();
    buf.lines[buf.cursor.row].insert(buf.cursor.col, ch);
    buf.cursor.col += 1;
    Ok(())
}

pub fn move_up(ed: &mut Editor) -> Result<()> {
    let buf = ed.focused_buf_mut();
    buf.cursor.row = buf.cursor.row.saturating_sub(1);
    buf.cursor.col = min(buf.cursor.col, buf.line_len(buf.cursor.row));
    Ok(())
}

pub fn move_down(ed: &mut Editor) -> Result<()> {
    let buf = ed.focused_buf_mut();
    buf.cursor.row = min(buf.cursor.row + 1, buf.line_count() - 1);
    buf.cursor.col = min(buf.cursor.col, buf.line_len(buf.cursor.row));
    Ok(())
}

pub fn beginning_of_line(ed: &mut Editor) -> Result<()> {
    ed.focused_buf_mut().cursor.col = 0;
    Ok(())
}

pub fn start_of_line(ed: &mut Editor) -> Result<()> {
    let line = &ed.focused_buf().lines[ed.focused_buf().cursor.row];
    let mut i = 0;
    while i < line.len() && line.as_bytes()[i].is_ascii_whitespace() {
        i += 1;
    }
    ed.focused_buf_mut().cursor.col = min(i, line.len());
    Ok(())
}

pub fn end_of_line(ed: &mut Editor) -> Result<()> {
    let buf = ed.focused_buf_mut();
    buf.cursor.col = buf.line_len(buf.cursor.row);
    Ok(())
}

pub fn word_forward(ed: &mut Editor) -> Result<()> {
    let buf = ed.focused_buf_mut();
    let line = &buf.lines[buf.cursor.row];
    let mut i = buf.cursor.col;
    if i < line.len() {
        i += 1;
    }
    while i < line.len() && line.as_bytes()[i].is_ascii_whitespace() {
        i += 1;
    }
    while i < line.len() && !line.as_bytes()[i].is_ascii_whitespace() {
        i += 1;
    }
    buf.cursor.col = min(i, line.len());
    Ok(())
}
pub fn new_line_below(ed: &mut Editor) -> Result<()> {
    let buf = ed.focused_buf_mut();
    let i = buf.lines[buf.cursor.row].len();
    let rest = buf.lines[buf.cursor.row].split_off(i);
    buf.lines.insert(buf.cursor.row + 1, rest);
    buf.cursor.row += 1;
    buf.cursor.col = 0;
    Ok(())
}

pub fn new_line_above(ed: &mut Editor) -> Result<()> {
    let buf = ed.focused_buf_mut();
    buf.lines.insert(buf.cursor.row, String::new());
    buf.cursor.col = 0;
    Ok(())
}

pub fn word_backward(ed: &mut Editor) -> Result<()> {
    let buf = ed.focused_buf_mut();
    let line = &buf.lines[buf.cursor.row];
    let mut i = buf.cursor.col;
    if i > 0 {
        i -= 1;
    }
    while i > 0 && line.as_bytes()[i].is_ascii_whitespace() {
        i -= 1;
    }
    while i > 0 && !line.as_bytes()[i - 1].is_ascii_whitespace() {
        i -= 1;
    }
    buf.cursor.col = i;
    Ok(())
}

pub fn word_end(ed: &mut Editor) -> Result<()> {
    let buf = ed.focused_buf_mut();
    let line = &buf.lines[buf.cursor.row];
    let mut i = buf.cursor.col;
    while i < line.len() && line.as_bytes()[i].is_ascii_whitespace() {
        i += 1;
    }
    while i < line.len() {
        if i + 1 >= line.len() || line.as_bytes()[i + 1].is_ascii_whitespace() {
            break;
        }
        i += 1;
    }
    buf.cursor.col = min(i, line.len());
    Ok(())
}

use anyhow::{Ok, Result};
use crossterm::QueueableCommand;
use crossterm::terminal::{Clear, ClearType, size};
use crossterm::{
    cursor::{MoveTo, SetCursorStyle, Show},
    style::Print,
};
use std::cmp::min;
use std::io::Write;

use crate::editor::{Editor, Mode};
pub fn render(editor: &mut Editor) -> Result<bool> {
    let (w, h) = size()?;
    editor.ensure_cursor_on_screen(w, h);
    let mut out = std::io::stdout();
    let _ = out.queue(Clear(ClearType::All));

    let text_rows = h.saturating_sub(1) as usize;
    for row in 0..text_rows {
        let buf_row = editor.top + row;
        if buf_row >= editor.buf.line_count() {
            break;
        }
        let line = &editor.buf.lines[buf_row];
        let visible = if editor.left < line.len() {
            &line[editor.left..min(line.len(), editor.left + w as usize)]
        } else {
            ""
        };
        out.queue(MoveTo(0, row as u16))?;
        out.queue(Print(visible))?;
    }

    out.queue(MoveTo(0, h - 1))?;
    // let mode = match editor.mode {
    //     Mode::Normal => "NORMAL",
    //     Mode::Insert => "INSERT",
    //     Mode::Visual => "VISUAL",
    // };
    let status = format!("Deneme");
    out.queue(Print(status))?;

    let cy = (editor.cursor.row) as u16;
    let cx = (editor.cursor.col) as u16;
    out.queue(MoveTo(cx, cy))?;
    match editor.mode {
        Mode::Normal => {
            out.queue(SetCursorStyle::SteadyBlock)?;
        }
        Mode::Insert => {
            out.queue(SetCursorStyle::SteadyBar)?;
        }
        Mode::Visual => {
            out.queue(SetCursorStyle::SteadyBlock)?;
        }
    }
    out.queue(Show)?;

    let _ = out.flush();

    Ok(true)
}

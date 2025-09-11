use anyhow::{Ok, Result};
use crossterm::QueueableCommand;
use crossterm::terminal::{Clear, ClearType, size};
use crossterm::{
    cursor::{MoveTo, SetCursorStyle, Show},
    style::Print,
};
use std::cmp::min;
use std::io::Write;

use crossterm::ExecutableCommand;
use crossterm::cursor::Hide;
use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, read,
};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use std::io::stdout;

use crate::buffer::Mode;
use crate::commands;
use crate::editor::Editor;
use crate::keymap::Keymap;

pub fn run() -> Result<()> {
    let mut keymap = Keymap::default();
    let mut ed = Editor::with_text(
        "Hello, Vim‑ish world!\nThis is a demo buffer.\nUse hjkl, wbe, 0,$, gg/G, i/a/o/O, x, dd, yy, p, u, Ctrl‑r.",
    );

    enable_raw_mode()?;
    let mut out = stdout();
    out.execute(EnterAlternateScreen)?;
    out.execute(EnableMouseCapture)?;
    out.execute(Hide)?;

    let result = (|| -> Result<()> {
        loop {
            render(&mut ed)?;
            match read()? {
                Event::Key(key) => {
                    if key.code == KeyCode::Esc
                        && ed.focused_buf.mode == Mode::Normal
                        && key.modifiers.contains(KeyModifiers::SHIFT)
                    {
                        break;
                    }
                    if key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        break;
                    }

                    let result = keymap.execute(&mut ed, key);

                    if result.is_ok_and(|a| a == false) {
                        if ed.focused_buf.mode == Mode::Insert && key.code.as_char().is_some() {
                            let _ = commands::insert_char(&mut ed, key.code.as_char().unwrap());
                            continue;
                        }
                    }
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    })();

    disable_raw_mode()?;
    out.execute(SetCursorStyle::DefaultUserShape)?;
    out.execute(Show)?;
    out.execute(DisableMouseCapture)?;
    out.execute(LeaveAlternateScreen)?;

    result
}

pub fn render(editor: &mut Editor) -> Result<bool> {
    let (w, h) = size()?;
    editor.focused_buf.ensure_cursor_on_screen(w, h);
    let mut out = std::io::stdout();
    let _ = out.queue(Clear(ClearType::All));

    let text_rows = h.saturating_sub(1) as usize;
    for row in 0..text_rows {
        let buf_row = editor.focused_buf.top + row;
        if buf_row >= editor.focused_buf.line_count() {
            break;
        }
        let line = &editor.focused_buf.lines[buf_row];
        let visible = if editor.focused_buf.left < line.len() {
            &line[editor.focused_buf.left..min(line.len(), editor.focused_buf.left + w as usize)]
        } else {
            ""
        };
        out.queue(MoveTo(0, row as u16))?;
        out.queue(Print(visible))?;
    }

    out.queue(MoveTo(0, h - 1))?;
    out.queue(Print(editor.status_line()))?;

    let cy = (editor.focused_buf.cursor.row) as u16;
    let cx = (editor.focused_buf.cursor.col) as u16;
    out.queue(MoveTo(cx, cy))?;
    match editor.focused_buf.mode {
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

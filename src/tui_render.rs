use anyhow::Result;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{Hide, MoveTo, SetCursorStyle, Show},
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, read},
    style::Print,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, size,
    },
};

use std::cmp::min;
use std::io::{Write, stdout};

use crate::editor::{Editor, EditorState};
use crate::{buffer::Mode, command::command::CommandContext};

pub fn run() -> Result<()> {
    // Initialize editor + state
    let editor = Editor::new();

    enable_raw_mode()?;
    let mut out = stdout();
    out.execute(EnterAlternateScreen)?;
    out.execute(EnableMouseCapture)?;
    out.execute(Hide)?;

    let result = (|| -> Result<()> {
        loop {
            render(&mut editor.state.lock().unwrap())?;

            match read()? {
                Event::Key(key_event) => {
                    if key_event.code == KeyCode::Char('c')
                        && key_event.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        break;
                    }

                    let mut state = editor.state.lock().unwrap();
                    if key_event.code == KeyCode::Esc
                        && state.focused_buf().mode == Mode::Normal
                        && key_event.modifiers.contains(KeyModifiers::SHIFT)
                    {
                        break;
                    }
                    let executed = editor.handle_key(&mut state, key_event);
                    if executed.is_err() {
                        let buf = state.focused_buf_mut();
                        match buf.mode {
                            Mode::Insert => {
                                if let Some(c) = key_event.code.as_char() {
                                    buf.insert_char(c);
                                }
                            }
                            Mode::Command => match key_event.code {
                                KeyCode::Char(c) => state.command_buffer.push(c),
                                KeyCode::Backspace => {
                                    state.command_buffer.pop();
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
                Event::Resize(_, _) => {
                    // optional: handle resize
                }
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

pub fn render(state: &mut EditorState) -> Result<()> {
    let status_line = state.status_line();
    let command_line = state.command_buffer.clone();

    let buf = state.focused_buf_mut();
    let (term_w, term_h) = size()?;

    buf.ensure_cursor_on_screen(term_w, term_h);

    let mut out = stdout();
    out.queue(Clear(ClearType::All))?;

    let text_rows = term_h.saturating_sub(1) as usize;
    for row in 0..text_rows {
        let buf_row = buf.top + row;
        if buf_row >= buf.line_count() {
            break;
        }
        let line = &buf.lines[buf_row];
        let visible = if buf.left < line.len() {
            &line[buf.left..min(line.len(), buf.left + term_w as usize)]
        } else {
            ""
        };
        out.queue(MoveTo(0, row as u16))?;
        out.queue(Print(visible))?;
    }

    out.queue(MoveTo(0, term_h - 1))?;
    if buf.mode == Mode::Command {
        out.queue(Print(format!(":{}", command_line)))?;
    } else {
        out.queue(Print(status_line))?;
    }

    if buf.mode == Mode::Command {
        let cy = term_h.saturating_sub(1);
        let cx = (1 + command_line.len()) as u16;
        out.queue(MoveTo(cx, cy))?;
    } else {
        let cy = buf.cursor.row as u16;
        let cx = buf.cursor.col as u16;
        out.queue(MoveTo(cx, cy))?;
    }

    match buf.mode {
        Mode::Normal | Mode::Visual | Mode::Command => {
            out.queue(SetCursorStyle::SteadyBlock)?;
        }
        Mode::Insert => {
            out.queue(SetCursorStyle::SteadyBar)?;
        }
    }
    out.queue(Show)?;
    out.flush()?;

    Ok(())
}

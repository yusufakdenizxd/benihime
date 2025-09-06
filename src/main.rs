mod buffer;
mod editor;
mod input;
mod tui_render;

use anyhow::{Ok, Result};
use crossterm::ExecutableCommand;
use crossterm::cursor::{Hide, SetCursorStyle, Show};
use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, read,
};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use editor::{Editor, Mode};
use std::io::stdout;
use tui_render::render;

fn main() -> Result<()> {
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
                        && ed.mode == Mode::Normal
                        && key.modifiers.contains(KeyModifiers::SHIFT)
                    {
                        break;
                    }
                    if key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        break;
                    }

                    match ed.mode {
                        Mode::Normal => input::handle_normal(&mut ed, key),
                        Mode::Insert => input::handle_insert(&mut ed, key),
                        Mode::Visual => {
                            if let KeyCode::Esc = key.code {
                                ed.mode = Mode::Normal;
                            } else { /* TODO: selections */
                            }
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

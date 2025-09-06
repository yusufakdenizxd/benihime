use anyhow::{Ok, Result};
use crossterm::cursor::{Hide, MoveTo, SetCursorStyle, Show};
use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers, read,
};
use crossterm::style::Print;
use crossterm::terminal::{
    Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
    enable_raw_mode, size,
};
use crossterm::{ExecutableCommand, QueueableCommand};
use std::cmp::min;
use std::io::{Write, stdout};

#[derive(Debug, Clone)]
struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    fn new() -> Self {
        Self {
            lines: vec![String::new()],
        }
    }
    fn from(text: &str) -> Self {
        Self {
            lines: text.split('\n').map(|s| s.to_string()).collect(),
        }
    }
    fn line_count(&self) -> usize {
        self.lines.len()
    }
    fn line_len(&self, row: usize) -> usize {
        self.lines.get(row).map(|l| l.len()).unwrap_or(0)
    }
}

#[derive(Clone)]
struct Cursor {
    row: usize,
    col: usize,
}

impl Cursor {
    fn new() -> Self {
        Self { row: 0, col: 0 }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Mode {
    Normal,
    Insert,
    Visual,
}

#[derive(Clone)]
struct Editor {
    buf: Buffer,
    cursor: Cursor,
    mode: Mode,
    top: usize,
    left: usize,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            buf: Buffer::new(),
            cursor: Cursor::new(),
            mode: Mode::Normal,
            top: 0,
            left: 0,
        }
    }

    fn with_text(text: &str) -> Self {
        Self {
            buf: Buffer::from(text),
            ..Self::new()
        }
    }

    fn move_left(&mut self) {
        self.cursor.col = self.cursor.col.saturating_sub(1);
    }

    fn move_right(&mut self) {
        self.cursor.col = min(self.cursor.col + 1, self.buf.line_len(self.cursor.row));
    }

    fn insert_char(&mut self, ch: char) {
        self.buf.lines[self.cursor.row].insert(self.cursor.col, ch);
        self.cursor.col += 1;
    }

    fn move_up(&mut self) {
        self.cursor.row = self.cursor.row.saturating_sub(1);
        self.cursor.col = min(self.cursor.col, self.buf.line_len(self.cursor.row));
    }

    fn move_down(&mut self) {
        self.cursor.row = min(self.cursor.row + 1, self.buf.line_count() - 1);
        self.cursor.col = min(self.cursor.col, self.buf.line_len(self.cursor.row));
    }

    fn ensure_cursor_on_screen(&mut self, width: u16, height: u16) {
        let h = height as usize - 1;
        if self.cursor.row < self.top {
            self.top = self.cursor.row;
        }
        if self.cursor.row >= self.top + h {
            self.top = self.cursor.row + 1 - h;
        }

        if self.cursor.col < self.left {
            self.left = self.cursor.col;
        }
        if self.cursor.col >= self.left + width as usize {
            self.left = self.cursor.col + 1 - width as usize;
        }
    }
}

fn render(editor: &mut Editor) -> Result<bool> {
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

fn handle_normal(ed: &mut Editor, key: KeyEvent) {
    match key.code {
        KeyCode::Char('h') => ed.move_left(),
        KeyCode::Char('j') => ed.move_down(),
        KeyCode::Char('k') => ed.move_up(),
        KeyCode::Char('l') => ed.move_right(),
        KeyCode::Char('i') => {
            ed.mode = Mode::Insert;
        }
        KeyCode::Char('a') => {
            ed.move_right();
            ed.mode = Mode::Insert;
        }
        KeyCode::Char('v') => {
            ed.mode = Mode::Visual;
        } // placeholder
        _ => {}
    }
}

fn handle_insert(ed: &mut Editor, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => ed.mode = Mode::Normal,
        KeyCode::Char(c) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) { /* ignore ctrl chars in insert */
            } else {
                ed.insert_char(c);
            }
        }
        _ => {}
    }
}

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
                        Mode::Normal => handle_normal(&mut ed, key),
                        Mode::Insert => handle_insert(&mut ed, key),
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

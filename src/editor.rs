use std::cmp::min;

use crate::buffer::Buffer;

#[derive(Clone)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

impl Cursor {
    fn new() -> Cursor {
        Cursor { row: 0, col: 0 }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
}

#[derive(Clone)]
pub struct Editor {
    pub buf: Buffer,
    pub cursor: Cursor,
    pub mode: Mode,
    pub top: usize,
    pub left: usize,
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

    pub fn with_text(text: &str) -> Self {
        Self {
            buf: Buffer::from(text),
            ..Self::new()
        }
    }

    pub fn move_left(&mut self) {
        self.cursor.col = self.cursor.col.saturating_sub(1);
    }

    pub fn move_right(&mut self) {
        self.cursor.col = min(self.cursor.col + 1, self.buf.line_len(self.cursor.row));
    }

    pub fn insert_char(&mut self, ch: char) {
        self.buf.lines[self.cursor.row].insert(self.cursor.col, ch);
        self.cursor.col += 1;
    }

    pub fn move_up(&mut self) {
        self.cursor.row = self.cursor.row.saturating_sub(1);
        self.cursor.col = min(self.cursor.col, self.buf.line_len(self.cursor.row));
    }

    pub fn move_down(&mut self) {
        self.cursor.row = min(self.cursor.row + 1, self.buf.line_count() - 1);
        self.cursor.col = min(self.cursor.col, self.buf.line_len(self.cursor.row));
    }

    pub fn beginning_of_line(&mut self) {
        self.cursor.col = 0;
    }

    pub fn start_of_line(&mut self) {
        let line = &self.buf.lines[self.cursor.row];
        let mut i = 0;
        while i < line.len() && line.as_bytes()[i].is_ascii_whitespace() {
            i += 1;
        }
        self.cursor.col = min(i, line.len());
    }

    pub fn end_of_line(&mut self) {
        self.cursor.col = self.buf.line_len(self.cursor.row);
    }

    pub fn word_forward(&mut self) {
        let line = &self.buf.lines[self.cursor.row];
        let mut i = self.cursor.col;
        if i < line.len() {
            i += 1;
        }
        while i < line.len() && line.as_bytes()[i].is_ascii_whitespace() {
            i += 1;
        }
        while i < line.len() && !line.as_bytes()[i].is_ascii_whitespace() {
            i += 1;
        }
        self.cursor.col = min(i, line.len());
    }

    pub fn word_backward(&mut self) {
        let line = &self.buf.lines[self.cursor.row];
        let mut i = self.cursor.col;
        if i > 0 {
            i -= 1;
        }
        while i > 0 && line.as_bytes()[i].is_ascii_whitespace() {
            i -= 1;
        }
        while i > 0 && !line.as_bytes()[i - 1].is_ascii_whitespace() {
            i -= 1;
        }
        self.cursor.col = i;
    }

    pub fn word_end(&mut self) {
        let line = &self.buf.lines[self.cursor.row];
        let mut i = self.cursor.col;
        while i < line.len() && line.as_bytes()[i].is_ascii_whitespace() {
            i += 1;
        }
        while i < line.len() {
            if i + 1 >= line.len() || line.as_bytes()[i + 1].is_ascii_whitespace() {
                break;
            }
            i += 1;
        }
        self.cursor.col = min(i, line.len());
    }

    pub fn ensure_cursor_on_screen(&mut self, width: u16, height: u16) {
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

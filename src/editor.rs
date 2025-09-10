use std::cmp::min;

use crate::buffer::{Buffer, Mode};

#[derive(Clone)]
pub struct Editor {
    pub focused_buf: Buffer,
    pub buffers: Vec<Buffer>,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            focused_buf: Buffer::new(1),
            buffers: vec![],
        }
    }

    pub fn status_line(&self) -> String {
        let mode = match self.focused_buf.mode {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
        };
        format!("{} {}", mode, self.focused_buf.id)
    }

    pub fn with_text(text: &str) -> Self {
        Self {
            focused_buf: Buffer::from(1, text),
            ..Self::new()
        }
    }

    pub fn move_left(&mut self) {
        self.focused_buf.cursor.col = self.focused_buf.cursor.col.saturating_sub(1);
    }

    pub fn move_right(&mut self) {
        self.focused_buf.cursor.col = min(
            self.focused_buf.cursor.col + 1,
            self.focused_buf.line_len(self.focused_buf.cursor.row),
        );
    }

    pub fn insert_char(&mut self, ch: char) {
        self.focused_buf.lines[self.focused_buf.cursor.row].insert(self.focused_buf.cursor.col, ch);
        self.focused_buf.cursor.col += 1;
    }

    pub fn move_up(&mut self) {
        self.focused_buf.cursor.row = self.focused_buf.cursor.row.saturating_sub(1);
        self.focused_buf.cursor.col = min(
            self.focused_buf.cursor.col,
            self.focused_buf.line_len(self.focused_buf.cursor.row),
        );
    }

    pub fn move_down(&mut self) {
        self.focused_buf.cursor.row = min(
            self.focused_buf.cursor.row + 1,
            self.focused_buf.line_count() - 1,
        );
        self.focused_buf.cursor.col = min(
            self.focused_buf.cursor.col,
            self.focused_buf.line_len(self.focused_buf.cursor.row),
        );
    }

    pub fn beginning_of_line(&mut self) {
        self.focused_buf.cursor.col = 0;
    }

    pub fn start_of_line(&mut self) {
        let line = &self.focused_buf.lines[self.focused_buf.cursor.row];
        let mut i = 0;
        while i < line.len() && line.as_bytes()[i].is_ascii_whitespace() {
            i += 1;
        }
        self.focused_buf.cursor.col = min(i, line.len());
    }

    pub fn end_of_line(&mut self) {
        self.focused_buf.cursor.col = self.focused_buf.line_len(self.focused_buf.cursor.row);
    }

    pub fn word_forward(&mut self) {
        let line = &self.focused_buf.lines[self.focused_buf.cursor.row];
        let mut i = self.focused_buf.cursor.col;
        if i < line.len() {
            i += 1;
        }
        while i < line.len() && line.as_bytes()[i].is_ascii_whitespace() {
            i += 1;
        }
        while i < line.len() && !line.as_bytes()[i].is_ascii_whitespace() {
            i += 1;
        }
        self.focused_buf.cursor.col = min(i, line.len());
    }
    pub fn new_line_below(&mut self) {
        let i = self.focused_buf.lines[self.focused_buf.cursor.row].len();
        let rest = self.focused_buf.lines[self.focused_buf.cursor.row].split_off(i);
        self.focused_buf
            .lines
            .insert(self.focused_buf.cursor.row + 1, rest);
        self.focused_buf.cursor.row += 1;
        self.focused_buf.cursor.col = 0;
    }

    pub fn new_line_above(&mut self) {
        self.focused_buf
            .lines
            .insert(self.focused_buf.cursor.row, String::new());
        self.focused_buf.cursor.col = 0;
    }

    pub fn word_backward(&mut self) {
        let line = &self.focused_buf.lines[self.focused_buf.cursor.row];
        let mut i = self.focused_buf.cursor.col;
        if i > 0 {
            i -= 1;
        }
        while i > 0 && line.as_bytes()[i].is_ascii_whitespace() {
            i -= 1;
        }
        while i > 0 && !line.as_bytes()[i - 1].is_ascii_whitespace() {
            i -= 1;
        }
        self.focused_buf.cursor.col = i;
    }

    pub fn word_end(&mut self) {
        let line = &self.focused_buf.lines[self.focused_buf.cursor.row];
        let mut i = self.focused_buf.cursor.col;
        while i < line.len() && line.as_bytes()[i].is_ascii_whitespace() {
            i += 1;
        }
        while i < line.len() {
            if i + 1 >= line.len() || line.as_bytes()[i + 1].is_ascii_whitespace() {
                break;
            }
            i += 1;
        }
        self.focused_buf.cursor.col = min(i, line.len());
    }
}

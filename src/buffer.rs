use anyhow::anyhow;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Clone)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

impl Cursor {
    fn new() -> Cursor {
        Cursor { row: 0, col: 0 }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    Command,
}

impl FromStr for Mode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(Mode::Normal),
            "insert" => Ok(Mode::Insert),
            "visual" => Ok(Mode::Visual),
            "command" => Ok(Mode::Command),
            _ => Err(anyhow!("Invalid mode: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub id: i32,
    pub lines: Vec<String>,
    pub name: String,
    pub cursor: Cursor,
    pub scroll_offset: usize,
    pub mode: Mode,
    pub top: usize,
    pub left: usize,
    pub file_path: Option<PathBuf>,
}

impl Buffer {
    pub fn new(id: i32, name: &str, file_path: Option<PathBuf>) -> Self {
        Self {
            id,
            name: name.to_string(),
            lines: vec![String::new()],
            cursor: Cursor::new(),
            mode: Mode::Normal,
            top: 0,
            left: 0,
            file_path,
            scroll_offset: 0,
        }
    }

    pub fn from(id: i32, name: &str, text: &str, file_path: Option<PathBuf>) -> Self {
        Self {
            id,
            name: name.to_string(),
            lines: text.split('\n').map(|s| s.to_string()).collect(),
            cursor: Cursor::new(),
            mode: Mode::Normal,
            top: 0,
            left: 0,
            file_path,
            scroll_offset: 0,
        }
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
    pub fn line_len(&self, row: usize) -> usize {
        self.lines.get(row).map(|l| l.len()).unwrap_or(0)
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
    pub fn insert_char(&mut self, c: char) {
        let row = self.cursor.row;
        let col = self.cursor.col;

        if row >= self.lines.len() {
            self.lines.push(String::new());
        }

        let line = &mut self.lines[row];
        if col <= line.len() {
            line.insert(col, c);
            self.cursor.col += 1;
        } else {
            line.push(c);
            self.cursor.col = line.len();
        }
    }

    pub fn insert_str(&mut self, s: &str) {
        for ch in s.chars() {
            self.insert_char(ch);
        }
    }

    pub fn delete_char_before_cursor(&mut self) {
        if self.cursor.col > 0 {
            let row = self.cursor.row;
            let col = self.cursor.col;
            if let Some(line) = self.lines.get_mut(row) {
                line.remove(col - 1);
                self.cursor.col -= 1;
            }
        } else if self.cursor.row > 0 {
            let row = self.cursor.row;
            let col = self.lines[row - 1].len();
            let current_line = self.lines.remove(row);
            self.cursor.row -= 1;
            self.cursor.col = col;
            self.lines[self.cursor.row].push_str(&current_line);
        }
    }

    pub fn update_scroll(&mut self, screen_height: usize, scrolloff: usize) {
        let row = self.cursor.row;

        // If cursor is too high
        if row < self.scroll_offset + scrolloff {
            self.scroll_offset = row.saturating_sub(scrolloff);
        }

        // If cursor is too low
        if row >= self.scroll_offset + screen_height - scrolloff {
            self.scroll_offset = row + scrolloff + 1 - screen_height;
        }
    }

    pub fn center_cursor(&mut self, screen_height: usize) {
        let cursor_row = self.cursor.row.min(self.lines.len().saturating_sub(1));

        let half_screen = screen_height / 2;
        if cursor_row >= half_screen {
            self.scroll_offset = cursor_row - half_screen;
        } else {
            self.scroll_offset = 0;
        }

        if self.scroll_offset + screen_height > self.lines.len() {
            self.scroll_offset = self.lines.len().saturating_sub(screen_height);
        }
    }
}

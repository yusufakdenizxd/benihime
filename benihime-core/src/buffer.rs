use anyhow::anyhow;
use ropey::{Rope, RopeSlice, iter::Lines};
use std::{path::PathBuf, str::FromStr};

use crate::{
    movement::selection::Range,
    undotree::{Edit, UndoTree},
};

#[derive(Debug, Clone, PartialEq, Eq, Copy, PartialOrd)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

impl Cursor {
    fn new() -> Cursor {
        Cursor { row: 0, col: 0 }
    }

    pub fn start() -> Cursor {
        Cursor { row: 0, col: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct Selection {
    pub start: Cursor,
}

impl Selection {
    pub fn normalized(&self, end: &Cursor) -> (Cursor, Cursor) {
        if self.start.row < end.row || self.start.row == end.row && self.start.col <= end.col {
            (self.start, *end)
        } else {
            (*end, self.start)
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    Command,
    Minibuffer,
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
    lines: Rope,
    pub name: String,
    pub cursor: Cursor,
    pub scroll_offset: usize,
    pub scroll_left: usize,
    pub mode: Mode,
    pub file_path: Option<PathBuf>,
    pub selection: Option<Selection>,
    pub range: Option<Range>,
    undo_tree: UndoTree,
    undo_recording: bool,
}

impl Buffer {
    pub fn new(id: i32, name: &str, file_path: Option<PathBuf>) -> Self {
        Self {
            id,
            name: name.to_string(),
            lines: Rope::new(),
            cursor: Cursor::new(),
            mode: Mode::Normal,
            file_path,
            scroll_offset: 0,
            scroll_left: 0,
            selection: None,
            range: None,
            undo_tree: UndoTree::new(),
            undo_recording: true,
        }
    }

    pub fn from(id: i32, name: &str, text: &str, file_path: Option<PathBuf>) -> Self {
        Self {
            id,
            name: name.to_string(),
            lines: Rope::from_str(text),
            cursor: Cursor::new(),
            mode: Mode::Normal,
            file_path,
            scroll_offset: 0,
            scroll_left: 0,
            selection: None,
            range: None,
            undo_tree: UndoTree::new(),
            undo_recording: true,
        }
    }

    pub fn to_string(&self) -> String {
        self.lines.to_string()
    }

    pub fn line_count(&self) -> usize {
        self.lines.len_lines()
    }

    pub fn line(&self, row: usize) -> RopeSlice<'_> {
        self.lines.line(row)
    }

    pub fn lines_at(&self, index: usize) -> Lines<'_> {
        self.lines.lines_at(index)
    }

    pub fn get_line_to_char(&self, row: usize) -> usize {
        self.lines.line_to_char(row)
    }

    pub fn get_slice(&self, range: std::ops::Range<usize>) -> RopeSlice<'_> {
        self.lines.slice(range)
    }

    pub fn insert_idx(&mut self, idx: usize, text: &str) {
        self.insert(idx, text);
    }

    pub fn remove_line(&mut self, start: usize, end: usize) {
        self.remove(start..end)
    }

    pub fn get_cursor_to_char(&self) -> usize {
        self.lines.line_to_char(self.cursor.row)
    }

    pub fn line_len(&self, row: usize) -> usize {
        if row >= self.lines.len_lines() {
            return 0;
        }
        let line = self.lines.line(row);
        let len_chars = line.len_chars();
        if len_chars == 0 {
            return 0;
        }

        let last_char = line.char(len_chars - 1);
        if last_char == '\n' {
            if len_chars > 1 && line.char(len_chars - 2) == '\r' {
                len_chars - 2
            } else {
                len_chars - 1
            }
        } else {
            len_chars
        }
    }

    pub fn insert_char(&mut self, c: char) {
        let mut s = String::new();
        s.push(c);
        self.insert_str(&s);
    }

    pub fn insert_str(&mut self, s: &str) {
        let row = self.cursor.row;
        let col = self.cursor.col;

        if row >= self.lines.len_lines() {
            return;
        }

        let line_len = self.line_len(row);
        let insert_col = col.min(line_len);
        let char_idx = self.lines.line_to_char(row) + insert_col;

        if self.undo_recording {
            self.undo_tree.record(Edit::Insert {
                at: char_idx,
                text: s.to_string(),
            });
        }

        self.insert(char_idx, s);

        let mut newlines = 0;
        let mut last_line_len = 0;

        for ch in s.chars() {
            if ch == '\n' {
                newlines += 1;
                last_line_len = 0;
            } else {
                last_line_len += 1;
            }
        }

        if newlines > 0 {
            self.cursor.row += newlines;
            self.cursor.col = last_line_len;
        } else {
            self.cursor.col = insert_col + last_line_len;
        }
    }

    pub fn delete_char_before_cursor(&mut self) {
        let row = self.cursor.row;
        let col = self.cursor.col;

        if row == 0 && col == 0 {
            return;
        }

        if col > 0 {
            let char_idx = self.lines.line_to_char(row) + col - 1;
            self.remove(char_idx..char_idx + 1);
            self.cursor.col -= 1;
        } else {
            let prev_line_len = self.line_len(row - 1);
            let char_idx = self.lines.line_to_char(row);
            if char_idx > 0 {
                self.remove(char_idx - 1..char_idx);
                self.cursor.row -= 1;
                self.cursor.col = prev_line_len;
            }
        }
    }

    pub fn update_scroll(
        &mut self,
        screen_height: usize,
        vertical_scrolloff: usize,
        screen_width: usize,
        horizontal_scrolloff: usize,
    ) {
        let row = self.cursor.row;
        let col = self.cursor.col;

        if row < self.scroll_offset + vertical_scrolloff {
            self.scroll_offset = row.saturating_sub(vertical_scrolloff);
        }

        if row >= self.scroll_offset + screen_height - vertical_scrolloff {
            self.scroll_offset = row + vertical_scrolloff + 1 - screen_height;
        }

        if col < self.scroll_left + horizontal_scrolloff {
            self.scroll_left = col.saturating_sub(horizontal_scrolloff);
        } else if col >= self.scroll_left + screen_width - horizontal_scrolloff {
            self.scroll_left = col.saturating_sub(screen_width - horizontal_scrolloff);
        }
    }

    pub fn delete_selection(&mut self) -> Option<Cursor> {
        if let Some(selection) = self.selection.take() {
            let (start, end) = selection.normalized(&self.cursor);

            let start_char = self.lines.line_to_char(start.row) + start.col;
            let end_char = self.lines.line_to_char(end.row) + end.col;

            if start_char <= end_char {
                self.remove(start_char..end_char);
            }

            self.cursor = start;
            self.selection = None;
            Some(start)
        } else {
            None
        }
    }

    pub fn center_cursor(&mut self, screen_height: usize) {
        let cursor_row = self
            .cursor
            .row
            .min(self.lines.len_lines().saturating_sub(1));

        let half_screen = screen_height / 2;
        if cursor_row >= half_screen {
            self.scroll_offset = cursor_row - half_screen;
        } else {
            self.scroll_offset = 0;
        }

        if self.scroll_offset + screen_height > self.lines.len_lines() {
            self.scroll_offset = self.lines.len_lines().saturating_sub(screen_height);
        }
    }

    fn insert(&mut self, at: usize, text: &str) {
        if self.undo_recording {
            self.undo_tree.record(Edit::Insert {
                at,
                text: text.to_string(),
            });
        }
        self.lines.insert(at, text);
    }

    fn remove(&mut self, range: std::ops::Range<usize>) {
        let deleted = self.lines.slice(range.clone()).to_string();

        if self.undo_recording {
            self.undo_tree.record(Edit::Delete {
                at: range.start,
                text: deleted,
            });
        }

        self.lines.remove(range);
    }

    pub fn undo(&mut self) {
        if let Some(edit) = self.undo_tree.undo() {
            self.undo_recording = false;
            self.apply_inverse_edit(&edit);
            self.undo_recording = true;
        }
    }

    pub fn redo(&mut self) {
        if let Some(edit) = self.undo_tree.redo() {
            self.undo_recording = false;
            self.apply_edit(&edit);
            self.undo_recording = true;
        }
    }

    fn apply_edit(&mut self, edit: &Edit) {
        match edit {
            Edit::Insert { at, text } => {
                self.lines.insert(*at, text);
            }
            Edit::Delete { at, text } => {
                self.lines.remove(*at..(*at + text.len()));
            }
        }
    }

    fn apply_inverse_edit(&mut self, edit: &Edit) {
        match edit {
            Edit::Insert { at, text } => {
                self.lines.remove(*at..(*at + text.len()));
            }
            Edit::Delete { at, text } => {
                self.lines.insert(*at, text);
            }
        }
    }
}

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
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub id: i32,
    pub lines: Vec<String>,
    pub cursor: Cursor,
    pub mode: Mode,
    pub top: usize,
    pub left: usize,
}

impl Buffer {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            lines: vec![String::new()],
            cursor: Cursor::new(),
            mode: Mode::Normal,
            top: 0,
            left: 0,
        }
    }
    pub fn from(id: i32, text: &str) -> Self {
        Self {
            id,
            lines: text.split('\n').map(|s| s.to_string()).collect(),
            cursor: Cursor::new(),
            mode: Mode::Normal,
            top: 0,
            left: 0,
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
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
        }
    }
    pub fn from(text: &str) -> Self {
        Self {
            lines: text.split('\n').map(|s| s.to_string()).collect(),
        }
    }
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
    pub fn line_len(&self, row: usize) -> usize {
        self.lines.get(row).map(|l| l.len()).unwrap_or(0)
    }
}

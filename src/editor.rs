use crate::{
    buffer::{Buffer, Mode},
    keymap::Keymap,
};

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
}

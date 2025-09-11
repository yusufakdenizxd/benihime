use crate::{
    buffer::{Buffer, Mode},
    buffer_manager::BufferManager,
};

pub struct Editor {
    pub focused_buf_id: i32,
    pub buffer_manager: BufferManager,
}

impl Editor {
    pub fn new() -> Self {
        let mut buffer_manager = BufferManager::new();

        let first_id = buffer_manager.create_empty_buffer("untitled".into());

        Self {
            focused_buf_id: first_id,
            buffer_manager,
        }
    }
    pub fn focused_buf(&self) -> &Buffer {
        self.buffer_manager
            .get_buffer(self.focused_buf_id)
            .expect("Focused buffer not found")
    }

    pub fn focused_buf_mut(&mut self) -> &mut Buffer {
        self.buffer_manager
            .get_buffer_mut(self.focused_buf_id)
            .expect("Focused buffer not found")
    }

    pub fn status_line(&self) -> String {
        let mode = match self.focused_buf().mode {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
        };
        format!("{} {}", mode, self.focused_buf().id)
    }
}

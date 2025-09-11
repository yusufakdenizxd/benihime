use std::collections::HashMap;

use crate::buffer::Buffer;

pub struct BufferManager {
    next_id: i32,
    buffers: HashMap<i32, Buffer>,
}

impl BufferManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            buffers: HashMap::new(),
        }
    }

    pub fn get_buffer(&self, id: i32) -> Option<&Buffer> {
        self.buffers.get(&id)
    }

    pub fn get_buffer_mut(&mut self, id: i32) -> Option<&mut Buffer> {
        self.buffers.get_mut(&id)
    }

    pub fn get_buffers(&self) -> Vec<&Buffer> {
        self.buffers.values().collect()
    }

    pub fn get_buffers_mut(&mut self) -> Vec<&mut Buffer> {
        self.buffers.values_mut().collect()
    }

    pub fn get_first_buffer(&self) -> Option<&Buffer> {
        self.buffers
            .keys()
            .min()
            .and_then(|id| self.buffers.get(id))
    }

    pub fn create_empty_buffer(&mut self, name: &str) -> i32 {
        let id = self.next_id;
        self.next_id += 1;

        let buf = Buffer::new(id, name);
        self.buffers.insert(id, buf);

        id
    }

    pub fn create_buffer_from(&mut self, name: &str, text: &str) -> i32 {
        let id = self.next_id;
        self.next_id += 1;

        let buf = Buffer::from(id, name, text);
        self.buffers.insert(id, buf);

        id
    }
}

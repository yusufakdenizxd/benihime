use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::PathBuf;

use crate::buffer::Buffer;

pub struct BufferManager {
    next_id: i32,
    buffers: BTreeMap<i32, Buffer>,
}

impl BufferManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            buffers: BTreeMap::new(),
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

    pub fn iter_buffers(&self) -> impl Iterator<Item = (&i32, &Buffer)> {
        self.buffers.iter()
    }

    pub fn get_buffer_ids(&self) -> Vec<&i32> {
        self.buffers.keys().collect()
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

        let buf = Buffer::new(id, name, None);
        self.buffers.insert(id, buf);

        id
    }

    pub fn create_buffer_from(
        &mut self,
        name: &str,
        text: &str,
        file_path: Option<&PathBuf>,
    ) -> i32 {
        let id = self.next_id;
        self.next_id += 1;

        let buf = Buffer::from(id, name, text, file_path.cloned());
        self.buffers.insert(id, buf);

        id
    }

    pub fn open_file(&mut self, path: &PathBuf) -> i32 {
        let contents = fs::read_to_string(&path).unwrap_or_default();

        self.create_buffer_from(
            path.file_name().unwrap().to_str().unwrap(),
            &contents,
            Some(path),
        )
    }
}

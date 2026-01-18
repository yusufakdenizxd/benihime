use anyhow::{Ok, Result};
use std::path::PathBuf;

use crate::{buffer::Buffer, editor_state::EditorState, project::Project};

pub trait MiniBufferTrait {
    fn render_candidates(&self) -> Vec<String>;
    fn move_focus(&mut self, delta: isize);
    fn run_callback(&mut self, editor: &mut EditorState) -> Result<MinibufferCallbackResult>;
    fn prompt(&self) -> &str;
    fn input_mut(&mut self) -> &mut String;
    fn input(&self) -> &String;
    fn index(&self) -> usize;
    fn offset(&self) -> usize;
    fn len(&self) -> usize;
    fn filter_items(&mut self);
}

pub struct MiniBuffer<T> {
    input: String,
    prompt: String,
    items: Vec<T>,
    base_items: Vec<T>,
    index: usize,
    offset: usize,
    callback: Box<dyn Fn(&mut EditorState, &T) -> Result<Option<Vec<T>>> + Send>,
}

impl<T: Clone> MiniBuffer<T> {
    pub fn new(
        prompt: &str,
        items: Vec<T>,
        callback: impl Fn(&mut EditorState, &T) -> Result<Option<Vec<T>>> + Send + 'static,
    ) -> Self {
        Self {
            input: String::new(),
            prompt: prompt.to_string(),
            items: items.clone(),
            base_items: items,
            index: 0,
            offset: 0,
            callback: Box::new(callback),
        }
    }
}

pub trait MiniBufferDisplay {
    fn as_display_string(&self) -> String;
}

impl MiniBufferDisplay for PathBuf {
    fn as_display_string(&self) -> String {
        self.display().to_string()
    }
}

impl MiniBufferDisplay for String {
    fn as_display_string(&self) -> String {
        self.clone()
    }
}

impl MiniBufferDisplay for Buffer {
    fn as_display_string(&self) -> String {
        self.name.to_string()
    }
}

impl MiniBufferDisplay for Project {
    fn as_display_string(&self) -> String {
        self.name.to_string()
    }
}

impl MiniBufferDisplay for &str {
    fn as_display_string(&self) -> String {
        self.to_string()
    }
}

pub enum MinibufferCallbackResult {
    NewItems,
    Executed,
}

impl<T> MiniBufferTrait for MiniBuffer<T>
where
    T: Clone + MiniBufferDisplay + 'static,
{
    fn render_candidates(&self) -> Vec<String> {
        self.items
            .iter()
            .map(|item| item.as_display_string())
            .collect()
    }

    fn move_focus(&mut self, delta: isize) {
        let scrolloff = 1;
        let max_count = 10;
        let len = self.items.len();

        assert!(len > 0);

        let new_index = ((self.index as isize + delta).rem_euclid(len as isize)) as usize;
        self.index = new_index;

        if len <= max_count {
            return;
        }

        let top = self.offset;
        let bottom = self.offset + max_count;

        //When Goes Up
        if new_index < top + scrolloff {
            self.offset = new_index.saturating_sub(scrolloff);
        }
        //When Goes Down
        if new_index + scrolloff >= bottom && bottom < len {
            self.offset = (new_index + scrolloff + 1).saturating_sub(max_count);
        }

        //Clamp
        if self.offset + max_count > len {
            self.offset = len.saturating_sub(max_count);
        }
    }

    fn run_callback(&mut self, editor: &mut EditorState) -> Result<MinibufferCallbackResult> {
        if let Some(item) = self.items.get(self.index).cloned() {
            if let Some(new_items) = (self.callback)(editor, &item)? {
                self.items = new_items;
                self.index = 0;
                return Ok(MinibufferCallbackResult::NewItems);
            }
        }
        Ok(MinibufferCallbackResult::Executed)
    }

    fn prompt(&self) -> &str {
        &self.prompt
    }

    fn input(&self) -> &String {
        &self.input
    }

    fn input_mut(&mut self) -> &mut String {
        &mut self.input
    }

    fn index(&self) -> usize {
        self.index
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn filter_items(&mut self) {
        if self.input().is_empty() {
            self.items = self.base_items.clone();
        } else {
            let query = self.input.to_lowercase();
            self.items = self
                .base_items
                .iter()
                .filter(|item| item.as_display_string().to_lowercase().contains(&query))
                .cloned()
                .collect();
        }

        self.index = 0;
        self.offset = 0;
    }
}

pub struct MiniBufferManager {
    pub current: Option<Box<dyn MiniBufferTrait>>,
}

impl MiniBufferManager {
    pub fn new() -> Self {
        Self { current: None }
    }

    pub fn activate(&mut self, minibuffer: Box<dyn MiniBufferTrait>) {
        self.current = Some(minibuffer);
    }
}

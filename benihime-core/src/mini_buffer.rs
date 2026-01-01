use anyhow::{Ok, Result};
use std::path::PathBuf;

use crate::{buffer::Buffer, editor_state::EditorState};

pub struct MiniBufferState<T> {
    pub input: String,
    pub prompt: String,
    pub items: Vec<T>,
    pub base_items: Vec<T>,
    pub index: usize,
    pub offset: usize,
    pub callback: Box<dyn Fn(&mut EditorState, &T) -> Result<Option<Vec<T>>> + Send>,
}

impl<T: Clone> MiniBufferState<T> {
    pub fn new(
        prompt: String,
        items: Vec<T>,
        callback: impl Fn(&mut EditorState, &T) -> Result<Option<Vec<T>>> + Send + 'static,
    ) -> Self {
        Self {
            input: String::new(),
            prompt,
            items: items.clone(),
            base_items: items,
            index: 0,
            callback: Box::new(callback),
            offset: 0,
        }
    }
}

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
    pub state: MiniBufferState<T>,
}

impl<T: Clone> MiniBuffer<T> {
    pub fn new(
        prompt: &str,
        items: Vec<T>,
        callback: impl Fn(&mut EditorState, &T) -> Result<Option<Vec<T>>> + Send + 'static,
    ) -> Self {
        Self {
            state: MiniBufferState::new(prompt.to_string(), items, callback),
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
    T: Clone + Send + MiniBufferDisplay + 'static,
{
    fn render_candidates(&self) -> Vec<String> {
        self.state
            .items
            .iter()
            .map(|item| item.as_display_string())
            .collect()
    }

    fn move_focus(&mut self, delta: isize) {
        let scrolloff = 1;
        let max_count = 10;
        let len = self.state.items.len();

        assert!(len > 0);

        let new_index = ((self.state.index as isize + delta).rem_euclid(len as isize)) as usize;
        self.state.index = new_index;

        if len <= max_count {
            return;
        }

        let top = self.state.offset;
        let bottom = self.state.offset + max_count;

        //When Goes Up
        if new_index < top + scrolloff {
            self.state.offset = new_index.saturating_sub(scrolloff);
        }
        //When Goes Down
        if new_index + scrolloff >= bottom && bottom < len {
            self.state.offset = (new_index + scrolloff + 1).saturating_sub(max_count);
        }

        //Clamp
        if self.state.offset + max_count > len {
            self.state.offset = len.saturating_sub(max_count);
        }
    }

    fn run_callback(&mut self, editor: &mut EditorState) -> Result<MinibufferCallbackResult> {
        if let Some(item) = self.state.items.get(self.state.index).cloned() {
            if let Some(new_items) = (self.state.callback)(editor, &item)? {
                self.state.items = new_items;
                self.state.index = 0;
                return Ok(MinibufferCallbackResult::NewItems);
            }
        }
        Ok(MinibufferCallbackResult::Executed)
    }

    fn prompt(&self) -> &str {
        &self.state.prompt
    }

    fn input(&self) -> &String {
        &self.state.input
    }

    fn input_mut(&mut self) -> &mut String {
        &mut self.state.input
    }

    fn index(&self) -> usize {
        self.state.index
    }

    fn offset(&self) -> usize {
        self.state.offset
    }

    fn len(&self) -> usize {
        self.state.items.len()
    }

    fn filter_items(&mut self) {
        if self.input().is_empty() {
            self.state.items = self.state.base_items.clone();
        } else {
            let query = self.state.input.to_lowercase();
            self.state.items = self
                .state
                .base_items
                .iter()
                .filter(|item| item.as_display_string().to_lowercase().contains(&query))
                .cloned()
                .collect();
        }

        self.state.index = 0;
        self.state.offset = 0;
    }
}

pub struct MiniBufferManager {
    pub current: Option<Box<dyn MiniBufferTrait + Send>>,
}

impl MiniBufferManager {
    pub fn new() -> Self {
        Self { current: None }
    }

    pub fn activate(&mut self, minibuffer: Box<dyn MiniBufferTrait + Send>) {
        self.current = Some(minibuffer);
    }
}

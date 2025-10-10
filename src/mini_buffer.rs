use anyhow::{Ok, Result};
use std::path::PathBuf;

use crate::editor::EditorState;

pub struct MiniBufferState<T> {
    pub input: String,
    pub prompt: String,
    pub items: Vec<T>,
    pub index: usize,
    pub offset: usize,
    pub callback: Box<dyn Fn(&mut EditorState, &T) -> Result<Option<Vec<T>>> + Send>,
}

impl<T> MiniBufferState<T> {
    pub fn new(
        prompt: String,
        items: Vec<T>,
        callback: impl Fn(&mut EditorState, &T) -> Result<Option<Vec<T>>> + Send + 'static,
    ) -> Self {
        Self {
            input: String::new(),
            prompt,
            items,
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
}

pub struct PathMiniBuffer {
    pub state: MiniBufferState<PathBuf>,
}

impl PathMiniBuffer {
    pub fn new(
        prompt: &str,
        items: Vec<PathBuf>,
        callback: impl Fn(&mut EditorState, &PathBuf) -> Result<Option<Vec<PathBuf>>> + Send + 'static,
    ) -> Self {
        Self {
            state: MiniBufferState::new(prompt.to_string(), items, callback),
        }
    }
}

pub enum MinibufferCallbackResult {
    NewItems,
    Executed,
}

impl MiniBufferTrait for PathMiniBuffer {
    fn render_candidates(&self) -> Vec<String> {
        self.state
            .items
            .iter()
            .map(|p| p.display().to_string())
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

    pub fn deactivate(&mut self) {
        self.current = None;
    }

    pub fn active(&self) -> bool {
        self.current.is_some()
    }
}

use anyhow::{Ok, Result};
use crossterm::event::KeyEvent;

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use thiserror::Error;

use crate::{
    buffer::{Buffer, Mode},
    buffer_manager::BufferManager,
    command::{self, command::CommandContext, command_registry::CommandRegistry},
    keymap::{self, keymap::Keymap},
};

#[derive(Debug, Error)]
pub enum HandleKeyError {
    #[error("Key not found in keymap")]
    KeyNotFound,
    #[error("Command not found: {0}")]
    CommandNotFound(String),
    #[error("Command execution failed: {0}")]
    ExecutionFailed(#[from] anyhow::Error),
}
pub struct EditorState {
    pub focused_buf_id: i32,
    pub buffer_manager: BufferManager,
    pub command_buffer: String,
}

impl EditorState {
    pub fn focused_buf_mut(&mut self) -> &mut Buffer {
        self.buffer_manager
            .get_buffer_mut(self.focused_buf_id)
            .expect("Focused buffer must exist")
    }

    pub fn focused_buf(&self) -> &Buffer {
        self.buffer_manager
            .get_buffer(self.focused_buf_id)
            .expect("Focused buffer must exist")
    }

    pub fn status_line(&self) -> String {
        let buf = self.buffer_manager.get_buffer(self.focused_buf_id).unwrap();
        let mode = match buf.mode {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
            Mode::Command => "COMMAND",
        };
        format!("{} {}", mode, buf.id)
    }
}

pub struct Editor {
    pub state: Arc<Mutex<EditorState>>,
    pub command_registry: CommandRegistry,
    pub keymap: Keymap,
}

impl Editor {
    pub fn new() -> Self {
        let mut buffer_manager = BufferManager::new();
        let first_id = buffer_manager.create_empty_buffer("untitled".into());

        let mut command_registry = CommandRegistry::new();
        command::default_commands::register_default_commands(&mut command_registry);

        let mut keymap = Keymap::new();
        keymap::default_keymap::register_default_keymap(&mut keymap);

        let state = EditorState {
            focused_buf_id: first_id,
            buffer_manager,
            command_buffer: String::new(),
        };

        Self {
            state: Arc::new(Mutex::new(state)),
            keymap,
            command_registry,
        }
    }

    pub fn handle_key(&self, state: &mut EditorState, key: KeyEvent) -> Result<(), HandleKeyError> {
        let (_modes, command_name, args) = self
            .keymap
            .lookup(state.focused_buf().mode, key)
            .ok_or(HandleKeyError::KeyNotFound)?;

        let mut ctx = CommandContext {
            state,
            args: &args,
            registry: &self.command_registry,
        };

        self.command_registry.execute(&command_name, &mut ctx)
    }

    pub fn open_file(&self, path: &str) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        let id = state.buffer_manager.open_file(PathBuf::from(path));
        state.focused_buf_id = id;
        Ok(())
    }
}

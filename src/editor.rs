use std::sync::{Arc, Mutex};

use thiserror::Error;

use crate::{
    buffer::{Buffer, Mode},
    buffer_manager::BufferManager,
    command::{
        self,
        command::{CommandArg, CommandContext},
        command_registry::CommandRegistry,
    },
    keymap::{
        self,
        key_chord::{KeyChord, KeyCode, KeyModifiers},
        keymap::Keymap,
    },
    mini_buffer::MiniBufferManager,
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

impl PartialEq for HandleKeyError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::KeyNotFound, Self::KeyNotFound) => true,
            _ => false,
        }
    }
}
pub struct EditorState {
    pub focused_buf_id: i32,
    pub buffer_manager: BufferManager,
    pub minibuffer_manager: MiniBufferManager,
    pub screen_height: usize,
    pub command_buffer: String,
    pub message: Option<String>,
    pub error_message: Option<String>,
    pub registry: Arc<CommandRegistry>,
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
            Mode::Minibuffer => "MINIBUFFER",
        };
        format!("{} {}", mode, buf.id)
    }

    pub fn exec(
        &mut self,
        name: &str,
        args: Option<Vec<CommandArg>>,
    ) -> Result<(), HandleKeyError> {
        let registry = Arc::clone(&self.registry);
        registry.execute(
            name,
            &mut CommandContext {
                state: self,
                args: &args,
            },
        )
    }
}

pub struct Editor {
    pub state: Arc<Mutex<EditorState>>,
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
            message: None,
            error_message: None,
            screen_height: 0,
            minibuffer_manager: MiniBufferManager::new(),
            registry: Arc::new(command_registry),
        };

        Self {
            state: Arc::new(Mutex::new(state)),
            keymap,
        }
    }

    pub fn handle_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        let mut state = self.state.lock().unwrap();
        let buf = state.focused_buf_mut();

        let chord = KeyChord {
            code: key,
            modifiers,
        };
        match self.keymap.push_key(buf.mode, &chord) {
            Some((command_name, args)) => {
                state.exec(&command_name, args);
            }
            None => match buf.mode {
                Mode::Insert => {
                    if chord.code == KeyCode::Backspace {
                        buf.delete_char_before_cursor();
                    } else if let Some(c) = chord.as_char() {
                        buf.insert_char(c);
                    }
                }
                Mode::Command => {
                    if chord.code == KeyCode::Backspace {
                        state.command_buffer.pop();
                    } else if let Some(c) = chord.as_char() {
                        state.command_buffer.push(c);
                    }
                }

                Mode::Minibuffer => {
                    if let Some(mini) = state.minibuffer_manager.current.as_mut() {
                        if chord.code == KeyCode::Backspace {
                            mini.input_mut().pop();
                        } else if let Some(c) = chord.as_char() {
                            mini.input_mut().push(c);
                        }
                        mini.filter_items();
                    }
                }
                _ => {}
            },
        }
    }
}

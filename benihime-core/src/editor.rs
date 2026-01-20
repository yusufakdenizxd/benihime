use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use benihime_loader::paths;
use egui::ViewportBuilder;
use thiserror::Error;

use crate::{
    buffer::{BufferId, Mode},
    buffer_manager::BufferManager,
    command::{self, command_registry::CommandRegistry},
    editor_state::EditorState,
    keymap::{
        self, Keymap,
        key_chord::{KeyChord, KeyCode, KeyModifiers},
    },
    mini_buffer::MiniBufferManager,
    project::project_manager::ProjectManager,
    theme::theme_loader::ThemeLoader,
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
            (Self::CommandNotFound(s1), Self::CommandNotFound(s2)) => s1 == s2,
            (Self::ExecutionFailed(e1), Self::ExecutionFailed(e2)) => {
                e1.to_string() == e2.to_string()
            }
            _ => false,
        }
    }
}

pub struct Editor {
    pub state: Arc<Mutex<EditorState>>,
    pub keymap: Keymap,
}

impl Editor {
    pub fn new() -> Self {
        let loader = benihime_loader::Loader::new().unwrap();

        let buffer_manager = BufferManager::new();

        let mut command_registry = CommandRegistry::new();
        command::default_commands::register_default_commands(&mut command_registry);

        let theme_loader = ThemeLoader::new(loader.paths.themes_dir());

        let mut keymap = Keymap::new();
        keymap::default_keymap::register_default_keymap(&mut keymap);

        let mut project_manager = ProjectManager::new();

        //TODO: fetch from config file
        if let Ok(dir) = paths::home_dir() {
            let projects_dir = dir.join("dev");
            project_manager.discover_in_path(&projects_dir);
        }

        let mut state = EditorState {
            focused_buf_id: BufferId(0),
            project_manager,
            buffer_manager,
            command_buffer: String::new(),
            message: None,
            error_message: None,
            screen_height: 0,
            screen_width: 0,
            minibuffer_manager: MiniBufferManager::new(),
            registry: Arc::new(command_registry),
            theme: theme_loader.default(),
            theme_loader: Arc::new(theme_loader),
            cwd: None,
        };

        let first_id = state.create_empty_buffer("[No Name]");
        state.focused_buf_id = first_id;

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
                let _ = state.exec(&command_name, args);
            }
            None => match buf.mode {
                Mode::Insert => {
                    if chord.code == KeyCode::Backspace {
                        buf.delete_char_before_cursor();
                    } else if chord.code == KeyCode::Enter {
                        buf.insert_char('\n');
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

    pub fn run(&mut self) -> eframe::Result<()> {
        let options = eframe::NativeOptions {
            viewport: ViewportBuilder::with_decorations(ViewportBuilder::default(), false),
            ..Default::default()
        };
        eframe::run_native(
            "Benihime Editor",
            options,
            Box::new(|_cc| Ok(Box::new(self))),
        )
    }
}

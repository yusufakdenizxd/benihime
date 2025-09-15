use std::collections::HashMap;

use crossterm::event::KeyEvent;

use crate::{buffer::Mode, command::command::CommandArg};

#[derive(Debug, Clone)]
pub struct Keymap {
    pub bindings: HashMap<KeyEvent, (Vec<Mode>, String, Option<Vec<CommandArg>>)>,
}

impl Keymap {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn bind(
        &mut self,
        modes: &[Mode],
        key: KeyEvent,
        command: &str,
        args: Option<Vec<CommandArg>>,
    ) {
        self.bindings
            .insert(key, (modes.to_vec(), command.to_string(), args));
    }

    pub fn lookup(
        &self,
        mode: Mode,
        key: KeyEvent,
    ) -> Option<(Vec<Mode>, String, Option<Vec<CommandArg>>)> {
        self.bindings
            .get(&key)
            .filter(|(modes, _, _)| modes.contains(&mode))
            .map(|(modes, cmd_name, args)| (modes.clone(), cmd_name.clone(), args.clone()))
    }
}

use anyhow::Result;
use std::collections::HashMap;

use crate::editor::HandleKeyError;

use super::command::{CommandContext, CommandFn};

pub struct CommandRegistry {
    commands: HashMap<String, CommandFn>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, cmd: CommandFn) {
        self.commands.insert(name.to_string(), cmd);
    }

    pub fn execute(&self, name: &str, ctx: &mut CommandContext) -> Result<(), HandleKeyError> {
        if let Some(cmd) = self.commands.get(name) {
            cmd(ctx).map_err(HandleKeyError::ExecutionFailed)
        } else {
            Err(HandleKeyError::CommandNotFound(name.to_string()))
        }
    }
}

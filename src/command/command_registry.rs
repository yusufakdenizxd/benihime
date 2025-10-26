use anyhow::Result;
use std::collections::HashMap;

use crate::editor::HandleKeyError;

use super::command::{CommandArg, CommandContext, CommandFn};

pub struct CommandRegistry {
    pub commands: HashMap<String, CommandFn>,
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
            match cmd(ctx) {
                Ok(o) => {
                    if name != "clear-error-message" && name != "error-message" {
                        let _ = self.execute(
                            "clear-error-message",
                            &mut CommandContext {
                                state: ctx.state,
                                args: &None,
                            },
                        );
                    }
                    Ok(o)
                }
                Err(e) => {
                    let _ = self.execute(
                        "error-message",
                        &mut CommandContext {
                            state: ctx.state,
                            args: &Some(vec![CommandArg::Str(e.to_string())]),
                        },
                    );
                    return Err(HandleKeyError::ExecutionFailed(e));
                }
            }
        } else {
            Err(HandleKeyError::CommandNotFound(name.to_string()))
        }
    }
}

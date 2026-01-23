use anyhow::Result;
use std::collections::HashMap;

use crate::editor::HandleKeyError;

use crate::command::{Cmd, CommandArg, CommandContext, CommandFn};

pub struct CommandRegistry {
    pub commands: HashMap<String, Cmd>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, cmd: CommandFn) {
        self.commands.insert(name.to_string(), Cmd::Plain(cmd));
    }

    pub fn register_motion(&mut self, name: &str, cmd: CommandFn) {
        self.commands.insert(name.to_string(), Cmd::Motion(cmd));
    }

    pub fn register_operator(&mut self, name: &str, cmd: CommandFn) {
        self.commands.insert(name.to_string(), Cmd::Operator(cmd));
    }

    pub fn register_navigation(&mut self, name: &str, cmd: CommandFn) {
        self.commands.insert(name.to_string(), Cmd::Navigation(cmd));
    }

    pub fn register_system(&mut self, name: &str, cmd: CommandFn) {
        self.commands.insert(name.to_string(), Cmd::System(cmd));
    }

    pub fn execute(&self, name: &str, ctx: &mut CommandContext) -> Result<(), HandleKeyError> {
        if let Some(cmd) = self.commands.get(name) {
            match cmd {
                Cmd::Plain(f) => {
                    ctx.state.focused_buf_mut().range = None;
                    self.call_cmd(f, ctx, name)
                }
                Cmd::Motion(f) => self.call_cmd(f, ctx, name),
                Cmd::Operator(f) => self.call_cmd(f, ctx, name),
                Cmd::System(f) => self.call_cmd(f, ctx, name),
                Cmd::Navigation(f) => {
                    ctx.state.focused_buf_mut().range = None;
                    self.call_cmd(f, ctx, name)
                }
            }
        } else {
            Err(HandleKeyError::CommandNotFound(name.to_string()))
        }
    }

    fn call_cmd(
        &self,
        f: &CommandFn,
        ctx: &mut CommandContext,
        name: &str,
    ) -> Result<(), HandleKeyError> {
        match f(ctx) {
            Ok(o) => {
                if name != "clear-error-message" && name != "error-message" {
                    let _ = self.execute(
                        "clear-error-message",
                        &mut CommandContext {
                            state: ctx.state,
                            args: &None,
                            count: 1,
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
                        count: 1,
                    },
                );
                Err(HandleKeyError::ExecutionFailed(e))
            }
        }
    }
}

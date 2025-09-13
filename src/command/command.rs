use anyhow::{Result, anyhow};
use std::collections::HashMap;

use crate::{buffer::Mode, editor::EditorState};

use super::command_registry::CommandRegistry;

#[derive(Debug, Clone)]
pub enum CommandArg {
    Str(String),
    Int(i64),
    Bool(bool),
    Mode(Mode),
    Position { row: usize, col: usize },
}

pub struct CommandContext<'a> {
    pub state: &'a mut EditorState,
    pub args: &'a Option<HashMap<String, CommandArg>>,
    pub registry: &'a CommandRegistry,
}

impl<'a> CommandContext<'a> {
    pub fn get_arg<T>(&self, name: &str) -> Result<T>
    where
        CommandArg: ArgAsOwned<T>,
    {
        let args = self
            .args
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No arguments provided"))?;

        let arg = args
            .get(name)
            .ok_or_else(|| anyhow!("Missing argument '{}'", name))?;
        arg.as_type_owned()
            .ok_or_else(|| anyhow!("Argument '{}' has wrong type", name))
    }
}

pub trait ArgAsOwned<T> {
    fn as_type_owned(&self) -> Option<T>;
}

impl ArgAsOwned<String> for CommandArg {
    fn as_type_owned(&self) -> Option<String> {
        match self {
            CommandArg::Str(s) => Some(s.clone()),
            _ => None,
        }
    }
}

impl ArgAsOwned<i64> for CommandArg {
    fn as_type_owned(&self) -> Option<i64> {
        match self {
            CommandArg::Int(i) => Some(*i),
            _ => None,
        }
    }
}

impl ArgAsOwned<bool> for CommandArg {
    fn as_type_owned(&self) -> Option<bool> {
        match self {
            CommandArg::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

impl ArgAsOwned<crate::buffer::Mode> for CommandArg {
    fn as_type_owned(&self) -> Option<crate::buffer::Mode> {
        match self {
            CommandArg::Mode(m) => Some(*m),
            _ => None,
        }
    }
}

impl ArgAsOwned<(usize, usize)> for CommandArg {
    fn as_type_owned(&self) -> Option<(usize, usize)> {
        match self {
            CommandArg::Position { row, col } => Some((*row, *col)),
            _ => None,
        }
    }
}

pub type CommandFn = fn(&mut CommandContext) -> anyhow::Result<()>;

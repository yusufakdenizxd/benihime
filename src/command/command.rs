use anyhow::{Result, anyhow};
use std::path::PathBuf;

use crate::{buffer::Mode, editor::EditorState};

#[derive(Debug, Clone)]
pub enum CommandArg {
    Str(String),
    Int(i64),
    Bool(bool),
    Mode(Mode),
    Position { row: usize, col: usize },
    Path(PathBuf),
}

impl CommandArg {
    pub fn parse_arg(token: &str) -> CommandArg {
        if token.contains('/') || token.contains('.') {
            return CommandArg::Path(PathBuf::from(token));
        }

        if let Some((row_str, col_str)) = token.split_once(',') {
            if let (Ok(row), Ok(col)) = (row_str.parse::<usize>(), col_str.parse::<usize>()) {
                return CommandArg::Position { row, col };
            }
        }

        if let Ok(mode) = token.parse::<Mode>() {
            return CommandArg::Mode(mode);
        }

        if let Ok(i) = token.parse::<i64>() {
            return CommandArg::Int(i);
        }

        if token.eq_ignore_ascii_case("true") {
            return CommandArg::Bool(true);
        }

        if token.eq_ignore_ascii_case("false") {
            return CommandArg::Bool(false);
        }

        CommandArg::Str(token.to_string())
    }
}

pub struct CommandContext<'a> {
    pub state: &'a mut EditorState,
    pub args: &'a Option<Vec<CommandArg>>,
}

impl CommandContext<'_> {
    pub fn get_arg<T>(&self, index: usize) -> Result<T>
    where
        CommandArg: ArgAsOwned<T>,
    {
        let args = self
            .args
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No arguments provided"))?;

        let arg = args
            .get(index)
            .ok_or_else(|| anyhow!("Missing argument '{}'", index))?;
        arg.as_type_owned()
            .ok_or_else(|| anyhow!("Argument '{}' has wrong type", index))
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

impl ArgAsOwned<PathBuf> for CommandArg {
    fn as_type_owned(&self) -> Option<PathBuf> {
        match self {
            CommandArg::Path(p) => Some(p.clone()),
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

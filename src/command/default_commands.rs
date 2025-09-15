use std::{cmp::min, path::PathBuf};

use anyhow::Ok;

use crate::{buffer::Mode, editor::HandleKeyError};

use super::{
    command::{CommandArg, CommandContext},
    command_registry::CommandRegistry,
};

pub fn register_default_commands(registry: &mut CommandRegistry) {
    registry.register("move-left", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        buf.cursor.col = buf.cursor.col.saturating_sub(1);
        Ok(())
    });

    registry.register("move-down", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        buf.cursor.row = min(buf.cursor.row + 1, buf.line_count() - 1);
        buf.cursor.col = min(buf.cursor.col, buf.line_len(buf.cursor.row));
        Ok(())
    });

    registry.register("move-up", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        buf.cursor.row = buf.cursor.row.saturating_sub(1);
        buf.cursor.col = min(buf.cursor.col, buf.line_len(buf.cursor.row));
        Ok(())
    });

    registry.register("move-right", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        buf.cursor.col = min(buf.cursor.col + 1, buf.line_len(buf.cursor.row));
        Ok(())
    });

    registry.register("set-mode", |ctx: &mut CommandContext| {
        let mode: Mode = ctx.get_arg(0)?;
        if mode == Mode::Command {
            ctx.state.command_buffer.clear();
        }

        let buf = ctx.state.focused_buf_mut();
        buf.mode = mode;

        Ok(())
    });

    registry.register("beginning-of-line", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        buf.cursor.col = 0;
        Ok(())
    });

    registry.register("end-of-line", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        buf.cursor.col = buf.line_len(buf.cursor.row);
        Ok(())
    });

    registry.register("first-non-blank", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        buf.cursor.col = buf.line_len(buf.cursor.row);

        let line = &buf.lines[buf.cursor.row];
        let mut i = 0;
        while i < line.len() && line.as_bytes()[i].is_ascii_whitespace() {
            i += 1;
        }
        buf.cursor.col = min(i, line.len());
        Ok(())
    });

    registry.register("open-above", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        buf.lines.insert(buf.cursor.row, String::new());
        buf.cursor.col = 0;
        ctx.registry.execute(
            "set-mode",
            &mut CommandContext {
                state: ctx.state,
                args: &Some(vec![CommandArg::Mode(Mode::Insert)]),
                registry: ctx.registry,
            },
        )?;
        Ok(())
    });

    registry.register("open-below", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        let i = buf.lines[buf.cursor.row].len();
        let rest = buf.lines[buf.cursor.row].split_off(i);
        buf.lines.insert(buf.cursor.row + 1, rest);
        buf.cursor.row += 1;
        buf.cursor.col = 0;

        ctx.registry.execute(
            "set-mode",
            &mut CommandContext {
                state: ctx.state,
                args: &Some(vec![CommandArg::Mode(Mode::Insert)]),
                registry: ctx.registry,
            },
        )?;

        Ok(())
    });

    registry.register("word-forward", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        let line = &buf.lines[buf.cursor.row];
        let mut i = buf.cursor.col;
        if i < line.len() {
            i += 1;
        }
        while i < line.len() && line.as_bytes()[i].is_ascii_whitespace() {
            i += 1;
        }
        while i < line.len() && !line.as_bytes()[i].is_ascii_whitespace() {
            i += 1;
        }
        buf.cursor.col = min(i, line.len());
        Ok(())
    });

    registry.register("word-start", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        let line = &buf.lines[buf.cursor.row];
        let mut i = buf.cursor.col;
        if i > 0 {
            i -= 1;
        }
        while i > 0 && line.as_bytes()[i].is_ascii_whitespace() {
            i -= 1;
        }
        while i > 0 && !line.as_bytes()[i - 1].is_ascii_whitespace() {
            i -= 1;
        }
        buf.cursor.col = i;
        Ok(())
    });

    registry.register("word-end", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        let line = &buf.lines[buf.cursor.row];
        let mut i = buf.cursor.col;
        while i < line.len() && line.as_bytes()[i].is_ascii_whitespace() {
            i += 1;
        }
        while i < line.len() {
            if i + 1 >= line.len() || line.as_bytes()[i + 1].is_ascii_whitespace() {
                break;
            }
            i += 1;
        }
        buf.cursor.col = min(i, line.len());
        Ok(())
    });

    registry.register("execute-command-buffer", |ctx: &mut CommandContext| {
        ctx.registry.execute(
            "set-mode",
            &mut CommandContext {
                state: ctx.state,
                args: &Some(vec![CommandArg::Mode(Mode::Normal)]),
                registry: ctx.registry,
            },
        )?;

        let line = ctx.state.command_buffer.clone();
        ctx.state.command_buffer.clear();

        if line.is_empty() {
            return Ok(());
        }

        let mut parts = line.split_whitespace();
        let command_name = parts
            .next()
            .ok_or_else(|| HandleKeyError::ExecutionFailed(anyhow::anyhow!("Empty command")))?;

        let args: Vec<CommandArg> = parts.map(|tok| CommandArg::parse_arg(tok)).collect();

        let mut command_ctx = CommandContext {
            state: ctx.state,
            args: &Some(args),
            registry: ctx.registry,
        };

        ctx.registry.execute(command_name, &mut command_ctx)?;
        Ok(())
    });

    registry.register("open-file", |ctx: &mut CommandContext| {
        let path: PathBuf = ctx.get_arg(0)?;
        let state = &mut ctx.state;
        let id = state.buffer_manager.open_file(PathBuf::from(path));
        state.focused_buf_id = id;
        Ok(())
    });

    registry.register("echo", |ctx: &mut CommandContext| {
        let text: String = ctx.get_arg(0)?;
        let state = &mut ctx.state;
        state.set_message(text);

        Ok(())
    });
}

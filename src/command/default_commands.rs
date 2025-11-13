use std::{
    cmp::{Ordering, min},
    fs,
    path::PathBuf,
};

use anyhow::Ok;
use ignore::Walk;

use crate::{
    buffer::{Buffer, Mode, Selection},
    editor::{EditorState, HandleKeyError},
    mini_buffer::{MiniBuffer, MinibufferCallbackResult},
};

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
        ctx.state
            .exec("set-mode", Some(vec![CommandArg::Mode(Mode::Insert)]))?;
        Ok(())
    });

    registry.register("open-below", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        let i = buf.lines[buf.cursor.row].len();
        let rest = buf.lines[buf.cursor.row].split_off(i);
        buf.lines.insert(buf.cursor.row + 1, rest);
        buf.cursor.row += 1;
        buf.cursor.col = 0;

        ctx.state
            .exec("set-mode", Some(vec![CommandArg::Mode(Mode::Insert)]))?;

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
        ctx.state
            .exec("set-mode", Some(vec![CommandArg::Mode(Mode::Normal)]))?;

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

        ctx.state.exec(command_name, Some(args))?;
        Ok(())
    });

    registry.register("open-file", |ctx: &mut CommandContext| {
        let cwd = std::env::current_dir().unwrap();
        let mut files: Vec<PathBuf> = fs::read_dir(&cwd)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();

        files.sort_by(|a, b| {
            if a.is_dir() && !b.is_dir() {
                return Ordering::Less;
            }
            if !a.is_dir() && b.is_dir() {
                return Ordering::Greater;
            }
            Ordering::Equal
        });

        let minibuffer: MiniBuffer<PathBuf> = MiniBuffer::new(
            "Open File: ",
            files,
            |state: &mut EditorState, path: &PathBuf| {
                if path.is_dir() {
                    let mut new_items: Vec<PathBuf> = fs::read_dir(path)?
                        .filter_map(|e| e.ok().map(|e| e.path()))
                        .collect();

                    new_items.sort_by(|a, b| {
                        if a.is_dir() && !b.is_dir() {
                            return Ordering::Less;
                        }
                        if !a.is_dir() && b.is_dir() {
                            return Ordering::Greater;
                        }
                        Ordering::Equal
                    });

                    return Ok(Some(new_items));
                } else {
                    let id = state.buffer_manager.open_file(&path.clone());
                    state.focused_buf_id = id;
                    println!("Opened file: {}", path.display());
                }
                Ok(None)
            },
        );

        ctx.state.minibuffer_manager.activate(Box::new(minibuffer));

        ctx.state
            .exec("set-mode", Some(vec![CommandArg::Mode(Mode::Minibuffer)]))?;

        Ok(())
    });

    registry.register("find-file", |ctx: &mut CommandContext| {
        let cwd = std::env::current_dir().unwrap();

        let files: Vec<PathBuf> = Walk::new(cwd)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().unwrap().is_file())
            .map(|x| x.path().to_owned())
            .collect();

        let minibuffer: MiniBuffer<PathBuf> = MiniBuffer::new(
            "Find File: ",
            files,
            |state: &mut EditorState, path: &PathBuf| {
                let id = state.buffer_manager.open_file(&path.clone());
                state.focused_buf_id = id;
                Ok(None)
            },
        );

        ctx.state.minibuffer_manager.activate(Box::new(minibuffer));

        ctx.state
            .exec("set-mode", Some(vec![CommandArg::Mode(Mode::Minibuffer)]))?;

        Ok(())
    });

    registry.register("echo", |ctx: &mut CommandContext| {
        let text: String = ctx.get_arg(0)?;
        let state = &mut ctx.state;
        state.message = Some(text);
        state.error_message = None;

        Ok(())
    });

    registry.register("error-message", |ctx: &mut CommandContext| {
        let text: String = ctx.get_arg(0)?;
        let state = &mut ctx.state;
        state.message = None;
        state.error_message = Some(text);

        Ok(())
    });

    registry.register("clear-error-message", |ctx: &mut CommandContext| {
        let state = &mut ctx.state;
        state.error_message = None;

        Ok(())
    });

    registry.register("next-buffer", |ctx: &mut CommandContext| {
        let state = &mut ctx.state;
        let focused_id = state.focused_buf_id;
        let mut ids = state.buffer_manager.get_buffer_ids();

        ids.sort();
        if ids.is_empty() {
            return Ok(());
        }

        if let Some(id) = ids.iter().position(|&id| *id == focused_id) {
            let next_index = (id + 1) % ids.len();
            state.focused_buf_id = *ids[next_index];
        }

        Ok(())
    });
    registry.register("previous-buffer", |ctx: &mut CommandContext| {
        let state = &mut ctx.state;
        let focused_id = state.focused_buf_id;
        let mut ids = state.buffer_manager.get_buffer_ids();

        ids.sort();
        if ids.is_empty() {
            return Ok(());
        }

        if let Some(pos) = ids.iter().position(|&id| *id == focused_id) {
            let prev_index = (pos + ids.len() - 1) % ids.len();
            state.focused_buf_id = *ids[prev_index];
        }

        Ok(())
    });
    registry.register("center-cursor", |ctx: &mut CommandContext| {
        let screen_height = ctx.state.screen_height;
        let buf = ctx.state.focused_buf_mut();
        buf.center_cursor(screen_height);
        Ok(())
    });

    registry.register("minibuffer-next-completion", |ctx: &mut CommandContext| {
        if let Some(mini) = ctx.state.minibuffer_manager.current.as_mut() {
            mini.move_focus(1);
        }
        Ok(())
    });

    registry.register(
        "minibuffer-previous-completion",
        |ctx: &mut CommandContext| {
            if let Some(mini) = ctx.state.minibuffer_manager.current.as_mut() {
                mini.move_focus(-1);
            }
            Ok(())
        },
    );

    registry.register("minibuffer-accept", |ctx: &mut CommandContext| {
        ctx.state
            .exec("set-mode", Some(vec![CommandArg::Mode(Mode::Normal)]))?;

        if let Some(mut mini) = ctx.state.minibuffer_manager.current.take() {
            let result = mini.run_callback(ctx.state)?;
            ctx.state.minibuffer_manager.current = Some(mini);
            match result {
                MinibufferCallbackResult::NewItems => {
                    //Early return to be in minibuffer mode
                    return Ok(());
                }
                MinibufferCallbackResult::Executed => {}
            }
        }

        Ok(())
    });

    registry.register("find-command", |ctx: &mut CommandContext| {
        let commands: Vec<String> = ctx.state.registry.commands.keys().cloned().collect();

        let minibuffer: MiniBuffer<String> = MiniBuffer::new(
            "Find Command: ",
            commands,
            |state: &mut EditorState, command_name: &String| {
                let _ = state.exec(&command_name, None);
                Ok(None)
            },
        );

        ctx.state.minibuffer_manager.activate(Box::new(minibuffer));

        ctx.state
            .exec("set-mode", Some(vec![CommandArg::Mode(Mode::Minibuffer)]))?;

        Ok(())
    });

    registry.register("kill-this-buffer", |ctx: &mut CommandContext| {
        let state = &mut ctx.state;
        state.kill_active_buffer();
        Ok(())
    });

    registry.register("enter-visual-mode", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        if buf.mode != Mode::Visual {
            buf.selection = Some(Selection {
                start: buf.cursor.clone(),
            });
            buf.mode = Mode::Visual;
        }
        Ok(())
    });

    registry.register("exit-visual-mode", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        buf.selection = None;
        buf.mode = Mode::Normal;
        Ok(())
    });

    registry.register("visual_select_other_end", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        if buf.mode == Mode::Visual {
            if let Some(selection) = &mut buf.selection {
                std::mem::swap(&mut selection.start, &mut buf.cursor);
            }
        }
        Ok(())
    });

    registry.register("delete-selection", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        if buf.mode == Mode::Visual {
            buf.delete_selection();
            buf.mode = Mode::Normal;
        }
        Ok(())
    });

    registry.register("change-selection", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        if buf.mode == Mode::Visual {
            buf.delete_selection();
            buf.mode = Mode::Insert;
        }
        Ok(())
    });

    registry.register("delete-char-under-cursor", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        if buf.cursor.row < buf.line_count() {
            let line = &mut buf.lines[buf.cursor.row];
            if buf.cursor.col < line.len() {
                line.remove(buf.cursor.col);
            }
        }

        Ok(())
    });

    registry.register("delete-line", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        if buf.cursor.row < buf.line_count() {
            buf.lines.remove(buf.cursor.row);
        }
        Ok(())
    });

    registry.register("find-buffer", |ctx: &mut CommandContext| {
        let buffers = ctx.state.buffer_manager.get_buffers_cloned();

        let minibuffer: MiniBuffer<Buffer> = MiniBuffer::new(
            "Find Buffer: ",
            buffers,
            |state: &mut EditorState, command_name: &Buffer| {
                state.focused_buf_id = command_name.id;
                Ok(None)
            },
        );

        ctx.state.minibuffer_manager.activate(Box::new(minibuffer));

        ctx.state
            .exec("set-mode", Some(vec![CommandArg::Mode(Mode::Minibuffer)]))?;

        Ok(())
    })
}

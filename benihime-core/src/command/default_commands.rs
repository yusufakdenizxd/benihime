use std::{
    cmp::{Ordering, min},
    fs,
    path::PathBuf,
};

use anyhow::{Ok, anyhow};
use ignore::Walk;

use crate::{
    buffer::{Buffer, Mode, Selection},
    editor::HandleKeyError,
    mini_buffer::{MiniBuffer, MinibufferCallbackResult},
    project::Project,
};
use crate::{editor_state::EditorState, movement::movement_commands};

use super::{
    command_registry::CommandRegistry,
    {CommandArg, CommandContext},
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
        if buf.mode != Mode::Insert && mode == Mode::Insert {
            buf.undo_tree.commit_group();
        }

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
        let line = buf.line(buf.cursor.row);
        let mut i = 0;
        for (idx, char) in line.chars().enumerate() {
            if !char.is_whitespace() {
                i = idx;
                break;
            }
            i = idx + 1;
        }
        buf.cursor.col = i;

        Ok(())
    });

    registry.register("open-above", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        let char_idx = buf.get_cursor_to_char();
        buf.insert_idx(char_idx, "\n");
        buf.cursor.col = 0;
        ctx.state
            .exec("set-mode", Some(vec![CommandArg::Mode(Mode::Insert)]))?;
        Ok(())
    });

    registry.register("open-below", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        let char_idx = buf.get_line_to_char(buf.cursor.row + 1);
        buf.insert_idx(char_idx, "\n");
        buf.cursor.row += 1;
        buf.cursor.col = 0;

        ctx.state
            .exec("set-mode", Some(vec![CommandArg::Mode(Mode::Insert)]))?;

        Ok(())
    });

    registry.register_motion("word-forward", |ctx: &mut CommandContext| {
        movement_commands::move_next_word_start(ctx);

        Ok(())
    });

    registry.register_motion("word-backward", |ctx: &mut CommandContext| {
        movement_commands::move_prev_word_start(ctx);

        Ok(())
    });

    registry.register_motion("word-end", |ctx: &mut CommandContext| {
        movement_commands::move_next_word_end(ctx);

        Ok(())
    });

    registry.register_motion("word-backward-end", |ctx: &mut CommandContext| {
        movement_commands::move_prev_word_end(ctx);
        Ok(())
    });

    registry.register_motion("long-word-forward", |ctx: &mut CommandContext| {
        movement_commands::move_next_long_word_start(ctx);
        Ok(())
    });

    registry.register_motion("long-word-forward-end", |ctx: &mut CommandContext| {
        movement_commands::move_next_long_word_end(ctx);
        Ok(())
    });

    registry.register_motion("long-word-backward", |ctx: &mut CommandContext| {
        movement_commands::move_prev_long_word_start(ctx);
        Ok(())
    });

    registry.register_motion("long-word-backward-end", |ctx: &mut CommandContext| {
        movement_commands::move_prev_long_word_end(ctx);
        Ok(())
    });

    registry.register_motion("sub-word-forward", |ctx: &mut CommandContext| {
        movement_commands::move_next_sub_word_start(ctx);
        Ok(())
    });

    registry.register_motion("sub-word-forward-end", |ctx: &mut CommandContext| {
        movement_commands::move_next_sub_word_end(ctx);
        Ok(())
    });

    registry.register_motion("sub-word-backward", |ctx: &mut CommandContext| {
        movement_commands::move_prev_sub_word_start(ctx);
        Ok(())
    });

    registry.register_motion("sub-word-backward-end", |ctx: &mut CommandContext| {
        movement_commands::move_prev_sub_word_end(ctx);
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

        let args: Vec<CommandArg> = parts.map(CommandArg::parse_arg).collect();

        ctx.state.exec(command_name, Some(args))?;
        Ok(())
    });

    registry.register("open-file", |ctx: &mut CommandContext| {
        let cwd = match ctx.state.cwd() {
            Some(v) => v,
            None => return Err(anyhow!("Not in a Project")),
        };

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
                    let id = state.open_file(&path.clone());
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
        let cwd = match ctx.state.cwd() {
            Some(v) => v,
            None => return Err(anyhow!("Not in a Project")),
        };

        let files: Vec<PathBuf> = Walk::new(cwd)
            .filter_map(Result::ok)
            .filter(|e| e.file_type().unwrap().is_file())
            .map(|x| x.path().to_owned())
            .collect();

        let minibuffer: MiniBuffer<PathBuf> = MiniBuffer::new(
            "Find File: ",
            files,
            |state: &mut EditorState, path: &PathBuf| {
                let id = state.open_file(&path.clone());
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

    registry.register_system("clear-error-message", |ctx: &mut CommandContext| {
        let state = &mut ctx.state;
        state.error_message = None;

        Ok(())
    });

    registry.register("next-buffer", |ctx: &mut CommandContext| {
        let state = &mut ctx.state;
        let focused_id = state.focused_buf_id;
        let ids = &state.project_manager.current().buffers;

        if ids.is_empty() {
            return Ok(());
        }

        let next = match ids.iter().position(|&id| id == focused_id) {
            Some(i) => ids[(i + 1) % ids.len()],
            None => ids[0],
        };

        state.focused_buf_id = next;

        Ok(())
    });

    registry.register("previous-buffer", |ctx: &mut CommandContext| {
        let state = &mut ctx.state;
        let focused_id = state.focused_buf_id;
        let ids = &state.project_manager.current().buffers;

        if ids.is_empty() {
            return Ok(());
        }

        let prev = match ids.iter().position(|&id| id == focused_id) {
            Some(i) => ids[(i + ids.len() - 1) % ids.len()],
            None => ids[0],
        };

        state.focused_buf_id = prev;

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
            match result {
                MinibufferCallbackResult::NewItems => {
                    ctx.state.minibuffer_manager.current = Some(mini);
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
                let _ = state.exec(command_name, None);
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
        state.kill_active_buffer()?;
        Ok(())
    });

    registry.register("enter-visual-mode", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        if buf.mode != Mode::Visual {
            buf.selection = Some(Selection { start: buf.cursor });
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
    });

    registry.register_operator("delete-range", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        let row = buf.cursor.row;
        let line_start = buf.get_line_to_char(row);
        let line_end = buf.get_line_to_char(row + 1);
        let line_len = line_end - line_start;

        if let Some(range) = buf.range.take() {
            let start = range.anchor.min(range.head);
            let end = range.anchor.max(range.head);
            let del_start = line_start + start.min(line_len);
            let del_end = line_start + end.min(line_len);
            buf.remove_line(del_start, del_end);
            buf.cursor.col = start;
        } else if line_len > 0 {
            let col = buf.cursor.col.min(line_len.saturating_sub(1));
            let del_start = line_start + col;
            let del_end = del_start + 1;
            buf.remove_line(del_start, del_end);
        }

        Ok(())
    });

    registry.register_operator("change-range", |ctx: &mut CommandContext| {
        let buf = ctx.state.focused_buf_mut();
        let row = buf.cursor.row;
        let line_start = buf.get_line_to_char(row);
        let line_end = buf.get_line_to_char(row + 1);
        let line_len = line_end - line_start;

        if let Some(range) = buf.range.take() {
            let start = range.anchor.min(range.head);
            let end = range.anchor.max(range.head);
            let del_start = line_start + start.min(line_len);
            let del_end = line_start + end.min(line_len);
            buf.remove_line(del_start, del_end);
            buf.cursor.col = start;
            buf.mode = Mode::Insert;
        }

        Ok(())
    });

    registry.register("undo", |ctx: &mut CommandContext| {
        ctx.state.focused_buf_mut().undo();
        Ok(())
    });

    registry.register("redo", |ctx: &mut CommandContext| {
        ctx.state.focused_buf_mut().redo();
        Ok(())
    });

    registry.register("undo-tree-show", |ctx| {
        let tree_text = {
            let buf = ctx.state.focused_buf();
            buf.undo_tree.render()
        };

        let id = ctx.state.create_buffer_from_text("*undo-tree*", &tree_text);

        ctx.state.focused_buf_id = id;
        Ok(())
    });

    registry.register("scroll-half-down", |ctx| {
        let h = ctx.state.screen_height / 2;
        let buf = ctx.state.focused_buf_mut();
        buf.scroll_down(h);
        Ok(())
    });

    registry.register("scroll-half-up", |ctx| {
        let h = ctx.state.screen_height / 2;
        let buf = ctx.state.focused_buf_mut();
        buf.scroll_up(h);
        Ok(())
    });

    registry.register("scroll-full-down", |ctx| {
        let h = ctx.state.screen_height;
        let buf = ctx.state.focused_buf_mut();
        buf.scroll_down(h);
        Ok(())
    });

    registry.register("scroll-full-up", |ctx| {
        let h = ctx.state.screen_height;
        let buf = ctx.state.focused_buf_mut();
        buf.scroll_up(h);
        Ok(())
    });
    registry.register("goto-first-line", |ctx| {
        let buf = ctx.state.focused_buf_mut();
        buf.goto_first_line();
        Ok(())
    });

    registry.register("goto-last-line", |ctx| {
        let buf = ctx.state.focused_buf_mut();
        buf.goto_last_line();
        Ok(())
    });

    registry.register("save-current-buffer", |ctx| {
        let buf = ctx.state.focused_buf_mut();
        let _ = buf.save();
        Ok(())
    });

    registry.register("open-project", |ctx: &mut CommandContext| {
        let projects = ctx.state.project_manager.get_projects_cloned();

        let minibuffer: MiniBuffer<Project> = MiniBuffer::new(
            "Open Project: ",
            projects,
            |state: &mut EditorState, project: &Project| {
                state.switch_project(project.id);
                Ok(None)
            },
        );

        ctx.state.minibuffer_manager.activate(Box::new(minibuffer));

        ctx.state
            .exec("set-mode", Some(vec![CommandArg::Mode(Mode::Minibuffer)]))?;

        Ok(())
    });

    registry.register("kill-current-project", |ctx: &mut CommandContext| {
        ctx.state.close_current_project()
    });
}

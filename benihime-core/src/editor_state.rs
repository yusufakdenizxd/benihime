use anyhow::anyhow;
use std::{fs, path::PathBuf, sync::Arc};

use crate::{
    buffer::{Buffer, BufferId, Mode},
    buffer_manager::BufferManager,
    command::{CommandArg, CommandContext, command_registry::CommandRegistry},
    editor::HandleKeyError,
    keymap::Keymap,
    mini_buffer::MiniBufferManager,
    project::{
        ProjectId,
        project_manager::{DEFAULT_PROJECT_ID, ProjectManager},
    },
    theme::{Theme, theme_loader::ThemeLoader},
};

pub struct EditorState {
    pub focused_buf_id: BufferId,
    pub project_manager: ProjectManager,
    pub buffer_manager: BufferManager,
    pub minibuffer_manager: MiniBufferManager,
    pub screen_height: usize,
    pub screen_width: usize,
    pub command_buffer: String,
    pub message: Option<String>,
    pub error_message: Option<String>,
    pub registry: Arc<CommandRegistry>,
    pub theme: Theme,
    pub theme_loader: Arc<ThemeLoader>,
    pub prefix_arg: Option<usize>,
    pub keymap: Keymap,
}

impl EditorState {
    pub fn cwd(&self) -> Option<&PathBuf> {
        self.project_manager.current().root.as_ref()
    }

    pub fn focused_buf_mut(&mut self) -> &mut Buffer {
        self.buffer_manager
            .get_buffer_mut(self.focused_buf_id)
            .expect("Focused buffer must exist")
    }

    pub fn focused_buf(&self) -> &Buffer {
        self.buffer_manager
            .get_buffer(self.focused_buf_id)
            .expect("Focused buffer must exist")
    }

    pub fn status_line(&self) -> String {
        let buf = self.buffer_manager.get_buffer(self.focused_buf_id).unwrap();
        let mode = match buf.mode {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
            Mode::Command => "COMMAND",
            Mode::Minibuffer => "MINIBUFFER",
        };
        format!("{} {}", mode, buf.id)
    }

    pub fn buffer_line(&self) -> Vec<(BufferId, String, bool, bool)> {
        self.project_manager
            .current()
            .buffers
            .iter()
            .filter_map(|f| self.buffer_manager.get_buffer(*f))
            .map(|buf| {
                let is_active = buf.id == self.focused_buf_id;
                (buf.id, buf.name.clone(), is_active, buf.is_modified())
            })
            .collect()
    }

    pub fn kill_active_buffer(&mut self) -> anyhow::Result<()> {
        let project = self.project_manager.current_mut();

        if project.buffers.is_empty() {
            return Ok(());
        }

        let buf_id_to_kill = self.focused_buf_id;

        let buf = self
            .buffer_manager
            .get_buffer(buf_id_to_kill)
            .ok_or_else(|| anyhow!("Buffer not found"))?;

        if buf.is_modified() {
            return Err(anyhow!(
                "Cannot kill modified buffer '{}'. Save it first.",
                buf.name
            ));
        }

        project.buffers.retain(|&id| id != buf_id_to_kill);

        self.buffer_manager.kill_buffer(buf_id_to_kill);

        if let Some(&new_focus) = project.buffers.last() {
            self.focused_buf_id = new_focus;
        } else {
            let new_buf = self.create_empty_buffer("[No Name]");
            self.focused_buf_id = new_buf;
        }

        Ok(())
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    pub fn exec(
        &mut self,
        name: &str,
        args: Option<Vec<CommandArg>>,
    ) -> Result<(), HandleKeyError> {
        let count = self.prefix_arg.take().unwrap_or(1);

        let registry = Arc::clone(&self.registry);
        registry.execute(
            name,
            &mut CommandContext {
                state: self,
                args: &args,
                count,
            },
        )
    }

    pub fn switch_project(&mut self, project_id: ProjectId) {
        let project = self.project_manager.switch_by_id(project_id);
        if let Some(project) = project {
            if project.buffers.is_empty() {
                let buf = self.create_empty_buffer("[No Name]");
                self.focused_buf_id = buf;
            } else {
                self.focused_buf_id = project.buffers[0];
            }
        }
    }

    pub fn create_empty_buffer(&mut self, name: &str) -> BufferId {
        let id = self.buffer_manager.create_empty_buffer(name);
        self.project_manager.add_buffer_to_current(id);
        id
    }

    pub fn create_buffer_from_text(&mut self, name: &str, text: &str) -> BufferId {
        let id = self.buffer_manager.create_buffer_from(name, text, None);

        self.project_manager.add_buffer_to_current(id);
        id
    }

    pub fn create_read_only_buffer_from_text(&mut self, name: &str, text: &str) -> BufferId {
        let id = self.buffer_manager.create_read_only_buffer_from(name, text);

        self.project_manager.add_buffer_to_current(id);
        id
    }

    pub fn open_file(&mut self, path: &PathBuf) -> BufferId {
        let contents = fs::read_to_string(path).unwrap_or_default();

        let id = self.buffer_manager.create_buffer_from(
            path.file_name().unwrap().to_str().unwrap(),
            &contents,
            Some(path),
        );

        self.project_manager.add_buffer_to_current(id);
        id
    }

    pub fn close_current_project(&mut self) -> anyhow::Result<()> {
        let id = self.project_manager.current_id();

        let killed_buffers = self
            .project_manager
            .close_project(id, &mut self.buffer_manager)?;

        for buf_id in &killed_buffers {
            self.buffer_manager.kill_buffer(*buf_id);
        }

        if self.project_manager.current_id() == DEFAULT_PROJECT_ID {
            let last_project = self.project_manager.current();

            if last_project.buffers.is_empty() {
                let new_buf = self.create_empty_buffer("[No Name]");
                self.focused_buf_id = new_buf;
            } else {
                self.focused_buf_id = last_project.buffers[0];
            }
        }

        Ok(())
    }

    pub fn switch_to_next_project(&mut self) -> anyhow::Result<()> {
        if self.project_manager.len() == 0 {
            return Err(anyhow!("Can't find next project"));
        }
        let next_id = self.project_manager.next_project_id();
        self.switch_to_project(next_id);
        Ok(())
    }

    pub fn switch_to_previous_project(&mut self) -> anyhow::Result<()> {
        if self.project_manager.len() == 0 {
            return Err(anyhow!("Can't find next project"));
        }
        let prev_id = self.project_manager.previous_project_id();
        self.switch_to_project(prev_id);
        Ok(())
    }

    pub fn switch_to_project(&mut self, id: ProjectId) {
        self.project_manager.switch_by_id(id);
        let project = self.project_manager.current();

        if project.buffers.is_empty() {
            let buf = self.create_empty_buffer("[No Name]");
            self.focused_buf_id = buf;
        } else {
            self.focused_buf_id = project.buffers[0];
        }
    }

    pub fn clear_prefix(&mut self) {
        self.prefix_arg = None;
    }
}

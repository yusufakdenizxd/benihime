use anyhow::anyhow;
use std::{path::PathBuf, sync::Arc};

use crate::{
    buffer::{Buffer, BufferId, Mode},
    buffer_manager::BufferManager,
    command::{CommandArg, CommandContext, command_registry::CommandRegistry},
    editor::HandleKeyError,
    mini_buffer::MiniBufferManager,
    project::{Project, project_manager::ProjectManager},
    theme::{Theme, theme_loader::ThemeLoader},
};

pub struct EditorState {
    pub focused_buf_id: BufferId,
    pub project_manager: ProjectManager,
    pub cwd: Option<PathBuf>,
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
}

impl EditorState {
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
        self.buffer_manager
            .iter_buffers()
            .map(|(id, buf)| {
                let is_active = *id == self.focused_buf_id;
                (*id, buf.name.clone(), is_active, buf.is_modified())
            })
            .collect()
    }

    pub fn kill_active_buffer(&mut self) -> anyhow::Result<()> {
        let len = self.buffer_manager.buffers_len();
        if len == 0 {
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

        if len == 1 {
            self.buffer_manager.create_empty_buffer("[No Name]");
        }

        self.buffer_manager.kill_buffer(buf_id_to_kill);

        let buffer_ids = self.buffer_manager.get_buffer_ids();
        if let Some(new_focus_id) = buffer_ids.iter().filter(|id| ***id != buf_id_to_kill).max() {
            self.focused_buf_id = **new_focus_id;
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
        let registry = Arc::clone(&self.registry);
        registry.execute(
            name,
            &mut CommandContext {
                state: self,
                args: &args,
            },
        )
    }

    pub fn switch_project(&mut self, project: Project) {
        let project = self.project_manager.switch(project);
        if let Some(project) = project {
            self.cwd = Some(project.root.clone());
        }
    }
}

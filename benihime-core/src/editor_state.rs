use anyhow::anyhow;
use std::{fs, path::PathBuf, sync::Arc};

use crate::{
    buffer::{Buffer, BufferId, Mode},
    buffer_manager::BufferManager,
    command::{CommandArg, CommandContext, command_registry::CommandRegistry},
    editor::HandleKeyError,
    mini_buffer::MiniBufferManager,
    project::{Project, ProjectId, project_manager::ProjectManager},
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
            self.create_empty_buffer("[No Name]");
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

    pub fn switch_project(&mut self, project_id: ProjectId) {
        let project = self.project_manager.switch_by_id(project_id);
        if let Some(project) = project {
            self.cwd = Some(project.root.clone());
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
}

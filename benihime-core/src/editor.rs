use anyhow::anyhow;
use std::{collections::BTreeMap, fs, path::PathBuf, str::FromStr, sync::Arc};

use crate::{
    application::HandleKeyError,
    buffer::{Buffer, BufferId},
    command::{CommandArg, CommandContext, command_registry::CommandRegistry},
    graphics::Rect,
    keymap::Keymap,
    mini_buffer::MiniBufferManager,
    project::{
        ProjectId,
        project_manager::{DEFAULT_PROJECT_ID, ProjectManager},
    },
    theme::{Theme, theme_loader::ThemeLoader},
    tree::Tree,
    window::Window,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    Command,
    Minibuffer,
}

impl FromStr for Mode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(Mode::Normal),
            "insert" => Ok(Mode::Insert),
            "visual" => Ok(Mode::Visual),
            "command" => Ok(Mode::Command),
            _ => Err(anyhow!("Invalid mode: {}", s)),
        }
    }
}

pub struct Editor {
    pub focused_buf_id: BufferId,
    pub project_manager: ProjectManager,
    pub minibuffer_manager: MiniBufferManager,
    pub screen_height: usize,
    pub screen_width: usize,
    pub command_buffer: String,
    pub message: Option<String>,
    pub error_message: Option<String>,
    pub registry: Arc<CommandRegistry>,
    theme: Theme,
    theme_loader: Arc<ThemeLoader>,
    pub prefix_arg: Option<usize>,
    pub keymap: Keymap,

    tree: Tree,
    buffers: BTreeMap<BufferId, Buffer>,
    next_buffer_id: BufferId,

    write_count: usize,

    pub needs_redraw: bool,
    pub config: Arc<EditorConfig>,
}

impl Editor {
    pub fn new(
        area: Rect,
        theme_loader: ThemeLoader,
        project_manager: ProjectManager,

        keymap: Keymap,
        registry: Arc<CommandRegistry>,
        config: Arc<EditorConfig>,
    ) -> Self {
        Self {
            focused_buf_id: BufferId(0),
            project_manager,
            command_buffer: String::new(),
            message: None,
            error_message: None,
            screen_height: 0,
            screen_width: 0,
            minibuffer_manager: MiniBufferManager::new(),
            registry,
            theme: theme_loader.default(),
            theme_loader: Arc::new(theme_loader),
            prefix_arg: None,
            keymap,
            write_count: 0,
            needs_redraw: false,
            config,
            tree: Tree::new(area),
            buffers: BTreeMap::new(),
            next_buffer_id: BufferId(1),
        }
    }

    pub fn cwd(&self) -> Option<&PathBuf> {
        self.project_manager.current().root.as_ref()
    }

    pub fn status_line(&self) -> String {
        let (window, buf) = self.focus_ref();
        let mode = match window.mode {
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
            .filter_map(|f| self.buffers.get(f))
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
            .buffers
            .get(&buf_id_to_kill)
            .ok_or_else(|| anyhow!("Buffer not found"))?;

        if buf.is_modified() {
            return Err(anyhow!(
                "Cannot kill modified buffer '{}'. Save it first.",
                buf.name
            ));
        }

        project.buffers.retain(|&id| id != buf_id_to_kill);

        self.buffers.remove(&buf_id_to_kill);

        if let Some(&new_focus) = project.buffers.last() {
            self.focused_buf_id = new_focus;
        } else {
            let new_buf = self.new_empty_buffer("[No Name]");
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
                editor: self,
                args: &args,
                count,
            },
        )?;

        self.needs_redraw = true;
        Ok(())
    }

    pub fn switch_project(&mut self, project_id: ProjectId) {
        let project = self.project_manager.switch_by_id(project_id);
        if let Some(project) = project {
            if project.buffers.is_empty() {
                let buf = self.new_empty_buffer("[No Name]");
                self.focused_buf_id = buf;
            } else {
                self.focused_buf_id = project.buffers[0];
            }
        }
    }

    pub fn open_file(&mut self, path: &PathBuf) -> BufferId {
        let contents = fs::read_to_string(path).unwrap_or_default();

        let id = self.new_buffer_from_text(
            path.file_name().unwrap().to_str().unwrap(),
            &contents,
            Some(path),
        );

        self.project_manager.add_buffer_to_current(id);
        id
    }

    pub fn close_current_project(&mut self) -> anyhow::Result<()> {
        let id = self.project_manager.current_id();

        let killed_buffers = self.project_manager.close_project(id)?;

        for buf_id in &killed_buffers {
            self.buffers.remove(buf_id);
        }

        if self.project_manager.current_id() == DEFAULT_PROJECT_ID {
            let last_project = self.project_manager.current();

            if last_project.buffers.is_empty() {
                let new_buf = self.new_empty_buffer("[No Name]");
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
            let buf = self.new_empty_buffer("[No Name]");
            self.focused_buf_id = buf;
        } else {
            self.focused_buf_id = project.buffers[0];
        }
    }

    pub fn clear_prefix(&mut self) {
        self.prefix_arg = None;
    }

    pub async fn flush_writes(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn set_error(&mut self, error: String) {
        self.error_message = Some(error);
    }

    pub fn set_status(&mut self, status: String) {
        self.message = Some(status);
    }

    pub fn mode(&self) -> Mode {
        self.focus_ref().0.mode
    }

    pub fn new_empty_buffer(&mut self, name: &str) -> BufferId {
        let buf = Buffer::new(BufferId(0), name, None, false);
        self.new_buffer(buf)
    }

    pub fn new_read_only_buffer_from_text(&mut self, name: &str, text: &str) -> BufferId {
        let buf = Buffer::from(BufferId(0), name, text, None, true);
        self.new_buffer(buf)
    }

    pub fn new_buffer_from_text(
        &mut self,
        name: &str,
        text: &str,
        file_path: Option<&PathBuf>,
    ) -> BufferId {
        let buf = Buffer::from(BufferId(0), name, text, file_path.cloned(), false);
        self.new_buffer(buf)
    }

    fn new_buffer(&mut self, mut buf: Buffer) -> BufferId {
        let id = BufferId(self.next_buffer_id.0);
        self.next_buffer_id = BufferId(self.next_buffer_id.0 + 1);
        buf.id = id;

        self.buffers.insert(id, buf);
        self.focus_buf(id);

        id
    }

    pub fn focused_buf_mut(&mut self) -> &mut Buffer {
        self.buffers
            .get_mut(&self.focused_buf_id)
            .expect("Focused buffer not exist")
    }

    pub fn focus(&mut self) -> (&mut Window, &mut Buffer) {
        let view = self.tree.get_mut(self.tree.focus);
        let buf = self
            .buffers
            .get_mut(&self.focused_buf_id)
            .expect("Focus buffer didn't found");

        return (view, buf);
    }

    pub fn focus_ref(&self) -> (&Window, &Buffer) {
        let view = self.tree.get(self.tree.focus);
        let buf = &self.buffers[&self.focused_buf_id];

        return (view, buf);
    }

    pub fn update_scroll(&mut self) {
        let screen_height = self.screen_height;
        let screen_width = self.screen_width;
        let offset = self.config.scroll_offset;

        let (window, _buf) = self.focus();
        window.update_scroll(screen_height, offset, screen_width, offset);
    }

    pub fn get_buffers_cloned(&self) -> Vec<Buffer> {
        self.buffers.values().cloned().collect()
    }

    pub fn focus_buf(&mut self, buf_id: BufferId) {
        let id = self.tree.insert(Window::new(buf_id));
        self.tree.focus = id;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EditorConfig {
    pub smooth_scroll_enabled: bool,
    pub scroll_lerp_factor: f32,
    pub scroll_min_step_lines: f32,
    pub scroll_min_step_cols: f32,
    pub scroll_lines: isize,
    pub scroll_offset: usize,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            smooth_scroll_enabled: true,
            scroll_lerp_factor: 0.25,
            scroll_min_step_lines: 0.75,
            scroll_min_step_cols: 1.0,
            scroll_lines: 3,
            scroll_offset: 8,
        }
    }
}

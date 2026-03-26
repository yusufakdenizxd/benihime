use std::sync::Arc;

use benihime_loader::paths;
use benihime_renderer::{
    Renderer,
    event::{InputEvent, Key, ScrollDelta},
};
use thiserror::Error;

use crate::{
    buffer::{BufferId, Mode},
    buffer_manager::BufferManager,
    command::{self, command_registry::CommandRegistry},
    editor::{Editor, EditorConfig},
    graphics::Rect,
    input_handler::InputHandler,
    keymap::{
        self, Keymap,
        key_chord::{KeyChord, KeyModifiers},
    },
    mini_buffer::MiniBufferManager,
    project::project_manager::ProjectManager,
    theme::theme_loader::ThemeLoader,
    ui::{
        components::buffer_line::BufferLine,
        components::cursor::CursorComponent,
        components::mini_buffer::MiniBufferComponent,
        components::status_line::StatusLine,
        composer::{Composer, Context, Event},
        editor_view::EditorView,
        job::Jobs,
    },
};

#[derive(Debug, Error)]
pub enum HandleKeyError {
    #[error("Key not found in keymap")]
    KeyNotFound,
    #[error("Command not found: {0}")]
    CommandNotFound(String),
    #[error("Command execution failed: {0}")]
    ExecutionFailed(#[from] anyhow::Error),
}

impl PartialEq for HandleKeyError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::KeyNotFound, Self::KeyNotFound) => true,
            (Self::CommandNotFound(s1), Self::CommandNotFound(s2)) => s1 == s2,
            (Self::ExecutionFailed(e1), Self::ExecutionFailed(e2)) => {
                e1.to_string() == e2.to_string()
            }
            _ => false,
        }
    }
}

pub struct Application {
    pub composer: Composer,
    pub jobs: Jobs,
    pub editor: Editor,
    pub input_handler: InputHandler,

    scroll_lerp_factor: f32,
    scroll_min_step_lines: f32,
    scroll_min_step_cols: f32,

    pending_scroll_lines: f32,
    pending_scroll_cols: f32,

    trackpad_scroll_lines: f32,
    trackpad_scroll_cols: f32,

    last_frame_time: std::time::Instant,
}

impl Application {
    pub fn new() -> Self {
        let loader = benihime_loader::Loader::new().unwrap();

        let buffer_manager = BufferManager::new();

        let mut command_registry = CommandRegistry::new();
        command::default_commands::register_default_commands(&mut command_registry);

        let theme_loader = ThemeLoader::new(loader.paths.themes_dir());

        let mut keymap = Keymap::new();
        keymap::default_keymap::register_default_keymap(&mut keymap);

        let mut project_manager = ProjectManager::new();

        if let Ok(dir) = paths::home_dir() {
            let projects_dir = dir.join("dev");
            project_manager.discover_in_path(&projects_dir);
        }

        let area = Rect::new(0, 0, 120, 40);
        let mut composer = Composer::new(area);

        composer.push(Box::new(BufferLine::new()));
        composer.push(Box::new(EditorView::new()));
        composer.push(Box::new(StatusLine::new()));
        composer.push(Box::new(CursorComponent::new()));
        composer.push(Box::new(MiniBufferComponent::new()));

        let c = EditorConfig::default();
        let mut editor = Editor {
            focused_buf_id: BufferId(0),
            project_manager,
            buffer_manager,
            command_buffer: String::new(),
            message: None,
            error_message: None,
            screen_height: 0,
            screen_width: 0,
            minibuffer_manager: MiniBufferManager::new(),
            registry: Arc::new(command_registry),
            theme: theme_loader.default(),
            theme_loader: Arc::new(theme_loader),
            prefix_arg: None,
            keymap,
            write_count: 0,
            needs_redraw: false,
            config: Arc::new(c),
        };

        let first_id = editor.create_buffer_from_text(
            "[No Name]",
            "Welcome to Benihime!\n\nType something here...",
        );
        editor.focused_buf_id = first_id;

        let mode = editor.mode();

        let config = Arc::clone(&editor.config);

        Self {
            editor,
            composer,
            jobs: Jobs::new(),

            scroll_lerp_factor: config.scroll_lerp_factor,
            scroll_min_step_lines: config.scroll_min_step_lines,
            scroll_min_step_cols: config.scroll_min_step_cols,
            pending_scroll_lines: 0.0,

            pending_scroll_cols: 0.0,
            trackpad_scroll_lines: 0.0,
            trackpad_scroll_cols: 0.0,
            last_frame_time: std::time::Instant::now(),

            input_handler: InputHandler::new(mode),
        }
    }

    pub fn handle_key(&mut self, key: Key, modifiers: KeyModifiers) {
        let state = &mut self.editor;
        let buf_mode = {
            let buf = state.focused_buf();
            buf.mode
        };

        self.handle_key_with_mode(key, modifiers, buf_mode);
    }

    pub fn handle_key_with_mode(&mut self, key: Key, modifiers: KeyModifiers, mode: Mode) {
        let state = &mut self.editor;
        let buf_mode = mode;

        let chord = KeyChord {
            code: key,
            modifiers,
        };

        if buf_mode == Mode::Normal
            && let Some(digit) = chord.as_digit()
        {
            state.prefix_arg = Some(state.prefix_arg.unwrap_or(0) * 10 + digit);
            return;
        }

        if let Some((command_name, args)) = state.keymap.push_key(buf_mode, &chord) {
            if let Err(e) = state.exec(&command_name, args) {
                log::error!("handle_key: exec error={}", e);
            }
            state.clear_prefix();
            state.needs_redraw = true;
            return;
        }

        match buf_mode {
            Mode::Insert => {
                let buf = state.focused_buf_mut();
                let result = if chord.code == Key::Backspace {
                    buf.delete_char_before_cursor();
                    Ok(())
                } else if chord.code == Key::Enter {
                    buf.insert_char('\n')
                } else if let Some(c) = chord.as_char() {
                    buf.insert_char(c)
                } else {
                    Ok(())
                };

                if let Err(err) = result {
                    state.error_message = Some(err.to_string());
                }
            }
            Mode::Command => {
                if chord.code == Key::Backspace {
                    state.command_buffer.pop();
                } else if let Some(c) = chord.as_char() {
                    state.command_buffer.push(c);
                }
                state.needs_redraw = true;
            }
            Mode::Minibuffer => {
                if let Some(mini) = state.minibuffer_manager.current.as_mut() {
                    if chord.code == Key::Backspace {
                        mini.input_mut().pop();
                    } else if let Some(c) = chord.as_char() {
                        mini.input_mut().push(c);
                    }
                    mini.filter_items();
                    state.needs_redraw = true;
                }
            }
            _ => {}
        }
    }
}

impl benihime_renderer::Application for Application {
    fn init(&mut self, renderer: &mut Renderer) {
        if let Ok(path) = std::env::var("THE_EDITOR_FONT_FILE")
            && let Err(err) = renderer.configure_font_from_path(&path, 32.0)
        {
            log::warn!("failed to load font from THE_EDITOR_FONT_FILE={path}: {err}");
        }
    }

    fn render(&mut self, renderer: &mut Renderer) {
        benihime_event::start_frame();

        self.editor.needs_redraw = false;

        while let Ok(status) = self.jobs.status_messages.try_recv() {
            self.editor.set_status(status.message.to_string());
        }

        let now = std::time::Instant::now();
        let dt = now.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;

        let mut cx = Context {
            editor: &mut self.editor,
            scroll: None,
            jobs: &mut self.jobs,
            dt,
        };

        {
            let area = self.composer.size();
            self.composer.render(area, renderer, &mut cx);
        }
    }

    fn handle_event(&mut self, event: InputEvent, _renderer: &mut Renderer) -> bool {
        // let pending_char = self.composer.layers.iter().any(|layer| {
        //     layer
        //         .as_any()
        //         .downcast_ref::<crate::ui::editor_view::EditorView>()
        //         .is_some_and(|view| view.has_pending_on_next_key())
        // });
        //
        // if pending_char {
        //     self.input_handler.set_pending_char();
        // }

        let result = self.input_handler.handle_input(event.clone());

        self.input_handler.set_mode(self.editor.mode());

        if result.cancelled {
            return true;
        }

        if let Some(ch) = result.pending_char {
            let key = if ch == '\n' {
                Key::Enter
            } else {
                Key::Char(ch)
            };
            let binding = KeyChord::new(key);
            let event = Event::Key(binding.clone());

            let mut cx = Context {
                editor: &mut self.editor,
                scroll: None,
                jobs: &mut self.jobs,
                dt: 0.0,
            };

            if !self.composer.handle_event(&event, &mut cx) {
                self.handle_key(binding.code, binding.modifiers);
            }
            return true;
        }

        if let Some(ch) = result.insert_char {
            let binding = KeyChord::new(Key::Char(ch));
            let event = Event::Key(binding);

            let mut cx = Context {
                editor: &mut self.editor,
                scroll: None,
                jobs: &mut self.jobs,
                dt: 0.0,
            };

            if !self.composer.handle_event(&event, &mut cx) {
                self.handle_key(Key::Char(ch), KeyModifiers::NONE);
            }
            return true;
        }

        if let Some(binding) = result.command_key {
            let event = Event::Key(binding);

            let mut cx = Context {
                editor: &mut self.editor,
                scroll: None,
                jobs: &mut self.jobs,
                dt: 0.0,
            };

            return self.composer.handle_event(&event, &mut cx);
        }

        if let Some(scroll) = result.scroll {
            let event = Event::Scroll(scroll);
            let mut cx = Context {
                editor: &mut self.editor,
                scroll: None,
                jobs: &mut self.jobs,
                dt: 0.0,
            };
            let handled = self.composer.handle_event(&event, &mut cx);

            if !handled {
                let needs_immediate_redraw = self.handle_scroll(scroll, _renderer);
                return needs_immediate_redraw;
            }
            return true;
        }

        if let Some(mouse) = result.mouse {
            let event = Event::Mouse(mouse);

            let mut cx = Context {
                editor: &mut self.editor,
                scroll: None,
                jobs: &mut self.jobs,
                dt: 0.0,
            };

            return self.composer.handle_event(&event, &mut cx);
        }

        if let Some(keys) = result.keys
            && let Some(binding) = keys.last()
        {
            let event = Event::Key(binding.clone());

            let mode_before = self.editor.mode();

            let mut cx = Context {
                editor: &mut self.editor,
                scroll: None,
                jobs: &mut self.jobs,
                dt: 0.0,
            };

            if !self.composer.handle_event(&event, &mut cx) {
                let binding = binding.clone();
                self.handle_key_with_mode(binding.code, binding.modifiers, mode_before);
            }
            return true;
        }

        let consumed = match event {
            InputEvent::Text(text) => {
                for ch in text.chars() {
                    let binding = KeyChord::new(Key::Char(ch));
                    let event = Event::Key(binding);

                    let mut cx = Context {
                        editor: &mut self.editor,
                        scroll: None,
                        jobs: &mut self.jobs,
                        dt: 0.0,
                    };

                    if self.composer.handle_event(&event, &mut cx) {
                        return true;
                    }
                }
                false
            }
            _ => result.consumed,
        };

        self.input_handler.set_mode(self.editor.mode());
        consumed
    }

    fn resize(&mut self, width: u32, height: u32, renderer: &mut Renderer) {
        let area = Rect::new(0, 0, width as u16, height as u16);
        self.composer.resize(area);

        let cell_height = renderer.cell_height() as usize;
        let cell_width = renderer.cell_width() as usize;

        let buffer_line_height = cell_height;
        let status_line_height = cell_height;
        let editor_height = height as usize - buffer_line_height - status_line_height;

        self.editor.screen_height = editor_height / cell_height;
        self.editor.screen_width = width as usize / cell_width;
    }

    fn wants_redraw(&self) -> bool {
        if self.editor.needs_redraw {
            return true;
        }

        for layer in self.composer.layers.iter() {
            if layer.should_update() {
                return true;
            }
        }

        false
    }
}

impl Application {
    fn handle_scroll(&mut self, delta: ScrollDelta, renderer: &mut Renderer) -> bool {
        match delta {
            ScrollDelta::Lines { x, y } => {
                if Self::is_precision_line_scroll(x, y) {
                    self.handle_precise_line_scroll(x, y);
                    return false;
                }

                let config_lines = self.editor.config.scroll_lines.max(1) as f32;
                let d_cols = -x * 4.0;
                let d_lines = -y * config_lines;

                self.pending_scroll_lines += d_lines;
                self.pending_scroll_cols += d_cols;

                benihime_event::request_redraw();
                true
            }

            ScrollDelta::Pixels { x, y } => {
                self.handle_precise_pixel_scroll(x, y, renderer);
                false
            }
        }
    }

    fn handle_precise_pixel_scroll(&mut self, x: f32, y: f32, renderer: &Renderer) {
        let line_h = renderer.cell_height().max(1.0);
        let col_w = renderer.cell_width().max(1.0);

        let config_lines = self.editor.config.scroll_lines.max(1) as f32;
        let d_cols = (-x / col_w) * 4.0; // Same horizontal multiplier as mouse wheel
        let d_lines = (-y / line_h) * config_lines;

        self.accumulate_precise_scroll(d_lines, d_cols);
    }

    fn handle_precise_line_scroll(&mut self, x: f32, y: f32) {
        let config_lines = self.editor.config.scroll_lines.max(1) as f32;
        let d_cols = -x * 4.0;
        let d_lines = -y * config_lines;
        self.accumulate_precise_scroll(d_lines, d_cols);
    }

    fn accumulate_precise_scroll(&mut self, d_lines: f32, d_cols: f32) {
        self.trackpad_scroll_lines += d_lines;
        self.trackpad_scroll_cols += d_cols;

        let lines_to_scroll = self.trackpad_scroll_lines.trunc() as i32;
        let cols_to_scroll = self.trackpad_scroll_cols.trunc() as i32;

        self.trackpad_scroll_lines -= lines_to_scroll as f32;
        self.trackpad_scroll_cols -= cols_to_scroll as f32;

        if lines_to_scroll != 0 || cols_to_scroll != 0 {
            self.apply_scroll_immediate(lines_to_scroll, cols_to_scroll);
            self.editor.needs_redraw = true;
        }
    }

    fn is_precision_line_scroll(x: f32, y: f32) -> bool {
        const EPSILON: f32 = 1e-3;
        let is_fractional = |value: f32| {
            if value == 0.0 {
                return false;
            }
            (value - value.round()).abs() > EPSILON
        };

        is_fractional(x) || is_fractional(y)
    }

    fn apply_scroll_immediate(&mut self, lines: i32, cols: i32) {
        use crate::movement::movement::Direction;
        let config = Arc::clone(&self.editor.config);

        if lines != 0 {
            let _direction = if lines > 0 {
                Direction::Forward
            } else {
                Direction::Backward
            };

            // let mut cmd_cx = CommandContext {
            //     count: 1,
            //     editor: &mut self.editor,
            //     jobs: &mut self.jobs,
            //     args: &None,
            // };
            //
            // self.editor.exec("scroll", None);

            // TODO: check if required
            // commands::scroll(&mut cmd_cx, lines.unsigned_abs() as usize, direction, false);
        }

        let focus = self.editor.focused_buf_mut();

        focus.update_scroll(
            lines as usize,
            config.scroll_offset,
            cols as usize,
            config.scroll_offset,
        );

        // if cols != 0 {
        //     let focus = self.editor.focused_buf_mut();
        //
        //     if cols >= 0 {
        //         focus.scroll_offset = focus.scroll_offset.saturating_add(cols as usize);
        //     } else {
        //         focus.scroll_offset = focus.scroll_offset.saturating_sub((-cols) as usize);
        //     }
        //
        //     focus.update_scroll(
        //         lines as usize,
        //         config.scroll_offset,
        //         cols as usize,
        //         config.scroll_offset,
        //     );
        // }
    }
}

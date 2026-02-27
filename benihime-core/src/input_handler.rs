use benihime_renderer::event::{InputEvent, Key, KeyPress, MouseEvent, ScrollDelta};

use crate::{
    buffer::Mode,
    keymap::key_chord::{KeyChord, KeyModifiers},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnifiedKey {
    Character(char),
    Special(SpecialKey),
    Modified {
        key: char,
        shift: bool,
        ctrl: bool,
        alt: bool,
        super_key: bool,
    },
    ModifiedSpecial {
        key: SpecialKey,
        shift: bool,
        ctrl: bool,
        alt: bool,
        super_key: bool,
    },
    Escape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialKey {
    Enter,
    Tab,
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    Up,
    Down,
    Left,
    Right,
    F(u8),
}

#[derive(Debug, Clone)]
pub enum ProcessedInput {
    Key(UnifiedKey),
    Mouse(MouseEvent),
    Scroll(ScrollDelta),
}

#[derive(Debug, Clone)]
pub struct InputProcessor {
    mode: Mode,
    pending_char: bool,
    shift_held: bool,
    ctrl_held: bool,
    alt_held: bool,
    super_held: bool,
}

impl InputProcessor {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            pending_char: false,
            shift_held: false,
            ctrl_held: false,
            alt_held: false,
            super_held: false,
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn set_pending_char(&mut self, pending: bool) {
        self.pending_char = pending;
    }

    pub fn process(&mut self, event: InputEvent) -> Vec<ProcessedInput> {
        match event {
            InputEvent::Keyboard(key_press) => self.process_keyboard(key_press),
            InputEvent::Text(text) => self.process_text(text),
            InputEvent::Mouse(mouse) => vec![ProcessedInput::Mouse(mouse)],
            InputEvent::Scroll(delta) => vec![ProcessedInput::Scroll(delta)],
        }
    }

    fn process_keyboard(&mut self, key_press: KeyPress) -> Vec<ProcessedInput> {
        if !key_press.pressed {
            return vec![];
        }

        self.shift_held = key_press.modifier.shift;
        self.ctrl_held = key_press.modifier.control;
        self.alt_held = key_press.modifier.alt;
        self.super_held = key_press.modifier.super_key;

        let has_modifiers = key_press.has_modifier();

        let unified = match key_press.code {
            Key::Esc => Some(UnifiedKey::Escape),
            Key::Enter if has_modifiers => Some(UnifiedKey::ModifiedSpecial {
                key: SpecialKey::Enter,
                shift: key_press.modifier.shift,
                ctrl: key_press.modifier.control,
                alt: key_press.modifier.alt,
                super_key: key_press.modifier.super_key,
            }),
            Key::Tab if has_modifiers => Some(UnifiedKey::ModifiedSpecial {
                key: SpecialKey::Tab,
                shift: key_press.modifier.shift,
                ctrl: key_press.modifier.control,
                alt: key_press.modifier.alt,
                super_key: key_press.modifier.super_key,
            }),
            Key::Backspace if has_modifiers => Some(UnifiedKey::ModifiedSpecial {
                key: SpecialKey::Backspace,
                shift: key_press.modifier.shift,
                ctrl: key_press.modifier.control,
                alt: key_press.modifier.alt,
                super_key: key_press.modifier.super_key,
            }),
            Key::Delete if has_modifiers => Some(UnifiedKey::ModifiedSpecial {
                key: SpecialKey::Delete,
                shift: key_press.modifier.shift,
                ctrl: key_press.modifier.control,
                alt: key_press.modifier.alt,
                super_key: key_press.modifier.super_key,
            }),
            Key::Enter => Some(UnifiedKey::Special(SpecialKey::Enter)),
            Key::Tab => Some(UnifiedKey::Special(SpecialKey::Tab)),
            Key::Backspace => Some(UnifiedKey::Special(SpecialKey::Backspace)),
            Key::Delete => Some(UnifiedKey::Special(SpecialKey::Delete)),
            Key::Insert => Some(UnifiedKey::Special(SpecialKey::Insert)),
            Key::Home => Some(UnifiedKey::Special(SpecialKey::Home)),
            Key::End => Some(UnifiedKey::Special(SpecialKey::End)),
            Key::PageUp => Some(UnifiedKey::Special(SpecialKey::PageUp)),
            Key::PageDown => Some(UnifiedKey::Special(SpecialKey::PageDown)),
            Key::Up => Some(UnifiedKey::Special(SpecialKey::Up)),
            Key::Down => Some(UnifiedKey::Special(SpecialKey::Down)),
            Key::Left => Some(UnifiedKey::Special(SpecialKey::Left)),
            Key::Right => Some(UnifiedKey::Special(SpecialKey::Right)),
            Key::F(f) => Some(UnifiedKey::Special(SpecialKey::F(f))),
            Key::Char(ch) if key_press.has_modifier() => Some(UnifiedKey::Modified {
                key: ch,
                shift: key_press.modifier.shift,
                ctrl: key_press.modifier.control,
                alt: key_press.modifier.alt,
                super_key: key_press.modifier.super_key,
            }),
            Key::Char(ch) => {
                if self.pending_char {
                    Some(UnifiedKey::Character(ch))
                } else {
                    Some(UnifiedKey::Character(ch))
                }
            }
            _ => None,
        };

        unified
            .map(|key| vec![ProcessedInput::Key(key)])
            .unwrap_or_default()
    }

    fn process_text(&mut self, text: String) -> Vec<ProcessedInput> {
        let mut events = Vec::new();

        for ch in text.chars() {
            if self.mode == Mode::Insert || self.pending_char {
                events.push(ProcessedInput::Key(UnifiedKey::Character(ch)));
            } else if self.ctrl_held || self.alt_held {
                events.push(ProcessedInput::Key(UnifiedKey::Modified {
                    key: ch,
                    shift: self.shift_held,
                    ctrl: self.ctrl_held,
                    alt: self.alt_held,
                    super_key: self.super_held,
                }));
            } else {
                events.push(ProcessedInput::Key(UnifiedKey::Character(ch)));
            }
        }

        events
    }
}

impl UnifiedKey {
    pub fn to_key_binding(&self) -> Option<KeyChord> {
        match self {
            UnifiedKey::Character(ch) => Some(KeyChord::new(Key::Char(*ch))),
            UnifiedKey::Modified {
                key,
                shift,
                ctrl,
                alt,
                super_key,
            } => {
                Some(KeyChord::new(Key::Char(*key)).with_modifiers(*ctrl, *shift, *alt, *super_key))
            }
            UnifiedKey::Escape => Some(KeyChord::new(Key::Esc)),
            UnifiedKey::Special(special) => {
                let key = match special {
                    SpecialKey::Enter => Key::Enter,
                    SpecialKey::Tab => Key::Tab,
                    SpecialKey::Backspace => Key::Backspace,
                    SpecialKey::Delete => Key::Delete,
                    SpecialKey::Insert => Key::Insert,
                    SpecialKey::Home => Key::Home,
                    SpecialKey::End => Key::End,
                    SpecialKey::PageUp => Key::PageUp,
                    SpecialKey::PageDown => Key::PageDown,
                    SpecialKey::Up => Key::Up,
                    SpecialKey::Down => Key::Down,
                    SpecialKey::Left => Key::Left,
                    SpecialKey::Right => Key::Right,
                    SpecialKey::F(u) => Key::F(*u),
                };
                Some(KeyChord::new(key))
            }
            UnifiedKey::ModifiedSpecial {
                key,
                shift,
                ctrl,
                alt,
                super_key,
            } => {
                let key_code = match key {
                    SpecialKey::Enter => Key::Enter,
                    SpecialKey::Tab => Key::Tab,
                    SpecialKey::Backspace => Key::Backspace,
                    SpecialKey::Delete => Key::Delete,
                    SpecialKey::Insert => Key::Insert,
                    SpecialKey::Home => Key::Home,
                    SpecialKey::End => Key::End,
                    SpecialKey::PageUp => Key::PageUp,
                    SpecialKey::PageDown => Key::PageDown,
                    SpecialKey::Up => Key::Up,
                    SpecialKey::Down => Key::Down,
                    SpecialKey::Left => Key::Left,
                    SpecialKey::Right => Key::Right,
                    SpecialKey::F(u) => Key::F(*u),
                };
                let binding = KeyChord {
                    code: key_code,
                    modifiers: KeyModifiers {
                        shift: *shift,
                        control: *ctrl,
                        alt: *alt,
                        super_key: *super_key,
                    },
                };
                Some(binding)
            }
        }
    }
}

pub enum InputState {
    Normal,
    Insert,
    Select,
    PendingChar,
    CommandMode,
}

pub struct InputHandler {
    processor: InputProcessor,
    state: InputState,
}

impl InputHandler {
    pub fn new(mode: Mode) -> Self {
        let state = match mode {
            Mode::Normal => InputState::Normal,
            Mode::Insert => InputState::Insert,
            Mode::Minibuffer => InputState::Insert,
            Mode::Visual => InputState::Select,
            Mode::Command => InputState::CommandMode,
        };
        Self {
            processor: InputProcessor::new(mode),
            state,
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.processor.set_mode(mode);
        if !matches!(self.state, InputState::PendingChar) {
            self.state = match mode {
                Mode::Normal => InputState::Normal,
                Mode::Insert => InputState::Insert,
                Mode::Minibuffer => InputState::Insert,
                Mode::Visual => InputState::Select,
                Mode::Command => InputState::CommandMode,
            };
        }
    }

    pub fn set_pending_char(&mut self) {
        self.state = InputState::PendingChar;
        self.processor.set_pending_char(true);
    }

    pub fn clear_pending_char(&mut self) {
        self.state = InputState::Normal;
        self.processor.set_pending_char(false);
    }

    pub fn handle_input(&mut self, event: InputEvent) -> InputResult {
        let processed = self.processor.process(event);

        let mut result = InputResult::default();

        for input in processed {
            match input {
                ProcessedInput::Key(key) => match &mut self.state {
                    InputState::PendingChar => {
                        self.clear_pending_char();

                        match key {
                            UnifiedKey::Escape => {
                                result.cancelled = true;
                            }
                            UnifiedKey::Character(ch) => {
                                result.pending_char = Some(ch);
                                result.consumed = true;
                            }
                            UnifiedKey::Special(SpecialKey::Enter) => {
                                result.pending_char = Some('\n');
                                result.consumed = true;
                            }
                            _ => {
                                result.cancelled = true;
                            }
                        }
                    }
                    InputState::Normal | InputState::Select => {
                        if let Some(binding) = key.to_key_binding() {
                            result.keys = Some(vec![binding]);
                        }
                    }
                    InputState::CommandMode => {
                        if let Some(binding) = key.to_key_binding() {
                            result.command_key = Some(binding);
                        }
                    }
                    InputState::Insert => {
                        if let UnifiedKey::Character(ch) = key {
                            result.insert_char = Some(ch);
                        } else if let Some(binding) = key.to_key_binding() {
                            result.keys = Some(vec![binding]);
                        }
                    }
                },
                ProcessedInput::Mouse(mouse) => {
                    result.mouse = Some(mouse);
                }
                ProcessedInput::Scroll(delta) => {
                    result.scroll = Some(delta);
                }
            }
        }

        result
    }
}

#[derive(Debug, Default)]
pub struct InputResult {
    pub keys: Option<Vec<KeyChord>>,
    pub mouse: Option<MouseEvent>,
    pub scroll: Option<ScrollDelta>,
    pub pending_char: Option<char>,
    pub insert_char: Option<char>,
    pub command_key: Option<KeyChord>,
    pub consumed: bool,
    pub cancelled: bool,
}

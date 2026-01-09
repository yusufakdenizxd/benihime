#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyModifiers {
    pub shift: bool,
    pub alt: bool,
    pub super_key: bool,
    pub control: bool,
}

impl KeyModifiers {
    pub const NONE: Self = Self {
        alt: false,
        control: false,
        shift: false,
        super_key: false,
    };

    pub const ALT: Self = Self {
        alt: true,
        control: false,
        shift: false,
        super_key: false,
    };
    pub const CTRL: Self = Self {
        alt: false,
        control: true,
        shift: false,
        super_key: false,
    };
    pub const SHIFT: Self = Self {
        alt: false,
        control: false,
        shift: true,
        super_key: false,
    };

    pub const SUPER: Self = Self {
        alt: false,
        control: false,
        shift: false,
        super_key: true,
    };
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Null,
    Esc,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyChord {
    pub code: KeyCode,

    /// True if the key was pressed, false if released
    pub pressed: bool,

    pub modifiers: KeyModifiers,
}

impl KeyChord {
    pub fn code_input(code: KeyCode) -> Self {
        KeyChord {
            code,
            pressed: true,
            modifiers: KeyModifiers::NONE,
        }
    }

    pub fn code_input_super(code: KeyCode) -> Self {
        KeyChord {
            code,
            pressed: true,
            modifiers: KeyModifiers::SUPER,
        }
    }

    pub fn code_input_shift(code: KeyCode) -> Self {
        KeyChord {
            code,
            pressed: true,
            modifiers: KeyModifiers::SHIFT,
        }
    }

    pub fn code_input_ctrl(code: KeyCode) -> Self {
        KeyChord {
            code,
            pressed: true,
            modifiers: KeyModifiers::CTRL,
        }
    }

    pub fn as_char(&self) -> Option<char> {
        match self.code {
            KeyCode::Char(c) => {
                if self.modifiers.shift {
                    Some(c.to_ascii_uppercase())
                } else {
                    Some(c)
                }
            }
            _ => None,
        }
    }
}

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
    pub modifiers: KeyModifiers,
}

impl KeyChord {
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

    pub fn as_digit(&self) -> Option<usize> {
        match self.code {
            KeyCode::Char(c) if c.is_ascii_digit() => c.to_digit(10).map(|d| d as usize),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();

        if self.modifiers.control {
            parts.push("Ctrl".to_string());
        }
        if self.modifiers.alt {
            parts.push("Alt".to_string());
        }
        if self.modifiers.shift {
            parts.push("Shift".to_string());
        }
        if self.modifiers.super_key {
            parts.push("Super".to_string());
        }

        let key_str = match &self.code {
            KeyCode::Char(c) => c.to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Esc => "Esc".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            // fallback for all other keys
            other => format!("{:?}", other),
        };

        parts.push(key_str);

        parts.join("+")
    }
}

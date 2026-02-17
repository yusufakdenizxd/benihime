use benihime_renderer::event::Key;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyChord {
    pub code: Key,
    pub modifiers: KeyModifiers,
}

impl KeyChord {
    pub fn new(code: Key) -> KeyChord {
        KeyChord {
            code,
            modifiers: KeyModifiers::NONE,
        }
    }

    pub fn with_modifiers(
        mut self,
        control: bool,
        shift: bool,
        alt: bool,
        super_key: bool,
    ) -> Self {
        self.modifiers = KeyModifiers {
            control,
            shift,
            alt,
            super_key,
        };

        self
    }

    pub fn as_char(&self) -> Option<char> {
        match self.code {
            Key::Char(c) => {
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
            Key::Char(c) if c.is_ascii_digit() => c.to_digit(10).map(|d| d as usize),
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
            Key::Char(c) => c.to_string(),
            Key::Enter => "Enter".to_string(),
            Key::Backspace => "Backspace".to_string(),
            Key::Esc => "Esc".to_string(),
            Key::Tab => "Tab".to_string(),
            // fallback for all other keys
            other => format!("{:?}", other),
        };

        parts.push(key_str);

        parts.join("+")
    }
}

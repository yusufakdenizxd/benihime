use egui::{Key, Modifiers};

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

    pub fn from_egui(modifier: Modifiers) -> Self {
        KeyModifiers {
            shift: modifier.shift,
            control: modifier.ctrl,
            alt: modifier.alt,
            super_key: modifier.command || modifier.mac_cmd,
        }
    }
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

impl KeyCode {
    pub fn from_egui(key: Key) -> Self {
        match key {
            Key::BrowserBack => KeyCode::Null,
            Key::Cut => KeyCode::Char('x'),
            Key::Copy => KeyCode::Char('c'),
            Key::Paste => KeyCode::Char('v'),

            Key::ArrowLeft => KeyCode::Left,
            Key::ArrowRight => KeyCode::Right,
            Key::ArrowUp => KeyCode::Up,
            Key::ArrowDown => KeyCode::Down,
            Key::Backspace => KeyCode::Backspace,
            Key::Enter => KeyCode::Enter,
            Key::Tab => KeyCode::Tab,
            Key::Escape => KeyCode::Esc,
            Key::Home => KeyCode::Home,
            Key::End => KeyCode::End,
            Key::PageUp => KeyCode::PageUp,
            Key::PageDown => KeyCode::PageDown,
            Key::Insert => KeyCode::Insert,
            Key::Delete => KeyCode::Delete,

            Key::F1 => KeyCode::F(1),
            Key::F2 => KeyCode::F(2),
            Key::F3 => KeyCode::F(3),
            Key::F4 => KeyCode::F(4),
            Key::F5 => KeyCode::F(5),
            Key::F6 => KeyCode::F(6),
            Key::F7 => KeyCode::F(7),
            Key::F8 => KeyCode::F(8),
            Key::F9 => KeyCode::F(9),
            Key::F10 => KeyCode::F(10),
            Key::F11 => KeyCode::F(11),
            Key::F12 => KeyCode::F(12),
            Key::F13 => KeyCode::F(13),
            Key::F14 => KeyCode::F(14),
            Key::F15 => KeyCode::F(15),
            Key::F16 => KeyCode::F(16),
            Key::F17 => KeyCode::F(17),
            Key::F18 => KeyCode::F(18),
            Key::F19 => KeyCode::F(19),
            Key::F20 => KeyCode::F(20),
            Key::F21 => KeyCode::F(21),
            Key::F22 => KeyCode::F(22),
            Key::F23 => KeyCode::F(23),
            Key::F24 => KeyCode::F(24),
            Key::F25 => KeyCode::F(25),
            Key::F26 => KeyCode::F(26),
            Key::F27 => KeyCode::F(27),
            Key::F28 => KeyCode::F(28),
            Key::F29 => KeyCode::F(29),
            Key::F30 => KeyCode::F(30),
            Key::F31 => KeyCode::F(31),
            Key::F32 => KeyCode::F(32),
            Key::F33 => KeyCode::F(33),
            Key::F34 => KeyCode::F(34),
            Key::F35 => KeyCode::F(35),

            Key::Space => KeyCode::Char(' '),
            Key::Minus => KeyCode::Char('-'),
            Key::Period => KeyCode::Char('.'),
            Key::Equals => KeyCode::Char('='),
            Key::Plus => KeyCode::Char('+'),
            Key::Semicolon => KeyCode::Char(';'),
            Key::Quote => KeyCode::Char('\''),
            Key::Comma => KeyCode::Char('.'),
            Key::Colon => KeyCode::Char(':'),
            Key::Pipe => KeyCode::Char('|'),
            Key::Questionmark => KeyCode::Char('?'),
            Key::Exclamationmark => KeyCode::Char('!'),
            Key::Backtick => KeyCode::Char('`'),
            Key::Slash => KeyCode::Char('/'),
            Key::Backslash => KeyCode::Char('\\'),
            Key::OpenCurlyBracket => KeyCode::Char('{'),
            Key::CloseCurlyBracket => KeyCode::Char('}'),
            Key::OpenBracket => KeyCode::Char('['),
            Key::CloseBracket => KeyCode::Char(']'),

            Key::A => KeyCode::Char('a'),
            Key::B => KeyCode::Char('b'),
            Key::C => KeyCode::Char('c'),
            Key::D => KeyCode::Char('d'),
            Key::E => KeyCode::Char('e'),
            Key::F => KeyCode::Char('f'),
            Key::G => KeyCode::Char('g'),
            Key::H => KeyCode::Char('h'),
            Key::I => KeyCode::Char('i'),
            Key::J => KeyCode::Char('j'),
            Key::K => KeyCode::Char('k'),
            Key::L => KeyCode::Char('l'),
            Key::M => KeyCode::Char('m'),
            Key::N => KeyCode::Char('n'),
            Key::O => KeyCode::Char('o'),
            Key::P => KeyCode::Char('p'),
            Key::Q => KeyCode::Char('q'),
            Key::R => KeyCode::Char('r'),
            Key::S => KeyCode::Char('s'),
            Key::T => KeyCode::Char('t'),
            Key::U => KeyCode::Char('u'),
            Key::V => KeyCode::Char('v'),
            Key::W => KeyCode::Char('w'),
            Key::X => KeyCode::Char('x'),
            Key::Y => KeyCode::Char('y'),
            Key::Z => KeyCode::Char('z'),

            Key::Num0 => KeyCode::Char('0'),
            Key::Num1 => KeyCode::Char('1'),
            Key::Num2 => KeyCode::Char('2'),
            Key::Num3 => KeyCode::Char('3'),
            Key::Num4 => KeyCode::Char('4'),
            Key::Num5 => KeyCode::Char('5'),
            Key::Num6 => KeyCode::Char('6'),
            Key::Num7 => KeyCode::Char('7'),
            Key::Num8 => KeyCode::Char('8'),
            Key::Num9 => KeyCode::Char('9'),
        }
    }
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
}

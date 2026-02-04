#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Key {
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
    Other,
}

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

#[derive(Debug, Clone)]
pub struct KeyPress {
    pub code: Key,
    pub pressed: bool,
    pub modifier: KeyModifiers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub position: (f32, f32),
    pub button: Option<MouseButton>,
    pub pressed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollDelta {
    Lines { x: f32, y: f32 },
    Pixels { x: f32, y: f32 },
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    Keyboard(KeyPress),
    Mouse(MouseEvent),
    Text(String),
    Scroll(ScrollDelta),
}

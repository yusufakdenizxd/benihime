use crate::key::KeyChord;

/// Mouse button identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button (scroll wheel click)
    Middle,
}

/// Mouse input event
#[derive(Debug, Clone)]
pub struct MouseEvent {
    /// Cursor position in window coordinates (x, y)
    pub position: (f32, f32),
    /// Button involved in the event (None for motion events)
    pub button: Option<MouseButton>,
    /// True if the button was pressed, false if released
    pub pressed: bool,
}

/// Mouse scroll delta reported by the windowing backend.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollDelta {
    /// Scroll expressed in logical lines.
    Lines { x: f32, y: f32 },
    /// Scroll expressed in physical pixels.
    Pixels { x: f32, y: f32 },
}

/// Input event types that can be handled by the application
#[derive(Debug, Clone)]
pub enum InputEvent {
    /// Keyboard key press or release
    Keyboard(KeyChord),
    /// Mouse button or motion event
    Mouse(MouseEvent),
    /// Text input (for typing)
    Text(String),
    /// Mouse wheel or trackpad scrolling
    Scroll(ScrollDelta),
}

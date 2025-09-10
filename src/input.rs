use crossterm::event::{KeyEvent, KeyModifiers};

use crate::{buffer::Mode, editor::Editor};

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ModifierKeyCode {
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    LeftHyper,
    LeftMeta,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    RightHyper,
    RightMeta,
    IsoLevel3Shift,
    IsoLevel5Shift,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Tab,
    BackTab,
    Delete,
    Insert,
    /// F key.
    F(u8),
    Char(char),
    Null,
    Esc,
    Modifier(ModifierKeyCode),
}

impl KeyCode {
    pub fn form_crossterm(k: crossterm::event::KeyCode) -> KeyCode {
        if k.is_esc() {
            return KeyCode::Esc;
        }

        if k.is_enter() {
            return KeyCode::Enter;
        }

        if k.is_backspace() {
            return KeyCode::Backspace;
        }

        if let Some(char) = k.as_char() {
            return KeyCode::Char(char);
        }
        return KeyCode::Null;
    }

    pub fn is_function_key(&self, n: u8) -> bool {
        matches!(self, KeyCode::F(m) if *m == n)
    }

    pub fn is_char(&self, c: char) -> bool {
        matches!(self, KeyCode::Char(m) if *m == c)
    }

    pub fn as_char(&self) -> Option<char> {
        match self {
            KeyCode::Char(c) => Some(*c),
            _ => None,
        }
    }

    pub fn is_modifier(&self, modifier: ModifierKeyCode) -> bool {
        matches!(self, KeyCode::Modifier(m) if *m == modifier)
    }
}

pub fn handle_normal(ed: &mut Editor, k: KeyEvent) {
    match KeyCode::form_crossterm(k.code) {
        KeyCode::Enter => ed.move_down(),
        KeyCode::Char('h') => ed.move_left(),
        KeyCode::Char('j') => ed.move_down(),
        KeyCode::Char('k') => ed.move_up(),
        KeyCode::Char('l') => ed.move_right(),
        KeyCode::Char('0') => ed.beginning_of_line(),
        KeyCode::Char('-') => ed.start_of_line(),
        KeyCode::Char('=') => ed.end_of_line(),
        KeyCode::Char('w') => ed.word_forward(),
        KeyCode::Char('e') => ed.word_end(),
        KeyCode::Char('b') => ed.word_backward(),
        KeyCode::Char('o') => {
            ed.new_line_below();
            ed.focused_buf.mode = Mode::Insert;
        }
        KeyCode::Char('O') => {
            ed.new_line_above();
            ed.focused_buf.mode = Mode::Insert;
        }
        KeyCode::Char('i') => {
            ed.focused_buf.mode = Mode::Insert;
        }
        KeyCode::Char('a') => {
            ed.move_right();
            ed.focused_buf.mode = Mode::Insert;
        }
        KeyCode::Char('v') => {
            ed.focused_buf.mode = Mode::Visual;
        } // placeholder
        _ => {}
    }
}

pub fn handle_insert(ed: &mut Editor, key: KeyEvent) {
    match KeyCode::form_crossterm(key.code) {
        KeyCode::Esc => ed.focused_buf.mode = Mode::Normal,
        KeyCode::Enter => {
            ed.new_line_below();
            ed.move_down();
        }
        KeyCode::Backspace => {
            if ed.focused_buf.cursor.col > 0 {
                ed.focused_buf.cursor.col -= 1;
                ed.focused_buf.lines[ed.focused_buf.cursor.row].remove(ed.focused_buf.cursor.col);
            } else if ed.focused_buf.cursor.row > 0 {
                let cursorrent = ed.focused_buf.lines.remove(ed.focused_buf.cursor.row);
                ed.focused_buf.cursor.row -= 1;
                let prev_len = ed.focused_buf.lines[ed.focused_buf.cursor.row].len();
                ed.focused_buf.lines[ed.focused_buf.cursor.row].push_str(&cursorrent);
                ed.focused_buf.cursor.col = prev_len;
            }
        }
        KeyCode::Char(c) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) { /* ignore ctrl chars in insert */
            } else {
                ed.insert_char(c);
            }
        }
        _ => {}
    }
}

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::editor::{Editor, Mode};

pub fn handle_normal(ed: &mut Editor, key: KeyEvent) {
    match key.code {
        KeyCode::Char('h') => ed.move_left(),
        KeyCode::Char('j') => ed.move_down(),
        KeyCode::Char('k') => ed.move_up(),
        KeyCode::Char('l') => ed.move_right(),
        KeyCode::Char('i') => {
            ed.mode = Mode::Insert;
        }
        KeyCode::Char('a') => {
            ed.move_right();
            ed.mode = Mode::Insert;
        }
        KeyCode::Char('v') => {
            ed.mode = Mode::Visual;
        } // placeholder
        _ => {}
    }
}

pub fn handle_insert(ed: &mut Editor, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => ed.mode = Mode::Normal,
        KeyCode::Char(c) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) { /* ignore ctrl chars in insert */
            } else {
                ed.insert_char(c);
            }
        }
        _ => {}
    }
}

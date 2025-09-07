use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::editor::{Editor, Mode};

pub fn handle_normal(ed: &mut Editor, key: KeyEvent) {
    match key.code {
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
            ed.mode = Mode::Insert;
        }
        KeyCode::Char('O') => {
            ed.new_line_above();
            ed.mode = Mode::Insert;
        }
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
        KeyCode::Backspace => {
            if ed.cursor.col > 0 {
                ed.cursor.col -= 1;
                ed.buf.lines[ed.cursor.row].remove(ed.cursor.col);
            } else if ed.cursor.row > 0 {
                let cursorrent = ed.buf.lines.remove(ed.cursor.row);
                ed.cursor.row -= 1;
                let prev_len = ed.buf.lines[ed.cursor.row].len();
                ed.buf.lines[ed.cursor.row].push_str(&cursorrent);
                ed.cursor.col = prev_len;
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

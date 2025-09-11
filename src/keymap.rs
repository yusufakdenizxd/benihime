use anyhow::{Ok, Result, anyhow};
use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{buffer::Mode, commands, editor::Editor};

pub type KeymapFn = fn(&mut Editor) -> Result<()>;

#[derive(Debug, Clone)]
pub struct Keymap {
    bindings: HashMap<KeyEvent, (&'static [Mode], KeymapFn)>,
}

impl Keymap {
    pub fn new(bindings: HashMap<KeyEvent, (&'static [Mode], KeymapFn)>) -> Self {
        Self { bindings }
    }

    pub fn default() -> Self {
        let mut bindings: HashMap<KeyEvent, (&'static [Mode], KeymapFn)> = HashMap::new();

        bindings.insert(
            KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
            (&[Mode::Normal, Mode::Visual], |ed| commands::move_left(ed)),
        );

        bindings.insert(
            KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
            (&[Mode::Normal, Mode::Visual], |ed| commands::move_down(ed)),
        );

        bindings.insert(
            KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
            (&[Mode::Normal, Mode::Visual], |ed| commands::move_up(ed)),
        );

        bindings.insert(
            KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
            (&[Mode::Normal, Mode::Visual], |ed| commands::move_right(ed)),
        );

        bindings.insert(
            KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE),
            (&[Mode::Normal, Mode::Visual], |ed| commands::word_end(ed)),
        );

        bindings.insert(
            KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE),
            (&[Mode::Normal, Mode::Visual], |ed| {
                commands::word_forward(ed)
            }),
        );

        bindings.insert(
            KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE),
            (&[Mode::Normal, Mode::Visual], |ed| {
                commands::word_backward(ed)
            }),
        );

        bindings.insert(
            KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE),
            (&[Mode::Normal, Mode::Visual], |ed| {
                commands::beginning_of_line(ed)
            }),
        );

        bindings.insert(
            KeyEvent::new(KeyCode::Char('-'), KeyModifiers::NONE),
            (&[Mode::Normal, Mode::Visual], |ed| {
                commands::start_of_line(ed)
            }),
        );

        bindings.insert(
            KeyEvent::new(KeyCode::Char('='), KeyModifiers::NONE),
            (&[Mode::Normal, Mode::Visual], |ed| {
                commands::end_of_line(ed)
            }),
        );

        bindings.insert(
            KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
            (&[Mode::Normal], |ed| commands::set_mode(ed, Mode::Insert)),
        );

        bindings.insert(
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
            (&[Mode::Normal], |ed| {
                commands::set_mode(ed, Mode::Insert)?;
                commands::move_right(ed)?;
                Ok(())
            }),
        );

        bindings.insert(
            KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE),
            (&[Mode::Normal], |ed| {
                commands::new_line_below(ed)?;
                commands::set_mode(ed, Mode::Insert)?;
                Ok(())
            }),
        );

        bindings.insert(
            KeyEvent::new(KeyCode::Char('O'), KeyModifiers::NONE),
            (&[Mode::Normal], |ed| {
                commands::new_line_above(ed)?;
                commands::set_mode(ed, Mode::Insert)?;
                Ok(())
            }),
        );

        bindings.insert(
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            (&[Mode::Insert], |ed| commands::set_mode(ed, Mode::Normal)),
        );

        Keymap::new(bindings)
    }

    pub fn execute(&mut self, editor: &mut Editor, event: KeyEvent) -> anyhow::Result<bool> {
        if let Some((modes, action)) = self.bindings.get(&event) {
            if modes.contains(&editor.focused_buf.mode) {
                let result = action(editor);
                if result.is_err() {
                    return Err(anyhow!("There was a error {}", result.err().unwrap()));
                }
                return Ok(true);
            }
        }
        Ok(false)
    }
}

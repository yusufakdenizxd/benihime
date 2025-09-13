use crate::{buffer::Mode, command::command::CommandArg};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use maplit::hashmap;

use super::keymap::Keymap;

pub fn register_default_keymap(km: &mut Keymap) {
    km.bind(
        &[Mode::Normal],
        KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
        "set-mode",
        Some(hashmap![
            "mode".to_string()=>CommandArg::Mode(Mode::Insert),
        ]),
    );

    km.bind(
        &[Mode::Insert, Mode::Command, Mode::Visual],
        KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
        "set-mode",
        Some(hashmap![
            "mode".to_string()=>CommandArg::Mode(Mode::Normal),
        ]),
    );

    km.bind(
        &[Mode::Normal],
        KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
        "move-left",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
        "move-down",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
        "move-up",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
        "move-right",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeyEvent::new(KeyCode::Char('-'), KeyModifiers::NONE),
        "first-non-blank",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE),
        "beginning-of-line",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeyEvent::new(KeyCode::Char('='), KeyModifiers::NONE),
        "end-of-line",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE),
        "open-below",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeyEvent::new(KeyCode::Char('O'), KeyModifiers::NONE),
        "open-above",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE),
        "word-end",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE),
        "word-start",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE),
        "word-forward",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE),
        "set-mode",
        Some(hashmap![
            "mode".to_string()=>CommandArg::Mode(Mode::Command),
        ]),
    );

    km.bind(
        &[Mode::Command],
        KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        "execute-command-buffer",
        None,
    );
}

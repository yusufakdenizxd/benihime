use crate::{buffer::Mode, command::command::CommandArg};

use super::{
    key_chord::{KeyChord, KeyCode, KeyModifiers},
    keymap::{KeySequence, Keymap},
};

pub fn register_default_keymap(km: &mut Keymap) {
    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('i'),
            modifiers: KeyModifiers::NONE,
        }),
        "set-mode",
        Some(vec![CommandArg::Mode(Mode::Insert)]),
    );

    km.bind(
        &[Mode::Insert, Mode::Command],
        KeySequence::single(KeyChord {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
        }),
        "set-mode",
        Some(vec![CommandArg::Mode(Mode::Normal)]),
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::NONE,
        }),
        "move-left",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::NONE,
        }),
        "move-down",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::NONE,
        }),
        "move-up",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('l'),
            modifiers: KeyModifiers::NONE,
        }),
        "move-right",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('-'),
            modifiers: KeyModifiers::NONE,
        }),
        "first-non-blank",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('0'),
            modifiers: KeyModifiers::NONE,
        }),
        "beginning-of-line",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('='),
            modifiers: KeyModifiers::NONE,
        }),
        "end-of-line",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('o'),
            modifiers: KeyModifiers::NONE,
        }),
        "open-below",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('o'),
            modifiers: KeyModifiers::SHIFT,
        }),
        "open-above",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('e'),
            modifiers: KeyModifiers::NONE,
        }),
        "word-end",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('b'),
            modifiers: KeyModifiers::NONE,
        }),
        "word-start",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('w'),
            modifiers: KeyModifiers::NONE,
        }),
        "word-forward",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char(':'),
            modifiers: KeyModifiers::SHIFT,
        }),
        "set-mode",
        Some(vec![CommandArg::Mode(Mode::Command)]),
    );

    km.bind(
        &[Mode::Command],
        KeySequence::single(KeyChord {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
        }),
        "execute-command-buffer",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::SHIFT,
        }),
        "next-buffer",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::SHIFT,
        }),
        "previous-buffer",
        None,
    );
}

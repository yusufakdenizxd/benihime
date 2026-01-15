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
        &[Mode::Insert, Mode::Command, Mode::Minibuffer],
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
            code: KeyCode::Char('x'),
            modifiers: KeyModifiers::SUPER,
        }),
        "set-mode",
        Some(vec![CommandArg::Mode(Mode::Minibuffer)]),
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::NONE,
        }),
        "move-left",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::NONE,
        }),
        "move-down",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::NONE,
        }),
        "move-up",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('l'),
            modifiers: KeyModifiers::NONE,
        }),
        "move-right",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('-'),
            modifiers: KeyModifiers::NONE,
        }),
        "first-non-blank",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('0'),
            modifiers: KeyModifiers::NONE,
        }),
        "beginning-of-line",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
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
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('e'),
            modifiers: KeyModifiers::NONE,
        }),
        "word-end",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('b'),
            modifiers: KeyModifiers::NONE,
        }),
        "word-backward",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
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

    km.bind(
        &[Mode::Normal],
        KeySequence::new(vec![
            KeyChord {
                code: KeyCode::Char('z'),
                modifiers: KeyModifiers::NONE,
            },
            KeyChord {
                code: KeyCode::Char('z'),
                modifiers: KeyModifiers::NONE,
            },
        ]),
        "center-cursor",
        None,
    );

    km.bind(
        &[Mode::Minibuffer],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::CTRL,
        }),
        "minibuffer-next-completion",
        None,
    );

    km.bind(
        &[Mode::Minibuffer],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::CTRL,
        }),
        "minibuffer-previous-completion",
        None,
    );

    km.bind(
        &[Mode::Minibuffer],
        KeySequence::single(KeyChord {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
        }),
        "minibuffer-next-completion",
        None,
    );

    km.bind(
        &[Mode::Minibuffer],
        KeySequence::single(KeyChord {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
        }),
        "minibuffer-previous-completion",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('p'),
            modifiers: KeyModifiers::SUPER,
        }),
        "find-file",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::new(vec![
            KeyChord {
                code: KeyCode::Char('x'),
                modifiers: KeyModifiers::CTRL,
            },
            KeyChord {
                code: KeyCode::Char('f'),
                modifiers: KeyModifiers::CTRL,
            },
        ]),
        "open-file",
        None,
    );

    km.bind(
        &[Mode::Minibuffer],
        KeySequence::single(KeyChord {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
        }),
        "minibuffer-accept",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('x'),
            modifiers: KeyModifiers::SUPER,
        }),
        "find-command",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CTRL,
        }),
        "kill-this-buffer",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('v'),
            modifiers: KeyModifiers::NONE,
        }),
        "enter-visual-mode",
        None,
    );

    km.bind(
        &[Mode::Visual],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('v'),
            modifiers: KeyModifiers::NONE,
        }),
        "exit-visual-mode",
        None,
    );

    km.bind(
        &[Mode::Visual],
        KeySequence::single(KeyChord {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
        }),
        "exit-visual-mode",
        None,
    );

    km.bind(
        &[Mode::Visual],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('o'),
            modifiers: KeyModifiers::NONE,
        }),
        "visual_select_other_end",
        None,
    );

    km.bind(
        &[Mode::Visual],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('d'),
            modifiers: KeyModifiers::NONE,
        }),
        "delete-selection",
        None,
    );

    km.bind(
        &[Mode::Visual],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::NONE,
        }),
        "change-selection",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('b'),
            modifiers: KeyModifiers::SUPER,
        }),
        "find-buffer",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('d'),
            modifiers: KeyModifiers::NONE,
        }),
        "delete-range",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::NONE,
        }),
        "change-range",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('u'),
            modifiers: KeyModifiers::NONE,
        }),
        "undo",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('r'),
            modifiers: KeyModifiers::CTRL,
        }),
        "redo",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('d'),
            modifiers: KeyModifiers::CTRL,
        }),
        "scroll-half-down",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('f'),
            modifiers: KeyModifiers::CTRL,
        }),
        "scroll-full-down",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('u'),
            modifiers: KeyModifiers::CTRL,
        }),
        "scroll-half-up",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('b'),
            modifiers: KeyModifiers::CTRL,
        }),
        "scroll-full-up",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::new(vec![
            KeyChord {
                code: KeyCode::Char('g'),
                modifiers: KeyModifiers::NONE,
            },
            KeyChord {
                code: KeyCode::Char('g'),
                modifiers: KeyModifiers::NONE,
            },
        ]),
        "goto-first-line",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('g'),
            modifiers: KeyModifiers::SHIFT,
        }),
        "goto-last-line",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::SUPER,
        }),
        "save-current-buffer",
        None,
    );
}

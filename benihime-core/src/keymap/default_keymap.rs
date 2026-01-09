use benihime_renderer::key::{KeyChord, KeyCode, KeyModifiers};

use crate::{buffer::Mode, command::command::CommandArg};

use super::keymap::{KeySequence, Keymap};

pub fn register_default_keymap(km: &mut Keymap) {
    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('i'))),
        "set-mode",
        Some(vec![CommandArg::Mode(Mode::Insert)]),
    );

    km.bind(
        &[Mode::Insert, Mode::Command, Mode::Minibuffer],
        KeySequence::single(KeyChord::code_input(KeyCode::Esc)),
        "set-mode",
        Some(vec![CommandArg::Mode(Mode::Normal)]),
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord::code_input_super(KeyCode::Char('x'))),
        "set-mode",
        Some(vec![CommandArg::Mode(Mode::Minibuffer)]),
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('h'))),
        "move-left",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('j'))),
        "move-down",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('k'))),
        "move-up",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('l'))),
        "move-right",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('-'))),
        "first-non-blank",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('0'))),
        "beginning-of-line",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('='))),
        "end-of-line",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('o'))),
        "open-below",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord::code_input_shift(KeyCode::Char('o'))),
        "open-above",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('e'))),
        "word-end",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('b'))),
        "word-backward",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('w'))),
        "word-forward",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord::code_input(KeyCode::Char(':'))),
        "set-mode",
        Some(vec![CommandArg::Mode(Mode::Command)]),
    );

    km.bind(
        &[Mode::Command],
        KeySequence::single(KeyChord::code_input(KeyCode::Enter)),
        "execute-command-buffer",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord::code_input_shift(KeyCode::Char('j'))),
        "next-buffer",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord::code_input_shift(KeyCode::Char('k'))),
        "previous-buffer",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::new(vec![
            KeyChord::code_input_shift(KeyCode::Char('z')),
            KeyChord::code_input_shift(KeyCode::Char('z')),
        ]),
        "center-cursor",
        None,
    );

    km.bind(
        &[Mode::Minibuffer],
        KeySequence::single(KeyChord::code_input_ctrl(KeyCode::Char('j'))),
        "minibuffer-next-completion",
        None,
    );

    km.bind(
        &[Mode::Minibuffer],
        KeySequence::single(KeyChord::code_input_ctrl(KeyCode::Char('k'))),
        "minibuffer-previous-completion",
        None,
    );

    km.bind(
        &[Mode::Minibuffer],
        KeySequence::single(KeyChord::code_input(KeyCode::Down)),
        "minibuffer-next-completion",
        None,
    );

    km.bind(
        &[Mode::Minibuffer],
        KeySequence::single(KeyChord::code_input(KeyCode::Up)),
        "minibuffer-previous-completion",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord::code_input_super(KeyCode::Char('p'))),
        "find-file",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::new(vec![
            KeyChord::code_input_ctrl(KeyCode::Char('x')),
            KeyChord::code_input_ctrl(KeyCode::Char('x')),
        ]),
        "open-file",
        None,
    );

    km.bind(
        &[Mode::Minibuffer],
        KeySequence::single(KeyChord::code_input(KeyCode::Enter)),
        "minibuffer-accept",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord::code_input_super(KeyCode::Char('x'))),
        "find-command",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord::code_input_ctrl(KeyCode::Char('q'))),
        "kill-this-buffer",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('v'))),
        "enter-visual-mode",
        None,
    );

    km.bind(
        &[Mode::Visual],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('v'))),
        "exit-visual-mode",
        None,
    );

    km.bind(
        &[Mode::Visual],
        KeySequence::single(KeyChord::code_input(KeyCode::Esc)),
        "exit-visual-mode",
        None,
    );

    km.bind(
        &[Mode::Visual],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('o'))),
        "visual_select_other_end",
        None,
    );

    km.bind(
        &[Mode::Visual],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('d'))),
        "delete-selection",
        None,
    );

    km.bind(
        &[Mode::Visual],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('c'))),
        "change-selection",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord::code_input_super(KeyCode::Char('b'))),
        "find-buffer",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('d'))),
        "delete-range",
        None,
    );

    km.bind(
        &[Mode::Normal],
        KeySequence::single(KeyChord::code_input(KeyCode::Char('c'))),
        "change-range",
        None,
    );
}

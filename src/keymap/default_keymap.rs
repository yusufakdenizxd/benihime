use egui::{Key, Modifiers};

use crate::{buffer::Mode, command::command::CommandArg};

use super::keymap::Keymap;

pub fn register_default_keymap(km: &mut Keymap) {
    km.bind(
        &[Mode::Normal],
        Key::I,
        Modifiers::default(),
        "set-mode",
        Some(vec![CommandArg::Mode(Mode::Insert)]),
    );

    km.bind(
        &[Mode::Insert, Mode::Command, Mode::Visual],
        Key::Escape,
        Modifiers::default(),
        "set-mode",
        Some(vec![CommandArg::Mode(Mode::Normal)]),
    );

    km.bind(
        &[Mode::Normal],
        Key::H,
        Modifiers::default(),
        "move-left",
        None,
    );

    km.bind(
        &[Mode::Normal],
        Key::J,
        Modifiers::default(),
        "move-down",
        None,
    );

    km.bind(
        &[Mode::Normal],
        Key::K,
        Modifiers::default(),
        "move-up",
        None,
    );

    km.bind(
        &[Mode::Normal],
        Key::L,
        Modifiers::default(),
        "move-right",
        None,
    );

    km.bind(
        &[Mode::Normal],
        Key::Minus,
        Modifiers::default(),
        "first-non-blank",
        None,
    );

    km.bind(
        &[Mode::Normal],
        Key::Num0,
        Modifiers::default(),
        "beginning-of-line",
        None,
    );

    km.bind(
        &[Mode::Normal],
        Key::Equals,
        Modifiers::default(),
        "end-of-line",
        None,
    );

    km.bind(
        &[Mode::Normal],
        Key::O,
        Modifiers::default(),
        "open-below",
        None,
    );

    km.bind(
        &[Mode::Normal],
        Key::O,
        Modifiers::SHIFT,
        "open-above",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        Key::E,
        Modifiers::default(),
        "word-end",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        Key::B,
        Modifiers::default(),
        "word-start",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        Key::W,
        Modifiers::default(),
        "word-forward",
        None,
    );

    km.bind(
        &[Mode::Normal, Mode::Visual],
        Key::Colon,
        Modifiers::default(),
        "set-mode",
        Some(vec![CommandArg::Mode(Mode::Command)]),
    );

    km.bind(
        &[Mode::Command],
        Key::Enter,
        Modifiers::default(),
        "execute-command-buffer",
        None,
    );

    km.bind(
        &[Mode::Normal],
        Key::J,
        Modifiers::SHIFT,
        "next-buffer",
        None,
    );

    km.bind(
        &[Mode::Normal],
        Key::K,
        Modifiers::SHIFT,
        "previous-buffer",
        None,
    );
}

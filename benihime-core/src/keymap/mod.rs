pub mod default_keymap;
pub mod key_chord;

use std::collections::HashMap;

use crate::{buffer::Mode, command::CommandArg, keymap::key_chord::KeyChord};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeySequence {
    pub chords: Vec<KeyChord>,
}

impl KeySequence {
    pub fn new(seq: Vec<KeyChord>) -> Self {
        KeySequence { chords: seq }
    }

    pub fn single(chord: KeyChord) -> Self {
        KeySequence {
            chords: vec![chord],
        }
    }

    pub fn default() -> Self {
        KeySequence { chords: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct Keymap {
    pub bindings: HashMap<(KeySequence, Mode), (String, Option<Vec<CommandArg>>)>,
    pub buffer: KeySequence,
}

impl Keymap {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            buffer: KeySequence::default(),
        }
    }
    pub fn bind(
        &mut self,
        modes: &[Mode],
        seq: KeySequence,
        command: &str,
        args: Option<Vec<CommandArg>>,
    ) {
        for mode in modes.iter() {
            self.bindings
                .insert((seq.clone(), *mode), (command.to_string(), args.clone()));
        }
    }

    pub fn push_key(
        &mut self,
        mode: Mode,
        chord: &KeyChord,
    ) -> Option<(String, Option<Vec<CommandArg>>)> {
        self.buffer.chords.push(chord.clone());

        let current_seq = KeySequence::new(self.buffer.chords.clone());

        if let Some(binding) = self.bindings.get(&(current_seq, mode)) {
            self.buffer.chords.clear();
            return Some(binding.clone());
        }

        let is_prefix = self
            .bindings
            .keys()
            .any(|seq| seq.0.chords.starts_with(&self.buffer.chords));

        if is_prefix {
        } else {
            self.buffer.chords.clear();
        }
        None
    }
}

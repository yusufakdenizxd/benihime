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

    pub fn to_string(&self) -> String {
        if self.chords.is_empty() {
            return "<empty>".to_string();
        }

        self.chords
            .iter()
            .map(|chord| chord.to_string())
            .collect::<Vec<_>>()
            .join(" ")
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

    pub fn render(&self) -> String {
        let mut lines: Vec<String> = Vec::new();

        let mut mode_map: HashMap<Mode, Vec<(&KeySequence, &String)>> = HashMap::new();

        for ((seq, mode), (command, _args)) in &self.bindings {
            mode_map.entry(*mode).or_default().push((seq, command));
        }

        let mut modes: Vec<Mode> = mode_map.keys().copied().collect();
        modes.sort_by_key(|m| *m as u8);

        for mode in modes {
            lines.push(format!("=== {:?} mode ===", mode));

            if let Some(bindings) = mode_map.get(&mode) {
                let mut sorted_bindings = bindings.clone();
                sorted_bindings.sort_by_key(|(seq, _)| seq.to_string());

                for (seq, command_name) in sorted_bindings {
                    lines.push(format!("{:<20} => {}", seq.to_string(), command_name));
                }
            }

            lines.push(String::new());
        }

        lines.join("\n")
    }
}

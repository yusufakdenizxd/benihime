use crate::buffer::Buffer;

#[derive(Clone, Debug)]
pub enum Edit {
    Insert { at: usize, text: String },
    Delete { at: usize, text: String },
}

#[derive(Debug, Clone)]
pub struct UndoTree {
    past: Vec<Edit>,
    future: Vec<Edit>,
}

impl UndoTree {
    pub fn new() -> Self {
        Self {
            past: Vec::new(),
            future: Vec::new(),
        }
    }

    pub fn record(&mut self, edit: Edit) {
        self.past.push(edit);
        self.future.clear();
    }

    pub fn undo(&mut self) -> Option<Edit> {
        let edit = self.past.pop()?;
        self.future.push(edit.clone());
        Some(edit)
    }

    pub fn redo(&mut self) -> Option<Edit> {
        let edit = self.future.pop()?;
        self.past.push(edit.clone());
        Some(edit)
    }
}

#[derive(Clone, Debug)]
pub enum Edit {
    Insert { at: usize, text: String },
    Delete { at: usize, text: String },
}

#[derive(Debug, Clone)]
pub enum UndoEntry {
    Single(Edit),
    Group(Vec<Edit>),
}

#[derive(Debug, Clone)]
pub struct UndoTree {
    past: Vec<UndoEntry>,
    future: Vec<UndoEntry>,
    current_group: Option<Vec<Edit>>,
}

impl UndoTree {
    pub fn new() -> Self {
        Self {
            past: Vec::new(),
            future: Vec::new(),
            current_group: None,
        }
    }

    pub fn begin_group(&mut self) {
        if self.current_group.is_none() {
            self.current_group = Some(Vec::new());
        }
    }

    pub fn end_group(&mut self) {
        if let Some(group) = self.current_group.take() {
            if !group.is_empty() {
                self.past.push(UndoEntry::Group(group));
                self.future.clear();
            }
        }
    }

    pub fn record(&mut self, edit: Edit) {
        if let Some(group) = self.current_group.as_mut() {
            group.push(edit);
        } else {
            self.past.push(UndoEntry::Single(edit));
            self.future.clear();
        }
    }

    pub fn undo(&mut self) -> Option<UndoEntry> {
        let entry = self.past.pop()?;
        self.future.push(entry.clone());
        Some(entry)
    }

    pub fn redo(&mut self) -> Option<UndoEntry> {
        let entry = self.future.pop()?;
        self.past.push(entry.clone());
        Some(entry)
    }
}

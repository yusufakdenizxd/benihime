use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

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

pub type NodeId = Rc<RefCell<UndoNode>>;

#[derive(Debug)]
pub struct UndoNode {
    pub entry: Option<UndoEntry>,
    pub parent: Option<Weak<RefCell<UndoNode>>>,
    pub children: Vec<NodeId>,
    pub active_child: usize,
}

#[derive(Debug, Clone)]
pub struct UndoTree {
    root: NodeId,
    current: NodeId,

    current_group: Vec<Edit>,
    grouping_enabled: bool,
}

impl UndoTree {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(UndoNode {
            entry: None,
            parent: None,
            children: Vec::new(),
            active_child: 0,
        }));

        Self {
            root: root.clone(),
            current: root,
            current_group: Vec::new(),
            grouping_enabled: true,
        }
    }
    fn push_node(&mut self, entry: UndoEntry) {
        let new_node = Rc::new(RefCell::new(UndoNode {
            entry: Some(entry),
            parent: Some(Rc::downgrade(&self.current)),
            children: Vec::new(),
            active_child: 0,
        }));

        let mut cur = self.current.borrow_mut();

        // Branching happens here
        cur.children.push(new_node.clone());
        cur.active_child = cur.children.len() - 1;

        drop(cur);

        self.current = new_node;
    }

    pub fn commit_group(&mut self) {
        if self.current_group.is_empty() {
            return;
        }

        let entry = if self.current_group.len() == 1 {
            UndoEntry::Single(self.current_group.remove(0))
        } else {
            UndoEntry::Group(std::mem::take(&mut self.current_group))
        };

        self.push_node(entry);
    }

    pub fn record(&mut self, edit: Edit) {
        if !self.grouping_enabled {
            self.push_node(UndoEntry::Single(edit));
            return;
        }

        self.current_group.push(edit);
    }

    pub fn undo(&mut self) -> Option<UndoEntry> {
        self.commit_group();

        let parent = {
            let cur = self.current.borrow();
            cur.parent.as_ref()?.upgrade()
        };

        let entry = self.current.borrow().entry.clone();
        self.current = parent.unwrap();
        entry
    }

    pub fn redo(&mut self) -> Option<UndoEntry> {
        let child = {
            let cur = self.current.borrow();
            if cur.children.is_empty() {
                return None;
            }
            cur.children[cur.active_child].clone()
        };

        let entry = child.borrow().entry.clone();
        self.current = child;
        entry
    }
}

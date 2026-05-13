use slotmap::SlotMap;

use crate::{
    graphics::Rect,
    window::{Window, WindowId},
};

#[derive(Debug)]
pub struct Tree {
    root: WindowId,
    pub focus: WindowId,
    area: Rect,

    nodes: SlotMap<WindowId, Node>,
}

#[derive(Debug)]
pub struct Node {
    parent: WindowId,
    content: Content,
}

#[derive(Debug)]
pub enum Content {
    Window(Box<Window>),
    Container(Box<Container>),
}

impl Node {
    pub fn container(layout: Layout) -> Self {
        Self {
            parent: WindowId::default(),
            content: Content::Container(Box::new(Container::new(layout))),
        }
    }

    pub fn window(window: Window) -> Self {
        Self {
            parent: WindowId::default(),
            content: Content::Window(Box::new(window)),
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.content {
            Content::Container(c) => c.children.is_empty(),
            Content::Window(_container) => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Container {
    layout: Layout,
    children: Vec<WindowId>,
    area: Rect,
}

impl Container {
    pub fn new(layout: Layout) -> Self {
        Self {
            layout,
            children: Vec::new(),
            area: Rect::default(),
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new(Layout::Vertical)
    }
}

impl Tree {
    pub fn new(area: Rect) -> Self {
        let root = Node::container(Layout::Vertical);

        let mut nodes = SlotMap::with_key();
        let root = nodes.insert(root);

        nodes[root].parent = root;

        Self {
            root,
            focus: root,
            area,
            nodes,
        }
    }

    pub fn get(&self, index: WindowId) -> &Window {
        self.try_get(index).unwrap()
    }

    pub fn try_get(&self, index: WindowId) -> Option<&Window> {
        match self.nodes.get(index) {
            Some(Node {
                content: Content::Window(window),
                ..
            }) => Some(window),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, index: WindowId) -> &mut Window {
        match &mut self.nodes[index] {
            Node {
                content: Content::Window(view),
                ..
            } => view,
            _ => unreachable!(),
        }
    }

    pub fn is_empty(&self) -> bool {
        *&self.nodes[self.root].is_empty()
    }

    pub fn contains(&self, index: WindowId) -> bool {
        self.nodes.contains_key(index)
    }

    pub fn area(&self) -> Rect {
        self.area
    }

    pub fn insert(&mut self, window: Window) -> WindowId {
        let node = Node::window(window);
        let id = self.nodes.insert(node);
        if let Content::Container(c) = &mut self.nodes[self.root].content {
            c.children.push(id);
        }
        id
    }
}

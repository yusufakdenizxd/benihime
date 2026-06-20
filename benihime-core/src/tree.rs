use slotmap::SlotMap;

use crate::{
    graphics::Rect,
    window::{Window, WindowId},
};

#[derive(Debug, Clone)]
pub struct Tree {
    root: WindowId,
    pub focus: WindowId,
    area: Rect,

    nodes: SlotMap<WindowId, Node>,
}

#[derive(Debug, Clone)]
pub struct Node {
    parent: WindowId,
    content: Content,
    area: Rect,
}

#[derive(Debug, Clone)]
pub enum Content {
    Window(Box<Window>),
    Container(Box<Container>),
}

impl Node {
    pub fn container(layout: Layout) -> Self {
        Self {
            parent: WindowId::default(),
            content: Content::Container(Box::new(Container::new(layout))),
            area: Rect::default(),
        }
    }

    pub fn window(window: Window) -> Self {
        Self {
            parent: WindowId::default(),
            content: Content::Window(Box::new(window)),
            area: Rect::default(),
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

#[derive(Debug, Clone)]
pub struct Container {
    layout: Layout,
    children: Vec<WindowId>,
}

impl Container {
    pub fn new(layout: Layout) -> Self {
        Self {
            layout,
            children: Vec::new(),
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

    pub fn resize(&mut self, area: Rect) -> bool {
        if self.area != area {
            self.area = area;
            self.recalculate();
            return true;
        }
        false
    }

    pub fn insert(&mut self, window: Window) -> WindowId {
        let focus = self.focus;
        let parent = self.nodes[focus].parent;
        let mut node = Node::window(window);
        node.parent = parent;
        let node = self.nodes.insert(node);
        self.get_mut(node).id = node;

        let container = match &mut self.nodes[parent] {
            Node {
                content: Content::Container(container),
                ..
            } => container,
            _ => unreachable!(),
        };

        let pos = if container.children.is_empty() {
            0
        } else {
            let pos = container
                .children
                .iter()
                .position(|&child| child == focus)
                .unwrap();
            pos + 1
        };

        container.children.insert(pos, node);
        self.focus = node;

        self.recalculate();

        node
    }

    pub fn set_single_window(&mut self, window: Window) -> WindowId {
        let area = self.area;
        let mut nodes = SlotMap::with_key();

        let root = nodes.insert(Node::container(Layout::Vertical));
        nodes[root].parent = root;

        let mut node = Node::window(window);
        node.parent = root;
        node.area = area;
        let window_id = nodes.insert(node);

        match &mut nodes[root].content {
            Content::Container(container) => container.children.push(window_id),
            Content::Window(_) => unreachable!(),
        }

        match &mut nodes[window_id].content {
            Content::Window(window) => window.id = window_id,
            Content::Container(_) => unreachable!(),
        }

        self.root = root;
        self.focus = window_id;
        self.nodes = nodes;
        self.recalculate();

        window_id
    }

    pub fn split(&mut self, window: Window, layout: Layout) -> WindowId {
        let focus = self.focus;
        let parent = self.nodes[focus].parent;

        let node = Node::window(window);
        let node = self.nodes.insert(node);
        self.get_mut(node).id = node;

        let container = match &mut self.nodes[parent] {
            Node {
                content: Content::Container(container),
                ..
            } => container,
            _ => unreachable!(),
        };
        if container.layout == layout {
            let pos = if container.children.is_empty() {
                0
            } else {
                let pos = container
                    .children
                    .iter()
                    .position(|&child| child == focus)
                    .unwrap();
                pos + 1
            };
            container.children.insert(pos, node);
            self.nodes[node].parent = parent;
        } else {
            let mut split = Node::container(layout);
            split.parent = parent;
            let split = self.nodes.insert(split);

            let container = match &mut self.nodes[split] {
                Node {
                    content: Content::Container(container),
                    ..
                } => container,
                _ => unreachable!(),
            };
            container.children.push(focus);
            container.children.push(node);
            self.nodes[focus].parent = split;
            self.nodes[node].parent = split;

            let container = match &mut self.nodes[parent] {
                Node {
                    content: Content::Container(container),
                    ..
                } => container,
                _ => unreachable!(),
            };

            let pos = container
                .children
                .iter()
                .position(|&child| child == focus)
                .unwrap();

            container.children[pos] = split;
        }

        self.focus = node;

        self.recalculate();

        node
    }

    pub fn area_of(&self, id: WindowId) -> Option<Rect> {
        self.nodes.get(id).map(|node| node.area)
    }

    pub fn windows(&self) -> impl Iterator<Item = (&Window, Rect, bool)> {
        let focus = self.focus;
        self.nodes.iter().filter_map(move |(key, node)| match node {
            Node {
                content: Content::Window(view),
                ..
            } => Some((view.as_ref(), node.area, focus == key)),
            _ => None,
        })
    }

    pub fn focus_move(&mut self, direction: Direction) {
        let mut current = self.focus;
        let new_focus = loop {
            let parent = match self.nodes.get(current) {
                Some(node) if node.parent != current => node.parent,
                _ => return,
            };

            let (layout, children) = match &self.nodes[parent].content {
                Content::Container(c) => (c.layout, c.children.clone()),
                _ => return,
            };

            let moves_in_layout = match (layout, direction) {
                (Layout::Vertical, Direction::Left | Direction::Right) => true,
                (Layout::Horizontal, Direction::Up | Direction::Down) => true,
                _ => false,
            };

            if moves_in_layout {
                let pos = match children.iter().position(|&id| id == current) {
                    Some(p) => p,
                    None => return,
                };

                let new_pos = match direction {
                    Direction::Left | Direction::Up => pos.checked_sub(1),
                    Direction::Right | Direction::Down => {
                        if pos + 1 < children.len() {
                            Some(pos + 1)
                        } else {
                            None
                        }
                    }
                };

                if let Some(new_pos) = new_pos {
                    break self.find_leaf(children[new_pos], direction);
                }
            }

            current = parent;
        };

        self.focus = new_focus;
    }

    fn find_leaf(&self, node_id: WindowId, direction: Direction) -> WindowId {
        match &self.nodes[node_id].content {
            Content::Window(_) => node_id,
            Content::Container(c) => {
                if c.children.is_empty() {
                    return node_id;
                }
                let prefer_first =
                    matches!(direction, Direction::Right | Direction::Down);
                let idx = if prefer_first {
                    0
                } else {
                    c.children.len() - 1
                };
                self.find_leaf(c.children[idx], direction)
            }
        }
    }

    pub fn recalculate(&mut self) {
        let root = self.root;
        let area = self.area;

        self.recalculate_node(root, area);
    }

    fn recalculate_node(&mut self, node_id: WindowId, area: Rect) {
        let node = match self.nodes.get_mut(node_id) {
            Some(node) => node,
            None => return,
        };

        node.area = area;

        let children_to_visit = {
            let node = match self.nodes.get_mut(node_id) {
                Some(node) => node,
                None => return,
            };

            node.area = area;

            match &mut node.content {
                Content::Window(_window) => None,

                Content::Container(container) => {
                    Some((container.layout, container.children.clone()))
                }
            }
        };

        let Some((direction, children)) = children_to_visit else {
            return;
        };

        if children.is_empty() {
            return;
        }

        let count = children.len() as u16;

        match direction {
            Layout::Vertical => {
                let base_width = area.width / count;
                let remainder = area.width % count;

                let mut current_x = area.x;

                for (i, child_id) in children.iter().enumerate() {
                    let extra = if i == children.len() - 1 {
                        remainder
                    } else {
                        0
                    };

                    let child_rect = Rect {
                        x: current_x,
                        y: area.y,
                        width: base_width + extra,
                        height: area.height,
                    };

                    current_x += child_rect.width;

                    self.recalculate_node(*child_id, child_rect);
                }
            }

            Layout::Horizontal => {
                let base_height = area.height / count;
                let remainder = area.height % count;

                let mut current_y = area.y;

                for (i, child_id) in children.iter().enumerate() {
                    let extra = if i == children.len() - 1 {
                        remainder
                    } else {
                        0
                    };

                    let child_rect = Rect {
                        x: area.x,
                        y: current_y,
                        width: area.width,
                        height: base_height + extra,
                    };

                    current_y += child_rect.height;

                    self.recalculate_node(*child_id, child_rect);
                }
            }
        }
    }
}

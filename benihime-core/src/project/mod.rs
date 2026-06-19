// project.rs
use std::{collections::BTreeMap, path::PathBuf};

use crate::{buffer::BufferId, tree::Tree, window::Window};

pub mod project_manager;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectId(u64);

#[derive(Debug, Clone)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub root: Option<PathBuf>,
    pub buffers: Vec<BufferId>,
    pub tree: Tree,
    pub windows: BTreeMap<BufferId, Window>,
}

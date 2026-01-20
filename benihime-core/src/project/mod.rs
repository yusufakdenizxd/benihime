// project.rs
use std::path::PathBuf;

use crate::buffer::BufferId;

pub mod project_manager;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectId(u64);

#[derive(Debug, Clone)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub root: Option<PathBuf>,
    pub buffers: Vec<BufferId>,
}

// project.rs
use std::path::PathBuf;

pub mod project_manager;

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub root: PathBuf,
}

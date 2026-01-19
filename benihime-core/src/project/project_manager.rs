use std::{collections::HashMap, path::PathBuf};

use crate::project::Project;

pub struct ProjectManager {
    projects: HashMap<String, Project>,
    current: Option<Project>,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
            current: None,
        }
    }

    pub fn add(&mut self, project: Project) {
        self.projects.insert(project.name.clone(), project);
    }

    pub fn current(&self) -> Option<&Project> {
        self.current.as_ref()
    }

    pub fn current_name(&self) -> Option<String> {
        self.current.as_ref().map(|p| p.name.clone())
    }

    pub fn discover_in_path(&mut self, path: &PathBuf) {
        let entries = match std::fs::read_dir(path) {
            Ok(entries) => entries,
            Err(_) => return,
        };

        for entry in entries.flatten() {
            let entry_path = entry.path();

            if !entry_path.is_dir() {
                continue;
            }

            let name = match entry_path.file_name().and_then(|n| n.to_str()) {
                Some(s) => s.to_string(),
                None => continue,
            };

            let project = Project {
                name: name.to_string(),
                root: entry_path,
            };

            self.projects.insert(name.to_string(), project);
        }
    }

    pub fn switch(&mut self, project: Project) -> Option<&Project> {
        self.current = Some(project);
        self.current()
    }

    pub fn get_projects(&self) -> Vec<Project> {
        self.projects.values().cloned().collect()
    }
}

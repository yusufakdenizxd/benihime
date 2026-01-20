use std::{collections::HashMap, path::PathBuf};

use crate::{
    buffer::BufferId,
    project::{Project, ProjectId},
};

pub const DEFAULT_PROJECT_ID: ProjectId = ProjectId(0);

pub struct ProjectManager {
    projects: HashMap<ProjectId, Project>,
    name_index: HashMap<String, ProjectId>,
    current: ProjectId,
    next_id: u64,
}

impl ProjectManager {
    pub fn new() -> Self {
        let mut projects = HashMap::new();
        let mut name_index = HashMap::new();

        let default_project = Project {
            id: DEFAULT_PROJECT_ID,
            name: "empty".to_string(),
            root: PathBuf::new(),
            buffers: Vec::new(),
        };

        projects.insert(DEFAULT_PROJECT_ID, default_project);
        name_index.insert("empty".to_string(), DEFAULT_PROJECT_ID);

        Self {
            projects,
            name_index,
            current: DEFAULT_PROJECT_ID,
            next_id: 1,
        }
    }

    pub fn add(&mut self, name: String, root: PathBuf) -> ProjectId {
        let id = ProjectId(self.next_id);
        self.next_id += 1;

        let project = Project {
            id,
            name: name.clone(),
            root,
            buffers: Vec::new(),
        };

        self.projects.insert(id, project);
        self.name_index.insert(name, id);

        id
    }

    pub fn current(&self) -> &Project {
        self.projects
            .get(&self.current)
            .expect("current project id must always exist")
    }

    pub fn current_mut(&mut self) -> &mut Project {
        self.projects
            .get_mut(&self.current)
            .expect("current project id must always exist")
    }

    pub fn current_name(&self) -> String {
        self.current().name.clone()
    }

    pub fn current_id(&self) -> ProjectId {
        self.current
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

            if self.name_index.contains_key(&name) {
                continue;
            }

            self.add(name, entry_path);
        }
    }

    pub fn switch_by_id(&mut self, id: ProjectId) -> Option<&Project> {
        if self.projects.contains_key(&id) {
            self.current = id;
            self.projects.get(&id)
        } else {
            None
        }
    }

    pub fn switch_by_name(&mut self, name: &str) -> Option<&Project> {
        let id = *self.name_index.get(name)?;
        self.switch_by_id(id)
    }

    pub fn get_projects(&self) -> Vec<&Project> {
        self.projects.values().collect()
    }

    pub fn get_projects_cloned(&self) -> Vec<Project> {
        self.projects.values().cloned().collect()
    }

    pub fn add_buffer_to_current(&mut self, id: BufferId) {
        self.current_mut().buffers.push(id);
    }
}

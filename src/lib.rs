//! SigmaOS-appcode: JetBrains AppCode IDE model.
//!
//! Real implementation: projects, workspaces, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub language: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    pub id: String,
    pub project_id: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppcodeError {
    EmptyProjectName,
    DuplicateProject(String),
}

impl fmt::Display for AppcodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyProjectName => write!(f, "project name must not be empty"),
            Self::DuplicateProject(id) => write!(f, "duplicate project {:?}", id),
        }
    }
}

impl std::error::Error for AppcodeError {}

pub struct Appcode {
    pub name: String,
    projects: BTreeMap<String, Project>,
    workspaces: BTreeMap<String, Workspace>,
}

impl Appcode {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), projects: BTreeMap::new(), workspaces: BTreeMap::new() }
    }

    pub fn add_project(&mut self, p: Project) -> Result<(), AppcodeError> {
        if p.name.trim().is_empty() { return Err(AppcodeError::EmptyProjectName); }
        let id = p.id.clone();
        if self.projects.contains_key(&id) {
            return Err(AppcodeError::DuplicateProject(id.clone()));
        }
        self.projects.insert(id, p);
        Ok(())
    }

    pub fn project_count(&self) -> usize { self.projects.len() }

    pub fn get_project(&self, id: &str) -> Option<&Project> { self.projects.get(id) }

    pub fn add_workspace(&mut self, w: Workspace) {
        self.workspaces.insert(w.id.clone(), w);
    }

    pub fn get_workspace(&self, id: &str) -> Option<&Workspace> { self.workspaces.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = Appcode::new("Appcode");
        assert_eq!(a.name, "Appcode");
        assert_eq!(a.project_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = Appcode::new("Appcode");
        let project = Project { id: "p1".into(), name: "PythonApp".into(), language: "Python".into() };
        a.add_project(project).unwrap();
        assert_eq!(a.project_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = Appcode::new("Appcode");
        let project = Project { id: "x".into(), name: "".into(), language: "x".into() };
        assert!(a.add_project(project).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = Appcode::new("Appcode");
        let project = Project { id: "x".into(), name: "X".into(), language: "x".into() };
        a.add_project(project).unwrap();
        let project2 = Project { id: "x".into(), name: "X".into(), language: "x".into() };
        assert!(a.add_project(project2).is_err());
    }
}

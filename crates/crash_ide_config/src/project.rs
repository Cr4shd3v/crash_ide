use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use crash_ide_project::EditorProject;
use crate::load::{default_load_config, EditorConfig};

/// Resource containing all [EditorProject]s
#[derive(Serialize, Deserialize, Resource, Default)]
pub struct EditorConfigProjects {
    /// All saved [EditorProject]s, indexed by their path
    pub projects: HashMap<String, EditorProject>,
}

impl EditorConfig for EditorConfigProjects {
    const FILENAME: &'static str = "projects.json";
}

default_load_config!(load_projects_config, EditorConfigProjects, projects);
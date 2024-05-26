use std::fs;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::load::{default_load_config, EditorConfig};

/// Saved project for the editor
#[derive(Serialize, Deserialize)]
pub struct EditorProject {
    /// Name of the project.
    ///
    /// On creation, this defaults to the folder name of the project.
    pub name: String,
    /// Actual path to the project
    pub path: String,
}

/// Resource containing all [EditorProject]s
#[derive(Serialize, Deserialize, Resource, Default)]
pub struct EditorConfigProjects {
    /// All saved [EditorProject]s
    pub projects: Vec<EditorProject>,
}

impl EditorConfig for EditorConfigProjects {
    const FILENAME: &'static str = "projects.json";
}

default_load_config!(load_projects_config, EditorConfigProjects, projects);
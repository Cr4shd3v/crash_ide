use std::fs;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::load::{default_load_config, EditorConfig};

#[derive(Serialize, Deserialize)]
pub struct EditorProject {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Resource, Default)]
pub struct EditorConfigProjects {
    pub projects: Vec<EditorProject>,
}

impl EditorConfig for EditorConfigProjects {
    const FILENAME: &'static str = "projects.json";
}

default_load_config!(load_projects_config, EditorConfigProjects, projects);
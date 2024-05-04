use std::fs;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::HomeDir;
use crate::load::default_load_config;

#[derive(Serialize, Deserialize)]
pub struct EditorProject {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Resource, Default)]
pub struct EditorConfigProjects {
    pub projects: Vec<EditorProject>,
}

default_load_config!(load_projects_config, "projects.json", EditorConfigProjects);
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Saved project for the editor
#[derive(Serialize, Deserialize, Clone)]
pub struct EditorProject {
    /// Name of the project.
    ///
    /// On creation, this defaults to the folder name of the project.
    pub name: String,
    /// Actual path to the project
    pub path: String,
}

/// Loaded project for the editor.
///
/// Is spawned as entity and referenced to by [ProjectRef]
#[derive(Component)]
pub struct LoadedEditorProject {
    /// See [EditorProject]
    pub crash_ide_project: EditorProject,
}
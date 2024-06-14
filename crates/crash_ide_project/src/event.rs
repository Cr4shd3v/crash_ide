use bevy::prelude::*;
use crate::EditorProject;

/// Event fired when opening a project
#[derive(Event)]
pub struct OpenProjectEvent {
    pub crash_ide_project: EditorProject,
    pub base_window: Option<Entity>,
}

impl OpenProjectEvent {
    /// Opens the provided project.
    ///
    /// When `base_window` is None, a new window is created.
    pub fn new(crash_ide_project: EditorProject, base_window: Option<Entity>) -> Self {
        Self {
            crash_ide_project,
            base_window,
        }
    }
}
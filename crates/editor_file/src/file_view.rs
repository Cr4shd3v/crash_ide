use std::path::PathBuf;
use bevy::prelude::*;

/// Component that is spawned for every view
#[derive(Component)]
pub struct FileViewInstance {
    /// Path to the file that is opened in this view
    pub path: PathBuf,
}
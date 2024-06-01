mod default_file_view;
mod file_view;

use bevy::prelude::*;
use crate::editor::editor_file_view::default_file_view::spawn_default_file_view;
use crate::editor::editor_file_view::file_view::{save_edited_content, spawn_file_view};

pub struct EditorFileViewPlugin;

impl Plugin for EditorFileViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_default_file_view, spawn_file_view, save_edited_content))
        ;
    }
}
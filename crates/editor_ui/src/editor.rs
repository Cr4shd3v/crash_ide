mod main_editor_screen;

use bevy::prelude::*;
use crate::editor::main_editor_screen::spawn_main_editor_screen;

pub(super) struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_main_editor_screen)
        ;
    }
}
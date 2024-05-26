mod main_editor_screen;

use bevy::prelude::*;
use editor_state::EditorState;
use crate::editor::main_editor_screen::spawn_main_editor_screen;

pub(super) struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(EditorState::Project), spawn_main_editor_screen)
        ;
    }
}
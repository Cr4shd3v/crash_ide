mod main_editor_screen;
mod editor_left_menu;

use bevy::prelude::*;
use crate::editor::main_editor_screen::MainEditorScreenPlugin;

pub(super) struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MainEditorScreenPlugin)
        ;
    }
}
mod main_editor_screen;
mod editor_left_menu;
mod editor_file_view;

use bevy::prelude::*;
use crate::editor::editor_file_view::EditorFileViewPlugin;
use crate::editor::editor_left_menu::EditorLeftMenuPlugin;
use crate::editor::main_editor_screen::MainEditorScreenPlugin;

pub(super) struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((MainEditorScreenPlugin, EditorLeftMenuPlugin, EditorFileViewPlugin))
        ;
    }
}
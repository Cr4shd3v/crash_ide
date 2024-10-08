//! This module contains all ui implementations for the main editor.

mod main_editor_screen;
mod editor_left_menu;
mod editor_file_view;
mod editor_top_menu;
mod editor_bottom_menu;

use bevy::prelude::*;
use crate::editor::editor_bottom_menu::EditorBottomMenuPlugin;
use crate::editor::editor_file_view::EditorFileViewPlugin;
use crate::editor::editor_left_menu::EditorLeftMenuPlugin;
use crate::editor::editor_top_menu::EditorTopMenuPlugin;
pub use crate::editor::main_editor_screen::*;

pub(super) struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                MainEditorScreenPlugin, EditorLeftMenuPlugin,
                EditorFileViewPlugin, EditorTopMenuPlugin,
                EditorBottomMenuPlugin,
            ))
        ;
    }
}
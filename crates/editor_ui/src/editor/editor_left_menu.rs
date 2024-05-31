mod filesystem_menu;

use bevy::prelude::*;
use crate::editor::editor_left_menu::filesystem_menu::FilesystemMenuPlugin;

pub struct EditorLeftMenuPlugin;

impl Plugin for EditorLeftMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FilesystemMenuPlugin)
        ;
    }
}
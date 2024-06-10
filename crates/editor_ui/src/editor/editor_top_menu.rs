mod top_menu;
mod file_menu;

use bevy::prelude::*;
use editor_widget::ExpandableMenuExtension;
use crate::editor::editor_top_menu::file_menu::{FileMenu, spawn_file_menu};
use crate::editor::editor_top_menu::top_menu::spawn_top_menu;

pub(super) struct EditorTopMenuPlugin;

impl Plugin for EditorTopMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_top_menu, spawn_file_menu))
            .register_expandable_menu::<FileMenu>()
        ;
    }
}
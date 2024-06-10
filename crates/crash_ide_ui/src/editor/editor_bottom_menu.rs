mod console_menu;

use bevy::prelude::*;
use crate::editor::editor_bottom_menu::console_menu::ConsoleMenuPlugin;

pub(super) struct EditorBottomMenuPlugin;

impl Plugin for EditorBottomMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ConsoleMenuPlugin)
        ;
    }
}


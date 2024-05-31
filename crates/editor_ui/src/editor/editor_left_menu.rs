mod filesystem_menu;

use bevy::prelude::*;
pub use filesystem_menu::*;

pub struct EditorLeftMenuPlugin;

impl Plugin for EditorLeftMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FilesystemMenuPlugin)
        ;
    }
}
mod top_menu;

use bevy::prelude::*;
use crate::editor::editor_top_menu::top_menu::spawn_top_menu;

pub struct EditorTopMenuPlugin;

impl Plugin for EditorTopMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_top_menu)
        ;
    }
}
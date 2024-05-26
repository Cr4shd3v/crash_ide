//! This module contains all screens.

use bevy::prelude::*;
pub use project_create_screen::*;

mod project_create_screen;

pub(super) struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_project_create_screen)
        ;
    }
}
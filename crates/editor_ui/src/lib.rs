mod root;
mod startup;

use bevy::prelude::*;
pub use root::*;
use crate::startup::StartupScreenPlugin;

pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, setup_ui)
            .add_plugins(StartupScreenPlugin)
        ;
    }
}
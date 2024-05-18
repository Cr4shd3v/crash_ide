mod root;
mod startup;
mod widget;
mod button;
mod file_dialog;

use bevy::prelude::*;
use bevy_file_dialog::FileDialogPlugin;
pub use root::*;
use crate::button::button_cursor;
use crate::file_dialog::OpenProjectDialog;
use crate::startup::StartupScreenPlugin;
use crate::widget::WidgetPlugin;

pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, setup_ui)
            .add_systems(Update, button_cursor)
            .add_plugins((StartupScreenPlugin, WidgetPlugin))
            .add_plugins(
                FileDialogPlugin::new()
                    .with_pick_directory::<OpenProjectDialog>(),
            )
        ;
    }
}
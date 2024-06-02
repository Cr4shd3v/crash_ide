//! UI crate of editor

#![warn(missing_docs)]

mod startup;
pub mod widget;
mod button;
mod window;
mod editor;
mod icons;
mod open_project;

use bevy::prelude::*;
use bevy_file_dialog::FileDialogPlugin;
use crate::button::button_cursor;
use crate::editor::EditorPlugin;
use crate::icons::IconPlugin;
use crate::open_project::OpenProjectPlugin;
use crate::startup::StartupScreenPlugin;
use crate::widget::button::OpenProjectDialog;
use crate::widget::EditorWidgetPlugin;
use crate::window::EditorWindowPlugin;

/// Plugin implementing UI for the editor
pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, button_cursor)
            .add_plugins((
                StartupScreenPlugin, EditorWidgetPlugin, EditorPlugin,
                EditorWindowPlugin, IconPlugin, OpenProjectPlugin,
            ))
            .add_plugins(
                FileDialogPlugin::new()
                    .with_pick_directory::<OpenProjectDialog>(),
            )
        ;
    }
}

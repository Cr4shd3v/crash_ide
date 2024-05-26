//! UI crate of editor

#![warn(missing_docs)]

mod startup;
pub mod widget;
mod button;
mod window;
mod editor;

use bevy::prelude::*;
use bevy_file_dialog::FileDialogPlugin;
use crate::button::button_cursor;
use crate::editor::EditorPlugin;
use crate::startup::StartupScreenPlugin;
use crate::widget::button::OpenProjectDialog;
use crate::widget::WidgetPlugin;
use crate::window::{check_for_exit, initial_window, process_new_window, update_active_window};

/// Plugin implementing UI for the editor
pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, initial_window)
            .add_systems(PreUpdate, (update_active_window, process_new_window))
            .add_systems(Update, button_cursor)
            .add_systems(PostUpdate, check_for_exit)
            .add_plugins((StartupScreenPlugin, WidgetPlugin, EditorPlugin))
            .add_plugins(
                FileDialogPlugin::new()
                    .with_pick_directory::<OpenProjectDialog>(),
            )
        ;
    }
}
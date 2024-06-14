//! UI crate of editor

#![warn(missing_docs)]

mod startup;
mod widget;
mod button;
mod window;
pub mod editor;
mod open_project;
mod switch_project;

use bevy::prelude::*;
use open_project::OpenProjectPlugin;
pub use switch_project::*;
use crate::button::button_cursor;
use crate::editor::EditorPlugin;
use crate::startup::StartupScreenPlugin;
use crate::widget::EditorWidgetPlugin;
use crate::window::EditorWindowPlugin;

/// Plugin implementing UI for the editor
pub struct CrashIDEUiPlugin;

impl Plugin for CrashIDEUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, button_cursor)
            .add_plugins((
                StartupScreenPlugin, EditorWidgetPlugin, EditorPlugin,
                EditorWindowPlugin, OpenProjectPlugin, SwitchProjectPlugin,
            ))
        ;
    }
}

//! UI crate of editor

#![warn(missing_docs)]

mod startup;
mod widget;
mod button;
mod window;
pub mod editor;
mod open_project;
mod switch_project;
pub mod settings;
mod checkbox;
mod text_scale;
mod trigger;

use bevy::prelude::*;
use crash_ide_state::EditorState;
use open_project::OpenProjectPlugin;
pub use switch_project::*;
use crate::button::button_cursor;
use crate::checkbox::init_checkbox;
use crate::editor::EditorPlugin;
use crate::settings::SettingsPlugin;
use crate::startup::StartupScreenPlugin;
use crate::text_scale::{scale_text, scale_text_input};
use crate::trigger::TriggerPlugin;
use crate::widget::EditorWidgetPlugin;
use crate::window::EditorWindowPlugin;

/// Plugin implementing UI for the editor
pub struct CrashIDEUiPlugin;

impl Plugin for CrashIDEUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(EditorState::Loaded), init_checkbox)
            .add_systems(Update, (button_cursor, scale_text, scale_text_input))
            .add_plugins((
                StartupScreenPlugin, EditorWidgetPlugin, EditorPlugin,
                EditorWindowPlugin, OpenProjectPlugin, SwitchProjectPlugin,
                SettingsPlugin, TriggerPlugin,
            ))
        ;
    }
}

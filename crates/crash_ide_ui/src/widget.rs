//! This module contains reusable widgets.

pub mod button;
pub mod screen;
pub mod folder_input;
pub mod context_menu;

use bevy::prelude::*;
use crate::widget::button::ButtonPlugin;
use crate::widget::folder_input::FolderInputPlugin;
use crate::widget::screen::ScreenPlugin;

pub(crate) struct EditorWidgetPlugin;

impl Plugin for EditorWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ButtonPlugin, ScreenPlugin, FolderInputPlugin));
    }
}
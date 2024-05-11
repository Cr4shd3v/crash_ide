mod root;
mod startup;
mod widget;
mod button;

use bevy::prelude::*;
pub use root::*;
use crate::button::button_cursor;
use crate::startup::StartupScreenPlugin;
use crate::widget::WidgetPlugin;

pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, setup_ui)
            .add_systems(Update, button_cursor)
            .add_plugins((StartupScreenPlugin, WidgetPlugin))
        ;
    }
}
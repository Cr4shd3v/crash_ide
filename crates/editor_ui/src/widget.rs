//! This module contains reusable widgets.

mod hoverable;
pub mod button;
pub mod input;

use bevy::prelude::*;
pub use hoverable::*;
use crate::widget::button::ButtonPlugin;
use crate::widget::input::InputPlugin;

pub(crate) struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HoverablePlugin, ButtonPlugin, InputPlugin));
    }
}
mod hoverable;
pub mod button;

use bevy::prelude::*;
pub use hoverable::*;
use crate::widget::button::ButtonPlugin;

pub(crate) struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HoverablePlugin, ButtonPlugin));
    }
}
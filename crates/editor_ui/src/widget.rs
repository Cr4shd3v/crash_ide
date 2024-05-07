mod hoverable;

use bevy::prelude::*;
pub use hoverable::*;

pub(crate) struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HoverablePlugin);
    }
}
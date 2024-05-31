mod hoverable;
mod input;
mod button;
mod window;

use bevy::prelude::*;
pub use hoverable::*;
pub use input::*;
pub use button::*;
pub use window::*;

pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HoverablePlugin, ButtonPlugin, InputPlugin, WindowPlugin));
    }
}
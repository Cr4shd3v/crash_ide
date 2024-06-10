//! Contains reusable, independent widgets for bevy.

#![warn(missing_docs)]

mod hoverable;
mod input;
mod button;
mod window;
mod focus_node;

use bevy::prelude::*;
pub use hoverable::*;
pub use input::*;
pub use button::*;
pub use window::*;
pub use focus_node::*;

/// Plugin adding all widget systems.
pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HoverablePlugin, ButtonPlugin, InputPlugin, WindowPlugin, FocusNodePlugin));
    }
}
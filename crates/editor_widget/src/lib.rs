//! Contains reusable, independent widgets for bevy.

#![warn(missing_docs)]

mod hoverable;
mod input;
mod button;
mod window;
mod expandable_menu;

use bevy::prelude::*;
pub use hoverable::*;
pub use input::*;
pub use button::*;
pub use window::*;
pub use expandable_menu::*;

/// Plugin adding all widget systems.
pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HoverablePlugin, ButtonPlugin, InputPlugin, WindowPlugin));
    }
}
mod double_click_button;

use bevy::prelude::*;
pub use double_click_button::*;

pub(super) struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, double_click_detection);
    }
}
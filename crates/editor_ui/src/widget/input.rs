//! This module contains input widgets.

mod text_input_field;
pub use text_input_field::*;

use bevy::prelude::*;

pub(super) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TextInputPlugin)
        ;
    }
}
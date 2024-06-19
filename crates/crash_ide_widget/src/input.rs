mod text_input_field;
mod right_clicked;
mod checkbox;

pub use text_input_field::*;
pub use right_clicked::*;
pub use checkbox::*;

use bevy::prelude::*;

pub(super) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((TextInputPlugin, RightClickedPlugin, CheckboxPlugin))
        ;
    }
}
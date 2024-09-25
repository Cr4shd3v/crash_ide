mod right_clicked;
mod checkbox;

pub use right_clicked::*;
pub use checkbox::*;

use bevy::prelude::*;

pub(super) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((RightClickedPlugin, CheckboxPlugin))
        ;
    }
}
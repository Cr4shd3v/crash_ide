use bevy::prelude::*;
use crate::extension::text::TextPlugin;

pub mod text;

pub(crate) struct StandardExtensionPlugin;

impl Plugin for StandardExtensionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TextPlugin)
        ;
    }
}
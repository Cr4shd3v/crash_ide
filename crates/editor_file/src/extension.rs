use bevy::prelude::*;
use crate::extension::sound::SoundPlugin;
use crate::extension::text::TextPlugin;

pub mod text;
mod sound;

pub(crate) struct StandardExtensionPlugin;

impl Plugin for StandardExtensionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((TextPlugin, SoundPlugin))
        ;
    }
}
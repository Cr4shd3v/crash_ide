use bevy::prelude::*;
use crate::extension::sound::SoundPlugin;
use crate::extension::text::TextPlugin;
use crate::video::VideoPlugin;

pub mod text;
pub mod sound;
pub mod video;

pub(crate) struct StandardExtensionPlugin;

impl Plugin for StandardExtensionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((TextPlugin, SoundPlugin, VideoPlugin))
        ;
    }
}
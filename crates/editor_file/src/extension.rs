use bevy::prelude::*;
use crate::extension::image::ImagePlugin;
use crate::extension::sound::SoundPlugin;
use crate::extension::text::TextPlugin;
use crate::video::VideoPlugin;

pub mod text;
pub mod sound;
pub mod video;
pub mod image;

pub(crate) struct StandardExtensionPlugin;

impl Plugin for StandardExtensionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((TextPlugin, SoundPlugin, VideoPlugin, ImagePlugin))
        ;
    }
}
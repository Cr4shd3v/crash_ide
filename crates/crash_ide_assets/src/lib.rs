mod fonts;
mod icons;
mod colors;

pub use fonts::*;
pub use icons::*;
pub use colors::*;

use bevy::prelude::*;

pub struct EditorAssetPlugin;

impl Plugin for EditorAssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((DefaultFontsPlugin, IconPlugin))
        ;
    }
}

mod fonts;
pub use fonts::*;

use bevy::prelude::*;

pub struct EditorAssetPlugin;

impl Plugin for EditorAssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultFontsPlugin)
        ;
    }
}
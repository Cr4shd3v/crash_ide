mod fonts;
mod icons;
mod colors;

pub use fonts::*;
pub use icons::*;
pub use colors::*;

use bevy::prelude::*;

pub struct CrashIDEAssetPlugin;

impl Plugin for CrashIDEAssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((DefaultFontsPlugin, IconPlugin))
        ;
    }
}

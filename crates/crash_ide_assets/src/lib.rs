#![warn(missing_docs)]

//! Crate handling all assets for the editor.

mod fonts;
mod icons;
mod colors;

pub use fonts::*;
pub use icons::*;
pub use colors::*;

use bevy::prelude::*;

/// Plugin responsible for loading all required resources.
pub struct CrashIDEAssetPlugin;

impl Plugin for CrashIDEAssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((DefaultFontsPlugin, IconPlugin))
        ;
    }
}

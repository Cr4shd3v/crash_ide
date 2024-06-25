#![warn(missing_docs)]
//! Crate implementing the code view of the editor.

mod bundle;

use bevy::prelude::*;
pub use bundle::*;

/// Plugin implementing the code view of the editor.
pub struct CrashIDECodeViewPlugin;

impl Plugin for CrashIDECodeViewPlugin {
    fn build(&self, _app: &mut App) {

    }
}
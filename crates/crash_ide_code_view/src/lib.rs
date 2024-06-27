#![warn(missing_docs)]
//! Crate implementing the code view of the editor.

mod bundle;
mod create;
mod component;

use bevy::prelude::*;
pub use bundle::*;
use crate::create::create_code_view;

/// Plugin implementing the code view of the editor.
pub struct CrashIDECodeViewPlugin;

impl Plugin for CrashIDECodeViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, create_code_view)
        ;
    }
}
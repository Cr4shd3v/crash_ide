//! This crate contains file loading and display

#![warn(missing_docs)]

mod event;
mod handler;
mod extension;
mod file_view;

use bevy::prelude::*;
pub use event::*;
pub use handler::*;
pub use extension::*;
pub use file_view::*;

/// Plugin implementing file loading & display
pub struct EditorFilePlugin;

impl Plugin for EditorFilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RawOpenFileEvent>()
            .init_resource::<FileExtensionManager>()
            .add_plugins(StandardExtensionPlugin)
            .add_systems(PreUpdate, handle_raw_file_event)
        ;
    }
}
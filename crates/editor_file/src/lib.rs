mod event;
mod handler;

use bevy::prelude::*;
pub use event::*;

pub struct EditorFilePlugin;

impl Plugin for EditorFilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RawOpenFileEvent>()
        ;
    }
}
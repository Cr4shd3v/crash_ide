mod event;
mod util;
mod project;
mod files;

use bevy::prelude::*;
pub use event::*;
pub use util::*;
pub use project::*;
pub use files::*;

pub struct CrashIDEProjectPlugin;

impl Plugin for CrashIDEProjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<OpenProjectEvent>()
            .add_event::<CloseProjectEvent>()
        ;
    }
}
mod event;
mod util;
mod project;

use bevy::prelude::*;
pub use event::*;
pub use util::*;
pub use project::*;

pub struct CrashIDEProjectPlugin;

impl Plugin for CrashIDEProjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<OpenProjectEvent>()
            .add_event::<CloseProjectEvent>()
        ;
    }
}
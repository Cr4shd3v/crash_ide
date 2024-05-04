mod project;
mod home_dir;
mod load;

pub use project::*;
pub use home_dir::*;
use bevy::prelude::*;

pub struct EditorConfigPlugin;

impl Plugin for EditorConfigPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (load_home_dir, load_projects_config).chain())
        ;
    }
}
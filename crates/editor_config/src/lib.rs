mod project;
mod home_dir;
mod load;

pub use project::*;
pub use home_dir::*;
pub(crate) use load::*;
use bevy::prelude::*;
use editor_state::EditorState;

pub struct EditorConfigPlugin;

impl Plugin for EditorConfigPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ConfigLoadStatus>()
            .add_systems(Startup, (load_home_dir, load_projects_config).chain())
            .add_systems(Update, check_config_load_status.run_if(in_state(EditorState::Loading)))
        ;
    }
}
//! Crate implementing configurations for the editor

#![warn(missing_docs)]

mod project;
mod home_dir;
mod load;

pub use project::*;
pub use home_dir::*;
pub(crate) use load::*;
use bevy::prelude::*;
use crash_ide_state::EditorState;

/// Plugin implementing configuration
pub struct CrashIDEConfigPlugin;

impl Plugin for CrashIDEConfigPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ConfigLoadStatus>()
            .add_systems(Startup, (load_home_dir, load_projects_config).chain())
            .add_systems(Update, check_config_load_status.run_if(in_state(EditorState::Loading)))
            .add_systems(Update, save_config_on_change::<EditorConfigProjects>())
        ;
    }
}
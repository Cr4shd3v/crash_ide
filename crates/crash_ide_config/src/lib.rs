//! Crate implementing configurations for the editor

#![warn(missing_docs)]

pub extern crate serde_json;

mod project;
mod home_dir;
mod load;
mod general;
mod plugin;

pub use project::*;
pub use home_dir::*;
pub use general::*;
pub use load::*;
pub use plugin::*;
use bevy::prelude::*;
use crash_ide_state::EditorState;

/// Plugin implementing configuration
pub struct CrashIDEConfigPlugin;

impl Plugin for CrashIDEConfigPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ConfigLoadStatus>()
            .add_systems(PreStartup, load_home_dir)
            .add_systems(Startup, (load_projects_config, load_general_settings))
            .add_systems(Update, check_config_load_status.run_if(in_state(EditorState::Loading)))
            .add_systems(Update, (
                save_config_on_change::<EditorConfigProjects>(),
                save_config_on_change::<GeneralSettings>(),
            ))
        ;
    }
}
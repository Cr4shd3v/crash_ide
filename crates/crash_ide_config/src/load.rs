macro_rules! default_load_config {
    ($name:ident,$struct_type:ty,$status_field:ident) => {
        pub(crate) fn $name (mut commands: Commands, home_dir: Res<crate::HomeDir>, mut load_status: ResMut<crate::ConfigLoadStatus>) {
            let projects_config_path = home_dir.config_path.join(<$struct_type>::FILENAME);
            let projects_config = if std::fs::metadata(&projects_config_path).is_err() {
                let config = <$struct_type>::default();
                std::fs::write(&projects_config_path, serde_json::to_vec(&config).unwrap()).unwrap();
                config
            } else {
                serde_json::from_slice::<$struct_type>(std::fs::read(&projects_config_path).unwrap().as_slice()).unwrap_or(<$struct_type>::default())
            };

            commands.insert_resource(projects_config);
            load_status.$status_field = true;
        }
    };
}

use std::fs;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
pub(crate) use default_load_config;
use crash_ide_state::EditorLoadStatus;
use crate::HomeDir;

/// Trait that must be implemented for all configs.
pub trait EditorConfig: Resource + Serialize + for<'a> Deserialize<'a> + Default {
    /// Name of the file that will be saved
    const FILENAME: &'static str;
}

/// Load status of the editor itself and its plugins.
#[derive(Resource, Default)]
pub struct ConfigLoadStatus {
    /// Projects loaded?
    pub projects: bool,
    /// General settings loaded?
    pub general_settings: bool,
    /// Plugin settings loaded?
    ///
    /// This counter is increased with each plugin registered with a config.
    pub plugin_counter: u32,
}

pub(crate) fn check_config_load_status(config_load_status: Res<ConfigLoadStatus>, mut load_status: ResMut<EditorLoadStatus>) {
    if config_load_status.is_changed() {
        if config_load_status.projects && config_load_status.general_settings && config_load_status.plugin_counter == 0 {
            load_status.config_loaded = true;
        }
    }
}

/// Generates a system to save every change to an [EditorConfig] to the filesystem
pub fn save_config_on_change<T: EditorConfig>() -> impl FnMut(Res<T>, Res<HomeDir>) {
    move |res: Res<T>, home_dir: Res<HomeDir>| {
        if res.is_changed() {
            fs::write(home_dir.config_path.join(T::FILENAME), serde_json::to_vec(&*res).unwrap()).unwrap();
        }
    }
}
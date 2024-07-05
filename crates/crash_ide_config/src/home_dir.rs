use std::fs;
use std::path::PathBuf;
use bevy::prelude::{Commands, Resource};
use homedir::my_home;

/// Resource containing the config and project path
#[derive(Resource)]
pub struct HomeDir {
    /// Path to config files
    pub config_path: PathBuf,
    /// Default path to projects
    pub projects_path: PathBuf,
    /// Path to plugins
    pub plugin_path: PathBuf,
}

pub(crate) fn load_home_dir(mut commands: Commands) {
    let home_path = my_home().expect("Could not determine home directory").unwrap();
    let base_config_path = home_path.join(".crash_ide");
    if fs::metadata(&base_config_path).is_err() {
        fs::create_dir(&base_config_path).unwrap();
    }

    let config_path = home_path.join("config");
    if fs::metadata(&config_path).is_err() {
        fs::create_dir(&config_path).unwrap();
    }

    let config_plugin_path = config_path.join("plugins");
    if fs::metadata(&config_plugin_path).is_err() {
        fs::create_dir(config_plugin_path).unwrap();
    }

    let plugin_path = base_config_path.join("plugins");
    if fs::metadata(&plugin_path).is_err() {
        fs::create_dir(&plugin_path).unwrap();
    }

    let projects_path = home_path.join("CrashIDEProjects");
    if fs::metadata(&projects_path).is_err() {
        fs::create_dir(&projects_path).unwrap();
    }

    commands.insert_resource(HomeDir {
        config_path,
        projects_path,
        plugin_path,
    });
}
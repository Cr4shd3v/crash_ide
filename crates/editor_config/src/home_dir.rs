use std::fs;
use std::path::PathBuf;
use bevy::prelude::{Commands, Resource};

#[derive(Resource)]
pub struct HomeDir {
    pub config_path: PathBuf,
    pub projects_path: PathBuf,
}

pub(crate) fn load_home_dir(mut commands: Commands) {
    let home_dir = std::env::var("HOME").expect("Could not determine home directory");
    let home_path = PathBuf::from(home_dir);
    let config_path = home_path.join(".crash_ide");
    if fs::metadata(&config_path).is_err() {
        fs::create_dir(&config_path).unwrap();
    }

    let projects_path = home_path.join("CrashIDEProjects");
    if fs::metadata(&projects_path).is_err() {
        fs::create_dir(&projects_path).unwrap();
    }

    commands.insert_resource(HomeDir {
        config_path,
        projects_path,
    });
}
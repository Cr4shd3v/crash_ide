use bevy::prelude::*;

mod instance;

pub use instance::*;

pub struct CrashIDEFileWatcherPlugin;

impl Plugin for CrashIDEFileWatcherPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_file_watcher)
        ;
    }
}


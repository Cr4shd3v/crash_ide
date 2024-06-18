use std::path::Path;
use bevy::prelude::*;
use notify::{RecursiveMode, Watcher};

pub struct CrashIDEFileWatcherPlugin;

impl Plugin for CrashIDEFileWatcherPlugin {
    fn build(&self, app: &mut App) {

    }
}

#[derive(Component)]
pub struct FileWatcher {
    pub path: String,
}

#[derive(Component)]
pub struct FileWatcherInstance {

}

fn spawn_file_watcher(
    mut commands: Commands,
    query: Query<(Entity, &FileWatcher), Added<FileWatcher>>,
) {
    for (entity, watcher) in query.iter() {
        let watcher_instance = notify::recommended_watcher(move |res: notify::Result<notify::Event> | {
            match res {
                Ok(event) => {}
                Err(e) => {}
            }
        });

        let Ok(mut watcher_instance) = watcher_instance else {
            continue;
        };

        if let Err(e) = watcher_instance.watch(Path::new(&watcher.path), RecursiveMode::Recursive) {
            println!("Failed to watch directory {}: {}", &watcher.path, e);
        }
    }
}


use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::mpsc::{channel, Receiver};

use bevy::prelude::*;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
pub use notify::{Event, event::*};

#[derive(Component)]
pub struct FileWatcher {
    /// Path this file watcher
    pub path: String,
}

#[derive(Component)]
pub struct FileWatcherInstance {
    #[allow(unused)]
    watcher: RecommendedWatcher,
    receiver: Arc<Mutex<Receiver<Event>>>,
}

impl FileWatcherInstance {
    /// Try polling for events in filesystem
    pub fn receiver(&self) -> MutexGuard<'_, Receiver<Event>> {
        self.receiver.lock().unwrap()
    }
}

pub(super) fn spawn_file_watcher(
    mut commands: Commands,
    query: Query<(Entity, &FileWatcher), Added<FileWatcher>>,
) {
    for (entity, watcher) in query.iter() {
        let (tx, rx) = channel();

        let watcher_instance = notify::recommended_watcher(move |res: notify::Result<Event> | {
            match res {
                Ok(event) => {
                    tx.send(event).ok();
                }
                Err(e) => {
                    error!("Error receiving watcher event: {}", e);
                }
            }
        });

        let Ok(mut watcher_instance) = watcher_instance else {
            continue;
        };

        if let Err(e) = watcher_instance.watch(Path::new(&watcher.path), RecursiveMode::Recursive) {
            error!("Failed to watch directory {}: {}", &watcher.path, e);
        }

        commands.entity(entity).insert(FileWatcherInstance {
            watcher: watcher_instance,
            receiver: Arc::new(Mutex::new(rx)),
        });
    }
}
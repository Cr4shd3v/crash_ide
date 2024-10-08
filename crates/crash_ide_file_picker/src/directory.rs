use std::path::PathBuf;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, block_on, Task};
use bevy::tasks::futures_lite::future;
use rfd::FileHandle;

#[derive(Component, Clone)]
pub struct DirectoryPicker {
    pub start_directory: Option<PathBuf>,
    pub title: String,
}

impl Default for DirectoryPicker {
    fn default() -> Self {
        Self {
            start_directory: None,
            title: "Select directory".to_string(),
        }
    }
}

#[derive(Component)]
pub(super) struct DirectoryPickerTask(Task<Option<FileHandle>>);

#[derive(Event)]
pub struct DirectoryPicked(pub FileHandle);

pub(super) fn start_directory_picker(
    mut commands: Commands,
    query: Query<(Entity, &DirectoryPicker), Added<DirectoryPicker>>,
) {
    let pool = AsyncComputeTaskPool::get();

    for (entity, picker) in query.iter() {
        let picker = picker.clone();

        let task = pool.spawn(async move {
            let mut dialog = rfd::AsyncFileDialog::new()
                .set_can_create_directories(true)
                .set_title(&picker.title);

            if let Some(start_directory) = &picker.start_directory {
                dialog = dialog.set_directory(start_directory);
            }

            dialog.pick_folder().await
        });

        commands.entity(entity).insert(DirectoryPickerTask(task)).remove::<DirectoryPicker>();
    }
}

pub(super) fn handle_picked_directory(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DirectoryPickerTask)>,
) {
    for (entity, mut task) in query.iter_mut() {
        let Some(result) = block_on(future::poll_once(&mut task.0)) else {
            continue;
        };

        commands.entity(entity).remove::<DirectoryPickerTask>();

        if let Some(file) = result {
            commands.trigger_targets(DirectoryPicked(file), entity);
        }
    }
}

use std::path::PathBuf;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, block_on, Task};
use bevy::tasks::futures_lite::future;
use rfd::FileHandle;

#[derive(Component, Clone)]
pub struct FilePicker {
    pub start_directory: Option<PathBuf>,
    pub title: String,
    pub save: bool,
    pub filename: Option<String>,
}

impl Default for FilePicker {
    fn default() -> Self {
        Self {
            start_directory: None,
            title: "Select file".to_string(),
            save: false,
            filename: None,
        }
    }
}

#[derive(Component)]
pub(super) struct FilePickerTask(Task<Option<FileHandle>>);

#[derive(Component)]
pub struct FilePicked(pub FileHandle);

pub(super) fn start_file_picker(
    mut commands: Commands,
    query: Query<(Entity, &FilePicker), Added<FilePicker>>,
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

            if picker.save {
                if let Some(filename) = picker.filename {
                    dialog = dialog.set_file_name(filename);
                }

                dialog.save_file().await
            } else {
                dialog.pick_file().await
            }
        });

        commands.entity(entity).insert(FilePickerTask(task)).remove::<(FilePicker, FilePicked)>();
    }
}

pub(super) fn handle_picked_file(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FilePickerTask)>,
) {
    for (entity, mut task) in query.iter_mut() {
        let Some(result) = block_on(future::poll_once(&mut task.0)) else {
            continue;
        };

        commands.entity(entity).remove::<FilePickerTask>();

        if let Some(file) = result {
            commands.entity(entity).insert(FilePicked(file));
        }
    }
}

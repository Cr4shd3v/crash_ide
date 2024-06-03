//! This module contains the implementation for .txt files

use std::fs;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, block_on, Task};
use bevy::tasks::futures_lite::future;
use editor_assets::DefaultFonts;
use editor_widget::{TextInputBundle, TextInputSettings, TextInputTextStyle, TextInputValue};
use crate::{default_file_handler_impl, FileEventData, FileHandlerAppExtension, FileViewInstance, OpenFileEvent};

pub(super) struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_file_handler::<TextFile>()
            .add_systems(Update, (load_text_file, spawn_file_view, save_edited_content))
        ;
    }
}

/// [FileHandler](crate::FileHandler) for .txt files
pub struct TextFile;

use crate as editor_file;
default_file_handler_impl!(TextFile, ["txt"], "text.png");

#[derive(Component)]
struct TextLoadingTask(Task<Option<String>>, FileEventData);

fn load_text_file(
    mut commands: Commands,
    mut event_reader: EventReader<OpenFileEvent<TextFile>>,
) {
    let pool = AsyncComputeTaskPool::get();
    for event in event_reader.read() {
        let path = event.event_data.path.clone();
        let task = pool.spawn(async move {
            let result = fs::read_to_string(&path);

            if result.is_err() {
                None
            } else {
                Some(result.unwrap())
            }
        });

        commands.spawn(TextLoadingTask(task, event.event_data.clone()));
    }
}

fn spawn_file_view(
    mut commands: Commands,
    mut task_query: Query<(Entity, &mut TextLoadingTask)>,
) {
    for (task_entity, mut loading_task) in task_query.iter_mut() {
        let Some(result) = block_on(future::poll_once(&mut loading_task.0)) else {
            continue;
        };

        commands.entity(task_entity).despawn();

        // Since TextFile is the default handler, we have to ensure that we can handle non-utf-8 files via OS default
        let Some(content) = result else {
            open::that_detached(&loading_task.1.path).unwrap();
            continue;
        };

        commands.entity(loading_task.1.view_entity).despawn_descendants().with_children(|parent| {
            parent.spawn((TextInputBundle {
                text_input_value: TextInputValue(content),
                text_input_text_style: TextInputTextStyle(TextStyle {
                    font: DefaultFonts::JETBRAINS_MONO_REGULAR,
                    font_size: 18.0,
                    ..default()
                }),
                text_input_settings: TextInputSettings {
                    with_border: false,
                    multiline: true,
                    ..default()
                },
                ..default()
            }, NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            }, FileViewInstance {
                path: loading_task.1.path.clone(),
            }));
        });
    }
}

fn save_edited_content(
    query: Query<(&TextInputValue, &FileViewInstance), Changed<TextInputValue>>
) {
    for (input_value, view_instance) in query.iter() {
        fs::write(&view_instance.path, &input_value.0).unwrap();
    }
}
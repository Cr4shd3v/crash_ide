//! This module contains the implementation for .txt files

use std::fs;

use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, block_on, Task};
use bevy::tasks::futures_lite::future;

use crash_ide_assets::DefaultFonts;
use crash_ide_code_view::{CodeViewBundle, CodeViewContent, CodeViewStyle};
use crash_ide_notification::{Notification, NotificationIcon};
use crash_ide_widget::{ActiveWindow, Scrollable, ScrollableContent, TextInputValue};

use crate::{default_file_handler_impl, FileEventData, FileHandlerAppExtension, FileViewInstance, OpenFileEvent};
use crate as crash_ide_file;

pub(super) struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_file_handler::<TextFile>()
            .add_systems(Update, (load_text_file, spawn_file_view, save_edited_content_timer, save_edited_content))
        ;
    }
}

/// [FileHandler](crate::FileHandler) for .txt files
pub struct TextFile;

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

#[derive(Component)]
struct TextFileView;

#[derive(Component)]
struct TextFilePendingChanges(Timer);

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
            parent.spawn((NodeBundle::default(), Scrollable::default(), Interaction::None)).with_children(|parent| {
                parent.spawn((
                    CodeViewBundle {
                        node_bundle: NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        },
                        content: CodeViewContent::from_string(content),
                        code_view_style: CodeViewStyle {
                            font_size: 18.0,
                            regular_font: DefaultFonts::JETBRAINS_MONO_REGULAR,
                            bold_font: DefaultFonts::JETBRAINS_MONO_BOLD,
                            italic_font: DefaultFonts::JETBRAINS_MONO_ITALIC,
                            bold_italic_font: DefaultFonts::JETBRAINS_MONO_BOLD_ITALIC,
                        },
                        ..default()
                    },
                    FileViewInstance {
                        path: loading_task.1.path.clone(),
                    },
                    ScrollableContent::default(),
                    TextFileView,
                ));
            });
        });
    }
}

fn save_edited_content_timer(
    mut commands: Commands,
    mut query: Query<(Entity, Option<&mut TextFilePendingChanges>), (Changed<TextInputValue>, With<TextFileView>)>,
) {
    for (entity, pending) in query.iter_mut() {
        if let Some(mut timer) = pending {
            timer.0.reset();
        } else {
            commands.entity(entity).insert(TextFilePendingChanges(Timer::from_seconds(1.0, TimerMode::Once)));
        }
    }
}

fn save_edited_content(
    mut commands: Commands,
    mut query: Query<(&TextInputValue, &FileViewInstance, &mut TextFilePendingChanges), With<TextFileView>>,
    window: Query<Entity, With<ActiveWindow>>,
    time: Res<Time>,
) {
    for (input_value, view_instance, mut pending) in query.iter_mut() {
        pending.0.tick(time.delta());

        if pending.0.finished() {
            if let Err(e) = fs::write(&view_instance.path, &input_value.0) {
                commands.spawn(Notification::new(
                    window.single(),
                    "Error while writing to disk".to_string(),
                    format!("Could not save content of {}: {}", &view_instance.path.to_str().unwrap(), e),
                    NotificationIcon::Error,
                ));
            }
        }
    }
}

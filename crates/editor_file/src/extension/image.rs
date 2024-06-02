//! This module contains the implementation for image files

use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::tasks::{AsyncComputeTaskPool, block_on, Task};
use bevy::tasks::futures_lite::future;
use crate::{default_file_handler_impl, FileEventData, FileHandlerAppExtension, OpenFileEvent};

pub(super) struct ImagePlugin;

impl Plugin for ImagePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_file_handler::<ImageFile>()
            .add_systems(Update, (open_image_file, spawn_image_view))
        ;
    }
}

/// [FileHandler](crate::FileHandler) for image files
pub struct ImageFile;

use crate as editor_file;
default_file_handler_impl!(
    ImageFile,
    ["bmp", "dds", "ff", "hdr", "ico", "jpeg", "jpg", "exr", "png",
        "pbm", "pam", "ppm", "pgm", "qoi", "tga", "tif", "tiff", "webp"],
    "image.png"
);

#[derive(Component)]
struct ImageLoadingTask(Task<Option<Image>>, FileEventData);

fn open_image_file(
    mut commands: Commands,
    mut event_reader: EventReader<OpenFileEvent<ImageFile>>,
) {
    let pool = AsyncComputeTaskPool::get();
    for event in event_reader.read() {
        let path = event.event_data.path.clone();
        let task = pool.spawn(async move {
            let Ok(result) = image::open(&path) else {
                return None;
            };

            Some(Image::from_dynamic(result, true, RenderAssetUsages::all()))
        });

        commands.spawn(ImageLoadingTask(task, event.event_data.clone()));
    }
}

fn spawn_image_view(
    mut commands: Commands,
    mut task_query: Query<(Entity, &mut ImageLoadingTask)>,
    mut image_assets: ResMut<Assets<Image>>,
) {
    for (task_entity, mut loading_task) in task_query.iter_mut() {
        let Some(result) = block_on(future::poll_once(&mut loading_task.0)) else {
            continue;
        };

        commands.entity(task_entity).despawn();

        let Some(image) = result else {
            open::that_detached(&loading_task.1.path).unwrap();
            continue;
        };

        let size = image.size();
        let handle = image_assets.add(image);

        commands.entity(loading_task.1.view_entity).despawn_descendants().with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn(ImageBundle {
                    image: UiImage {
                        texture: handle,
                        ..default()
                    },
                    style: Style {
                        width: Val::Px(size.x as f32),
                        height: Val::Px(size.y as f32),
                        ..default()
                    },
                    ..default()
                });
            });
        });
    }
}
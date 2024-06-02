//! This module contains the implementation for video files

use bevy::prelude::*;
use crate::{default_file_handler_impl, FileHandlerManager, OpenFileEvent};

pub(super) struct VideoPlugin;

impl Plugin for VideoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<OpenFileEvent<VideoFile>>()
            .add_systems(Startup, register_handler)
            .add_systems(Update, open_video_file)
        ;
    }
}

/// [FileHandler](crate::FileHandler) for video files
pub struct VideoFile;

use crate as editor_file;
default_file_handler_impl!(VideoFile, ["mp4", "mkv", "gif", "wmv", "mov"]);

fn register_handler(mut file_handler_manager: ResMut<FileHandlerManager>) {
    file_handler_manager.register_handler::<VideoFile>();
}

fn open_video_file(mut event_reader: EventReader<OpenFileEvent<VideoFile>>) {
    for event in event_reader.read() {
        open::that_detached(&event.event_data.path).unwrap();
    }
}
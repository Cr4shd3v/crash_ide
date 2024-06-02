//! This module contains the implementation for video files

use bevy::prelude::*;

use crate::{default_file_handler_impl, FileHandlerAppExtension, OpenFileEvent};
use crate as editor_file;

pub(super) struct VideoPlugin;

impl Plugin for VideoPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_file_handler::<VideoFile>()
            .add_systems(Update, open_video_file)
        ;
    }
}

/// [FileHandler](crate::FileHandler) for video files
pub struct VideoFile;

default_file_handler_impl!(VideoFile, ["mp4", "mkv", "gif", "wmv", "mov"], "video.png");

fn open_video_file(mut event_reader: EventReader<OpenFileEvent<VideoFile>>) {
    for event in event_reader.read() {
        open::that_detached(&event.event_data.path).unwrap();
    }
}
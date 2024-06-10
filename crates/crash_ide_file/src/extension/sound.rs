//! This module contains the implementation for sound files

use bevy::prelude::*;

use crate::{default_file_handler_impl, FileHandlerAppExtension, OpenFileEvent};
use crate as crash_ide_file;

pub(super) struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_file_handler::<SoundFile>()
            .add_systems(Update, open_sound_file)
        ;
    }
}

/// [FileHandler](crate::FileHandler) for sound files
pub struct SoundFile;

default_file_handler_impl!(SoundFile, ["mp3", "wav", "flac", "ogg", "oga", "mogg"], "sound.png");

fn open_sound_file(mut event_reader: EventReader<OpenFileEvent<SoundFile>>) {
    for event in event_reader.read() {
        open::that_detached(&event.event_data.path).unwrap();
    }
}
use bevy::prelude::*;
use crate::{default_file_handler_impl, FileHandlerManager, OpenFileEvent};

pub(super) struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<OpenFileEvent<SoundFile>>()
            .add_systems(Startup, register_handler)
            .add_systems(Update, open_sound_file)
        ;
    }
}

pub struct SoundFile;

use crate as editor_file;
default_file_handler_impl!(SoundFile, ["mp3", "wav", "flac", "ogg", "oga", "mogg"]);

fn register_handler(mut file_handler_manager: ResMut<FileHandlerManager>) {
    file_handler_manager.register_handler::<SoundFile>();
}

fn open_sound_file(mut event_reader: EventReader<OpenFileEvent<SoundFile>>) {
    for event in event_reader.read() {
        open::that_detached(&event.event_data.path).unwrap();
    }
}
use std::ffi::OsStr;
use std::marker::PhantomData;
use std::path::PathBuf;
use bevy::prelude::*;
use crate::{FileHandlerManager, FileViewInstance};

/// Contains common event data for [RawOpenFileEvent] and [OpenFileEvent]
#[derive(Clone)]
pub struct FileEventData {
    /// File view [Entity]
    pub view_entity: Entity,
    /// Path to the file
    pub path: PathBuf,
}

/// Raw open file event.
///
/// This will be converted to a typed [OpenFileEvent]
#[derive(Event)]
pub struct RawOpenFileEvent {
    /// See [FileEventData]
    pub event_data: FileEventData,
}

impl RawOpenFileEvent {
    /// Converts self to an [OpenFileEvent] by cloning all data
    pub fn to_type_event<T>(&self) -> OpenFileEvent<T> {
        OpenFileEvent {
            event_data: self.event_data.clone(),
            phantom_data: PhantomData,
        }
    }
}

/// Typed event to open a file.
///
/// The generic parameter marks the corresponding [FileHandler](crate::FileHandler).
#[derive(Event)]
pub struct OpenFileEvent<T> {
    /// See [FileEventData]
    pub event_data: FileEventData,
    #[allow(missing_docs)]
    phantom_data: PhantomData<T>,
}

pub(super) fn handle_raw_file_event(
    mut commands: Commands,
    mut event_reader: EventReader<RawOpenFileEvent>,
    handler_manager: Res<FileHandlerManager>,
) {
    for event in event_reader.read() {
        let handler = handler_manager.get_handler(
            &event.event_data.path.extension().unwrap_or(&OsStr::new("")).to_str().unwrap().to_string()
        );

        commands.entity(event.event_data.view_entity).insert(FileViewInstance {
            path: event.event_data.path.clone(),
        });

        if let Some(handler) = handler {
            handler.create_event(&mut commands, event);
        }
    }
}
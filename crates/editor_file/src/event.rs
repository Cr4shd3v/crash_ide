use std::marker::PhantomData;
use std::path::PathBuf;
use bevy::prelude::*;

#[derive(Event)]
pub struct RawOpenFileEvent {
    pub view_entity: Entity,
    pub path: PathBuf,
}

impl RawOpenFileEvent {
    pub fn to_type_event<T>(&self) -> OpenFileEvent<T> {
        OpenFileEvent {
            view_entity: self.view_entity,
            path: self.path.clone(),
            phantom_data: PhantomData,
        }
    }
}

#[derive(Event)]
pub struct OpenFileEvent<T> {
    pub view_entity: Entity,
    pub path: PathBuf,
    pub phantom_data: PhantomData<T>,
}
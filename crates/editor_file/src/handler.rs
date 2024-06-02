use std::sync::Arc;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::RawOpenFileEvent;

#[derive(Resource, Default)]
pub struct FileHandlerManager {
    handler_map: HashMap<String, Arc<Box<dyn FileHandler>>>,
}

impl FileHandlerManager {
    pub fn register_handler<T: FileHandler, R: Event>(&mut self) {
        let handler: Arc<Box<dyn FileHandler>> = Arc::new(Box::new(T::get_instance()));

        for extension in handler.get_file_extensions() {
            self.handler_map.insert(extension.to_string(), handler.clone());
        }
    }

    pub fn get_handler(&self, extension: &String) -> Option<&Arc<Box<dyn FileHandler>>> {
        self.handler_map.get(extension)
    }
}

pub trait FileHandler: Sync + Send + 'static {

    fn get_instance() -> Self where Self: Sized;

    fn get_file_extensions(&self) -> Vec<&'static str>;

    fn create_event(&self, commands: Commands, raw_event: &RawOpenFileEvent);
}

pub struct TextFileHandler;

impl FileHandler for TextFileHandler {
    fn get_instance() -> Self where Self: Sized {
        TextFileHandler
    }

    fn get_file_extensions(&self) -> Vec<&'static str> {
        vec!["txt"]
    }

    fn create_event(&self, mut commands: Commands, raw_event: &RawOpenFileEvent) {
        let typed_event = raw_event.to_type_event::<TextFileHandler>();

        commands.add(|w: &mut World| {
            w.send_event(typed_event);
        });
    }
}
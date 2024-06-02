use std::sync::Arc;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::{OpenFileEvent, RawOpenFileEvent};

/// Resource containing all [FileHandler]
#[derive(Resource, Default)]
pub struct FileHandlerManager {
    handler_map: HashMap<String, Arc<Box<dyn FileHandler>>>,
}

impl FileHandlerManager {
    /// Register a [FileHandler].
    ///
    /// Must be called before opening a file of that type.
    pub fn register_handler<T: FileHandler>(&mut self) {
        let handler: Arc<Box<dyn FileHandler>> = Arc::new(Box::new(T::get_instance()));

        for extension in handler.get_file_extensions() {
            self.handler_map.insert(extension.to_string(), handler.clone());
        }
    }

    /// Retrieve a [FileHandler] by extension.
    pub fn get_handler(&self, extension: &String) -> Option<&Arc<Box<dyn FileHandler>>> {
        self.handler_map.get(extension)
    }
}

/// Trait for all file handler implementations
pub trait FileHandler: Sync + Send + 'static {
    /// Creates an instance of Self.
    fn get_instance() -> Self where Self: Sized;

    /// Returns all file extension this [FileHandler] should be used
    fn get_file_extensions(&self) -> Vec<&'static str>;

    /// Generates an [OpenFileEvent] for this type
    fn create_event(&self, commands: &mut Commands, raw_event: &RawOpenFileEvent);
}

/// Generates a default implementation for a [FileHandler].
#[macro_export]
macro_rules! default_file_handler_impl {
    ($handler_type:tt, $extensions:expr) => {
        impl editor_file::FileHandler for $handler_type {
            fn get_instance() -> Self where Self: Sized {
                $handler_type
            }

            fn get_file_extensions(&self) -> Vec<&'static str> {
                $extensions.into()
            }

            fn create_event(&self, commands: &mut Commands, raw_event: &editor_file::RawOpenFileEvent) {
                let typed_event = raw_event.to_type_event::<$handler_type>();

                commands.add(|w: &mut World| {
                    w.send_event(typed_event);
                });
            }
        }
    };
}

/// Helper trait to register a file handler with a single call on [App]
pub trait FileHandlerAppExtension {
    /// Register a [FileHandler] with a single call
    fn register_file_handler<T: FileHandler>(&mut self) -> &mut Self;
}

impl FileHandlerAppExtension for App {
    fn register_file_handler<T: FileHandler>(&mut self) -> &mut Self {
        self.add_event::<OpenFileEvent<T>>()
            .add_systems(Startup, default_register_handler::<T>())
    }
}

fn default_register_handler<T: FileHandler>() -> impl FnMut(ResMut<FileHandlerManager>) {
    move |mut handler_manager: ResMut<FileHandlerManager>| {
        handler_manager.register_handler::<T>();
    }
}
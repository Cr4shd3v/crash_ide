use std::sync::Arc;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::{OpenFileEvent, RawOpenFileEvent};

/// Resource containing all [FileHandler]
#[derive(Resource, Default)]
pub struct FileExtensionManager {
    handler_map: HashMap<String, Arc<FileExtensionData>>,
}

impl FileExtensionManager {
    /// Register a [FileHandler].
    ///
    /// Must be called before opening a file of that type.
    pub fn register_handler<T: FileHandler>(&mut self, asset_server: Res<AssetServer>) {
        let handler: Box<dyn FileHandler> = Box::new(T::get_instance());
        let icon_name = T::get_icon_name();
        let icon = asset_server.load(format!("icons/extension/{}", icon_name));
        let extensions = handler.get_file_extensions();
        let data = Arc::new(FileExtensionData {
            handler,
            icon,
        });

        for extension in extensions {
            self.handler_map.insert(extension.to_string(), data.clone());
        }
    }

    /// Retrieve a [FileHandler] by extension.
    pub fn get_data(&self, extension: &String) -> Option<&Arc<FileExtensionData>> {
        self.handler_map.get(extension)
    }
}

/// Contains the [FileHandler] and the icon for a file extension
pub struct FileExtensionData {
    handler: Box<dyn FileHandler>,
    icon: Handle<Image>,
}

impl FileExtensionData {
    /// Get the [FileHandler]
    pub fn get_handler(&self) -> &Box<dyn FileHandler> {
        &self.handler
    }

    /// Get the icon handle
    pub fn get_icon(&self) -> &Handle<Image> {
        &self.icon
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

    /// Returns the icon name for this type
    fn get_icon_name() -> &'static str where Self: Sized;
}

/// Generates a default implementation for a [FileHandler].
#[macro_export]
macro_rules! default_file_handler_impl {
    ($handler_type:tt, $extensions:expr, $icon_name:expr) => {
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

            fn get_icon_name() -> &'static str {
                $icon_name
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

fn default_register_handler<T: FileHandler>() -> impl FnMut(ResMut<FileExtensionManager>, Res<AssetServer>) {
    move |mut handler_manager: ResMut<FileExtensionManager>, asset_server: Res<AssetServer>| {
        handler_manager.register_handler::<T>(asset_server);
    }
}
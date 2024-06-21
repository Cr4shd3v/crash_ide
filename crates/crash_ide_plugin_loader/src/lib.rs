use bevy::prelude::*;
use wasmtime::Engine;

use crate::load::load_plugins;
use crate::messages::PluginMessagesPlugin;

mod plugin_instance;
mod stream;
mod load;
mod messages;

pub use plugin_instance::*;

pub struct CrashIDEPluginLoaderPlugin;

impl Plugin for CrashIDEPluginLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GlobalWasmEngine>()
            .add_systems(Startup, load_plugins)
            .add_plugins(PluginMessagesPlugin)
        ;
    }
}

#[derive(Resource, Default)]
pub(crate) struct GlobalWasmEngine {
    pub engine: Engine,
}
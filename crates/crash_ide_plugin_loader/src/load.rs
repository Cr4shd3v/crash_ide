use bevy::prelude::{Commands, Res};
use crash_ide_config::HomeDir;
use crate::GlobalWasmEngine;
use crate::plugin_instance::PluginInstance;

pub(super) fn load_plugins(
    mut commands: Commands,
    wasm_engine: Res<GlobalWasmEngine>,
    home_dir: Res<HomeDir>,
) {
    for entry in home_dir.plugin_path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension.to_str().unwrap() != "wasm" {
                    continue;
                }

                println!("Load plugin {}", path.to_str().unwrap());
                let instance = match PluginInstance::new(&wasm_engine.engine, path) {
                    Ok(instance) => instance,
                    Err(e) => {
                        println!("Error loading plugin: {}", e);
                        continue;
                    }
                };

                commands.spawn(instance);
            }
        }
    }
}
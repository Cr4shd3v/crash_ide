use bevy::prelude::*;
use crash_ide_config::HomeDir;

pub struct CrashIDEPluginLoaderPlugin;

impl Plugin for CrashIDEPluginLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load_plugins)
        ;
    }
}

fn load_plugins(
    home_dir: Res<HomeDir>,
) {
    for entry in home_dir.plugin_path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path().to_str().unwrap();
            if path.ends_with(".wasm") {
                println!("Load plugin {}", entry.path().to_str().unwrap());
            }
        }
    }
}
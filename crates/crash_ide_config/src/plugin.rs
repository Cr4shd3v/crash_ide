use std::fs;
use bevy::prelude::*;
use crate::{ConfigLoadStatus, EditorConfig, HomeDir};

/// Adds the ability to simply register a plugin config.
pub trait ConfigAppExt {
    /// Registers a [EditorConfig] as a plugin config.
    fn register_plugin_config<T: EditorConfig>(&mut self) -> &mut Self;
}

impl ConfigAppExt for App {
    fn register_plugin_config<T: EditorConfig>(&mut self) -> &mut Self {
        self.add_systems(PreStartup, register_function::<T>)
            .add_systems(Startup, load_function::<T>)
            .add_systems(Update, save_plugin_config_on_change::<T>())
    }
}

fn register_function<T: EditorConfig>(mut load_status: ResMut<ConfigLoadStatus>) {
    load_status.plugin_counter += 1;
}

fn load_function<T: EditorConfig>(mut commands: Commands, home_dir: Res<HomeDir>, mut load_status: ResMut<ConfigLoadStatus>) {
    let projects_config_path = home_dir.config_path.join("plugins").join(T::FILENAME);
    let config = if std::fs::metadata(&projects_config_path).is_err() {
        let config = T::default();
        std::fs::write(&projects_config_path, serde_json::to_vec(&config).unwrap()).unwrap();
        config
    } else {
        serde_json::from_slice::<T>(std::fs::read(&projects_config_path).unwrap().as_slice())
            .unwrap_or(T::default())
    };

    commands.insert_resource(config);
    load_status.plugin_counter -= 1;
}

fn save_plugin_config_on_change<T: EditorConfig>() -> impl FnMut(Res<T>, Res<HomeDir>) {
    move |res: Res<T>, home_dir: Res<HomeDir>| {
        if res.is_changed() {
            fs::write(home_dir.config_path.join("plugins").join(T::FILENAME), serde_json::to_vec(&*res).unwrap()).unwrap();
        }
    }
}
macro_rules! default_load_config {
    ($name:ident,$struct_type:ty,$status_field:ident) => {
        pub(crate) fn $name (mut commands: Commands, home_dir: Res<crate::HomeDir>, mut load_status: ResMut<crate::ConfigLoadStatus>) {
            let projects_config_path = home_dir.config_path.join(<$struct_type>::FILENAME);
            let projects_config = if fs::metadata(&projects_config_path).is_err() {
                let config = <$struct_type>::default();
                fs::write(&projects_config_path, serde_json::to_vec(&config).unwrap()).unwrap();
                config
            } else {
                serde_json::from_slice::<$struct_type>(fs::read(&projects_config_path).unwrap().as_slice()).unwrap()
            };

            commands.insert_resource(projects_config);
            load_status.$status_field = true;
        }
    };
}

use std::fs;
use bevy::prelude::*;
use serde::Serialize;
pub(crate) use default_load_config;
use editor_state::EditorLoadStatus;
use crate::HomeDir;

pub trait EditorConfig: Resource + Serialize {
    const FILENAME: &'static str;
}

#[derive(Resource, Default)]
pub(crate) struct ConfigLoadStatus {
    pub projects: bool,
}

pub(crate) fn check_config_load_status(config_load_status: Res<ConfigLoadStatus>, mut load_status: ResMut<EditorLoadStatus>) {
    if config_load_status.is_changed() {
        if config_load_status.projects {
            load_status.config_loaded = true;
        }
    }
}

pub(crate) fn save_config_on_change<T: EditorConfig>() -> impl FnMut(Res<T>, Res<HomeDir>) {
    move |res: Res<T>, home_dir: Res<HomeDir>| {
        if res.is_changed() {
            fs::write(home_dir.config_path.join(T::FILENAME), serde_json::to_vec(&*res).unwrap()).unwrap();
        }
    }
}
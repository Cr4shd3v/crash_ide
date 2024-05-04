macro_rules! default_load_config {
    ($name:ident,$path:expr,$struct_type:ty,$status_field:ident) => {
        pub(crate) fn $name (mut commands: Commands, home_dir: Res<crate::HomeDir>, mut load_status: ResMut<crate::ConfigLoadStatus>) {
            let projects_config_path = home_dir.config_path.join($path);
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

use bevy::prelude::*;
pub(crate) use default_load_config;
use editor_state::EditorLoadStatus;

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
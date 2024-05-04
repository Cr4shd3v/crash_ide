macro_rules! default_load_config {
    ($name:ident,$path:expr,$struct_type:ty) => {
        pub(crate) fn $name (mut commands: Commands, home_dir: Res<HomeDir>) {
            let projects_config_path = home_dir.config_path.join($path);
            let projects_config = if fs::metadata(&projects_config_path).is_err() {
                let config = <$struct_type>::default();
                fs::write(&projects_config_path, serde_json::to_vec(&config).unwrap()).unwrap();
                config
            } else {
                serde_json::from_slice::<$struct_type>(fs::read(&projects_config_path).unwrap().as_slice()).unwrap()
            };

            commands.insert_resource(projects_config);
        }
    };
}

pub(crate) use default_load_config;
use bevy::prelude::*;

pub(crate) fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let root_id = commands.spawn_empty().id();
    commands.insert_resource(MainUiRoot {
        root: root_id,
    });
}

/// UI Root of the main window
#[derive(Resource)]
pub struct MainUiRoot {
    pub root: Entity,
}
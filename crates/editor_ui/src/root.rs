use bevy::prelude::*;

pub(crate) fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let root_id = commands.spawn_empty().id();
    commands.insert_resource(UiRoot {
        root: root_id,
    });
}

#[derive(Resource)]
pub struct UiRoot {
    pub root: Entity,
}
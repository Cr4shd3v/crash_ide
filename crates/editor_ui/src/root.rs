use bevy::prelude::*;

pub(crate) fn setup_ui(mut commands: Commands) {
    let root_id = commands.spawn_empty().id();
    commands.insert_resource(UiRoot {
        root: root_id,
    });
}

/// UI Root of the main window
#[derive(Resource)]
pub struct UiRoot {
    pub root: Entity,
}
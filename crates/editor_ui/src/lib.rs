use bevy::prelude::*;

pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ui_cam)
        ;
    }
}

fn setup_ui_cam(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
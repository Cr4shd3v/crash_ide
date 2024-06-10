use bevy::prelude::*;

pub struct IconPlugin;

impl Plugin for IconPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load_default_icons)
        ;
    }
}

#[derive(Resource)]
#[allow(unused)]
pub struct DefaultIcons {
    pub cross: Handle<Image>,
    pub down: Handle<Image>,
    pub right: Handle<Image>,
    pub folder: Handle<Image>,
    pub unknown_file: Handle<Image>,
    pub github: Handle<Image>,
}

fn load_default_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DefaultIcons {
        cross: asset_server.load("icons/cross.png"),
        down: asset_server.load("icons/down.png"),
        right: asset_server.load("icons/right.png"),
        folder: asset_server.load("icons/folder.png"),
        unknown_file: asset_server.load("icons/unknown_file.png"),
        github: asset_server.load("icons/github.png"),
    });
}
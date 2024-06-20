use bevy::prelude::*;

pub(super) struct IconPlugin;

impl Plugin for IconPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load_default_icons)
        ;
    }
}

#[derive(Resource)]
#[allow(unused, missing_docs)]
/// Resource containing all loaded icons.
pub struct DefaultIcons {
    pub cross: Handle<Image>,
    pub down: Handle<Image>,
    pub right: Handle<Image>,
    pub folder: Handle<Image>,
    pub unknown_file: Handle<Image>,
    pub github: Handle<Image>,
    pub info: Handle<Image>,
    pub warning: Handle<Image>,
    pub error: Handle<Image>,
    pub check: Handle<Image>,
}

fn load_default_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DefaultIcons {
        cross: asset_server.load("icons/cross.png"),
        down: asset_server.load("icons/down.png"),
        right: asset_server.load("icons/right.png"),
        folder: asset_server.load("icons/folder.png"),
        unknown_file: asset_server.load("icons/unknown_file.png"),
        github: asset_server.load("icons/github.png"),
        info: asset_server.load("icons/info.png"),
        warning: asset_server.load("icons/warning.png"),
        error: asset_server.load("icons/error.png"),
        check: asset_server.load("icons/check.png"),
    });
}
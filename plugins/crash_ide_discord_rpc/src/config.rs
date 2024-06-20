use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crash_ide_config::{ConfigAppExt, EditorConfig};

pub(super) struct DiscordRpcConfigPlugin;

impl Plugin for DiscordRpcConfigPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_plugin_config::<DiscordRpcConfig>()
        ;
    }
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct DiscordRpcConfig {
    pub active: bool,
    pub show_project: bool,
    pub show_filename: bool,
}

impl EditorConfig for DiscordRpcConfig {
    const FILENAME: &'static str = "discord_rpc.json";
}
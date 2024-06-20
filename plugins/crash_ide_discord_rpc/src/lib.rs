mod client;
mod activity;
mod status;
mod config;

use bevy::prelude::*;
use crash_ide_state::EditorState;
use crate::activity::{close_project_event, handle_set_activity, set_filename, set_project_activity, set_project_activity_switch_window, trigger_rpc_update};
use crate::client::{DiscordRpcClient, finish_loading, init_client};
use crate::config::{DiscordRpcConfig, DiscordRpcConfigPlugin};
use crate::status::DiscordRpcActivity;

pub struct CrashIDEDiscordRpcPlugin;

impl Plugin for CrashIDEDiscordRpcPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DiscordRpcActivity>()
            .init_resource::<DiscordRpcClient>()
            .add_systems(OnEnter(EditorState::Loaded), init_client)
            .add_systems(Update, (
                finish_loading, set_project_activity, handle_set_activity,
                set_project_activity_switch_window, set_filename, close_project_event,
            ))
            .add_systems(Update, trigger_rpc_update.run_if(resource_changed::<DiscordRpcActivity>))
            .add_systems(Update, trigger_rpc_update.run_if(resource_changed::<DiscordRpcConfig>))
            .add_plugins(DiscordRpcConfigPlugin)
        ;
    }
}
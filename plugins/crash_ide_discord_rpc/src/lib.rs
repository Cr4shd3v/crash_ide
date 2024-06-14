mod client;
mod activity;
mod status;

use bevy::prelude::*;
use crate::activity::{close_project_event, handle_set_activity, set_filename, set_project_activity, set_project_activity_switch_window, trigger_rpc_update};
use crate::client::{finish_loading, init_client};
use crate::status::DiscordRpcActivity;

pub struct CrashIDEDiscordRpcPlugin;

impl Plugin for CrashIDEDiscordRpcPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DiscordRpcActivity>()
            .add_systems(Startup, init_client)
            .add_systems(Update, (
                finish_loading, set_project_activity, handle_set_activity,
                set_project_activity_switch_window, set_filename, close_project_event,
            ))
            .add_systems(Update, trigger_rpc_update.run_if(resource_changed::<DiscordRpcActivity>))
        ;
    }
}
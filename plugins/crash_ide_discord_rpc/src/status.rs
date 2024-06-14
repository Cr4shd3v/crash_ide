use bevy::prelude::*;

#[derive(Resource)]
pub struct DiscordRpcActivity {
    pub project: String,
    pub filename: Option<String>,
}

impl Default for DiscordRpcActivity {
    fn default() -> Self {
        Self {
            project: Self::DEFAULT_ACTIVITY.to_string(),
            filename: None,
        }
    }
}

impl DiscordRpcActivity {
    pub const DEFAULT_ACTIVITY: &'static str = "Selecting project...";
}
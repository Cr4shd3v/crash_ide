use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::load::{default_load_config, EditorConfig};

/// Global resource holding the general configuration of the IDE
#[derive(Serialize, Deserialize, Resource)]
pub struct GeneralSettings {
    /// Whether the IDE should open the last opened project or not.
    pub open_last_project_on_startup: bool,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            open_last_project_on_startup: true,
        }
    }
}

impl EditorConfig for GeneralSettings {
    const FILENAME: &'static str = "general.json";
}

default_load_config!(load_general_settings, GeneralSettings, general_settings);


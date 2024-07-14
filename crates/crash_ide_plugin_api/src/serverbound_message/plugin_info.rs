use serde::{Deserialize, Serialize};

/// Struct describing the plugin.
///
/// Must be sent as the first message to the IDE.
#[derive(Serialize, Deserialize, Debug)]
pub struct PluginInfo {
    /// Technical name of the plugin.
    ///
    /// Must be unique.
    pub technical_name: String,
    /// Name of the plugin that should be displayed
    pub display_name: String,
    /// List of all config fields
    pub config_fields: Vec<ConfigField>,
}

impl PluginInfo {
    /// Function used in initialization of the plugin.
    #[cfg(target_family = "wasm")]
    pub fn register(self) {
        crate::serverbound_message::ServerboundPluginMessage::PluginInfo(self).send();
    }
}

/// Represents a config field.
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigField {
    /// Name of the config field
    pub name: String,
    /// Label for the config field
    pub label: String,
    /// Field type
    pub field_type: ConfigFieldType,
}

impl ConfigField {
    /// Create a new config field.
    pub fn new(name: &'static str, label: &'static str, field_type: ConfigFieldType) -> Self {
        Self {
            name: name.to_string(),
            label: label.to_string(),
            field_type,
        }
    }
}

/// Field type of the config field.
#[derive(Serialize, Deserialize, Debug)]
pub enum ConfigFieldType {
    /// Will render as checkbox
    Bool,
}
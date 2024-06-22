use bincode::{Decode, Encode};
use crate::serverbound_message::ServerboundPluginMessage;

#[derive(Encode, Decode, Debug)]
pub struct PluginInfo {
    pub technical_name: String,
    pub display_name: String,
    // pub config_fields: Vec<ConfigField>,
}

impl PluginInfo {
    /// Function used in initialization of the plugin.
    pub fn register(self) {
        ServerboundPluginMessage::PluginInfo(self).send();
    }
}

#[derive(Encode, Decode, Debug)]
pub struct ConfigField {
    pub name: String,
    pub label: String,
    pub field_type: ConfigFieldType,
}

impl ConfigField {
    pub fn new(name: &'static str, label: &'static str, field_type: ConfigFieldType) -> Self {
        Self {
            name: name.to_string(),
            label: label.to_string(),
            field_type,
        }
    }
}

#[derive(Encode, Decode, Debug)]
pub enum ConfigFieldType {
    /// Will render as checkbox
    Bool,
}
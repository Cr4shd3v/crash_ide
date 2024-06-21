use bincode::{Decode, Encode};
use crate::message::PluginMessage;

#[derive(Encode, Decode, Debug)]
pub struct PluginInfo {
    pub technical_name: String,
    pub display_name: String,
}

impl PluginInfo {
    /// Function used in initialization of the plugin.
    pub fn register(self) {
        PluginMessage::PluginInfo(self).send();
    }
}
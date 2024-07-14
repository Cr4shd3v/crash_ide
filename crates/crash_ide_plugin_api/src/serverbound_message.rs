mod plugin_info;
mod println;

use serde::{Deserialize, Serialize};
pub use plugin_info::*;
pub use println::*;
use crate::UpdateConfigFields;

/// Messages sent from plugin to IDE
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerboundPluginMessage {
    /// First message to be sent by plugin
    PluginInfo(PluginInfo),
    /// Println in IDE
    PrintLn(PrintLn),
    /// WIP
    UpdateConfigFields(UpdateConfigFields),
}

impl ServerboundPluginMessage {
    /// Shortcut method for sending a [ServerboundPluginMessage] from a plugin.
    #[cfg(target_family = "wasm")]
    pub fn send(self) {
        use std::io::{stdout, Write};
        let mut out = stdout().lock();
        out.write_all(
            &serde_json::to_vec(&self).unwrap(),
        ).unwrap();
        out.flush().unwrap();
    }
}

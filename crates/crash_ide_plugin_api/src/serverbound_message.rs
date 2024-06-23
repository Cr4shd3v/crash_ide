use bincode::{Decode, Encode};

mod plugin_info;
mod println;

pub use plugin_info::*;
pub use println::*;
use crate::UpdateConfigFields;

/// Messages sent from plugin to IDE
#[derive(Encode, Decode, Debug)]
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
            &bincode::encode_to_vec(
                self,
                bincode::config::standard(),
            ).unwrap(),
        ).unwrap();
        out.flush().unwrap();
    }
}

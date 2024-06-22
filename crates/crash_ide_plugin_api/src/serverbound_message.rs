use std::io::{stdout, Write};
use bincode::{Decode, Encode};

mod plugin_info;
mod println;

pub use plugin_info::*;
pub use println::*;
use crate::UpdateConfigFields;

#[derive(Encode, Decode, Debug)]
pub enum ServerboundPluginMessage {
    PluginInfo(PluginInfo),
    PrintLn(PrintLn),
    UpdateConfigFields(UpdateConfigFields),
}

impl ServerboundPluginMessage {
    pub fn send(self) {
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

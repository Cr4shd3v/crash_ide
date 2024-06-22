use std::io::{stdout, Write};
use bincode::{Decode, Encode};

mod plugin_info;
pub use plugin_info::*;

#[derive(Encode, Decode, Debug)]
pub enum ServerboundPluginMessage {
    PluginInfo(PluginInfo),
    PrintLn(String),
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

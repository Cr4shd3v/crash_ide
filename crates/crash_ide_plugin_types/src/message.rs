use std::io::{stdout, Write};
use bincode::{Decode, Encode};
use crate::PluginInfo;

#[derive(Encode, Decode, Debug)]
pub enum PluginMessage {
    PluginInfo(PluginInfo),
    PrintLn(String),
}

impl PluginMessage {
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
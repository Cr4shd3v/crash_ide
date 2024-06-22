use bincode::{Decode, Encode};
use crate::ActiveFile;

#[derive(Encode, Decode, Debug)]
pub struct ActiveProject {
    pub opened: bool,
    pub name: String,
    pub path: String,
    pub active_file: Option<ActiveFile>,
}
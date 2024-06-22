mod active_project;
mod active_file;

use bincode::{Decode, Encode};
pub use active_project::*;
pub use active_file::*;
use crate::UpdateConfigFields;

#[derive(Encode, Decode, Debug)]
pub enum PluginboundMessage {
    Exit,
    ActiveProject(ActiveProject),
    ActiveFile(ActiveFile),
    UpdateConfigFields(UpdateConfigFields),
}
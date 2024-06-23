mod active_project;
mod active_file;

use bincode::{Decode, Encode};
pub use active_project::*;
pub use active_file::*;
use crate::UpdateConfigFields;

/// Messages sent from IDE to plugin
#[derive(Encode, Decode, Debug)]
pub enum PluginboundMessage {
    /// Plugin should exit.
    ///
    /// WIP
    Exit,
    /// Set new active project
    ActiveProject(ActiveProject),
    /// Current project is closed
    CloseProject,
    /// Set new active file
    ActiveFile(ActiveFile),
    /// WIP
    UpdateConfigFields(UpdateConfigFields),
}
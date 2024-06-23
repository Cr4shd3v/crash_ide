use bincode::{Decode, Encode};
use crate::ActiveFile;

/// Message to set a new project as active
#[derive(Encode, Decode, Debug)]
pub struct ActiveProject {
    /// If this project was opened or just switched to
    pub opened: bool,
    /// Name of the project
    pub name: String,
    /// Path of project root
    pub path: String,
    /// Active file in the project if any
    pub active_file: Option<ActiveFile>,
}
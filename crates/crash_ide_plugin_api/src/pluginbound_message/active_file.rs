use serde::{Deserialize, Serialize};

/// Message to set the currently active file
#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveFile {
    /// Filename
    pub filename: String,
    /// Full path to file
    pub path: String,
}
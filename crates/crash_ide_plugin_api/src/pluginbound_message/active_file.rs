use bincode::{Decode, Encode};

/// Message to set the currently active file
#[derive(Encode, Decode, Debug)]
pub struct ActiveFile {
    /// Filename
    pub filename: String,
    /// Full path to file
    pub path: String,
}
use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug)]
pub struct ActiveFile {
    pub filename: String,
    pub path: String,
}
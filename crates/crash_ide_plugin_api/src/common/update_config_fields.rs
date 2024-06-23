use bincode::{Decode, Encode};

/// Represents a config field value
#[derive(Encode, Decode, Debug)]
pub enum ConfigFieldVal {
    /// Bool value
    Bool(bool),
}

/// Message to update fields on plugin or IDE
#[derive(Encode, Decode, Debug)]
pub struct UpdateConfigFields {
    /// All field updates, see [UpdateConfigField]
    pub fields: Vec<UpdateConfigField>,
}

/// Contains a single field update.
#[derive(Encode, Decode, Debug)]
pub struct UpdateConfigField {
    /// Name of the field, NOT the label
    pub name: String,
    /// New value
    pub value: ConfigFieldVal,
}
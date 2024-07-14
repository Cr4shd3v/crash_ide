use serde::{Deserialize, Serialize};

/// Represents a config field value
#[derive(Serialize, Deserialize, Debug)]
pub enum ConfigFieldVal {
    /// Bool value
    Bool(bool),
}

/// Message to update fields on plugin or IDE
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateConfigFields {
    /// All field updates, see [UpdateConfigField]
    pub fields: Vec<UpdateConfigField>,
}

/// Contains a single field update.
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateConfigField {
    /// Name of the field, NOT the label
    pub name: String,
    /// New value
    pub value: ConfigFieldVal,
}
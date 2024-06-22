use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug)]
pub enum ConfigFieldVal {
    Bool(bool),
}

#[derive(Encode, Decode, Debug)]
pub struct UpdateConfigFields {
    pub fields: Vec<UpdateConfigField>,
}

#[derive(Encode, Decode, Debug)]
pub struct UpdateConfigField {
    pub name: String,
    pub value: ConfigFieldVal,
}
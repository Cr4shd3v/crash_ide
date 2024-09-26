use bevy::prelude::*;

/// Event triggered on an entity when [crate::TextInputSettings] `submittable` is true.
#[derive(Event)]
pub struct TextInputSubmitted {
    /// Submitted content
    pub content: String,
}
use bevy::prelude::*;

#[derive(Component)]
pub struct CodeViewLine {
    /// Line index, this is 0-indexed
    pub line_index: usize,
}
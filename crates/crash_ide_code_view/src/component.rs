use bevy::prelude::*;
use bevy::utils::HashMap;

/// Component used in every line. Stores the line index.
#[derive(Component)]
pub struct CodeViewLine {
    /// Line index, this is 0-indexed
    pub line_index: usize,
}

/// Marker component for the active line
#[derive(Component)]
pub struct HighlightedLine;

/// Component holding information of the cursor entity of a code view.
#[derive(Component)]
pub struct CursorEntityRef(pub Entity);

/// Component marking a code view as focused
#[derive(Component)]
pub struct CodeViewFocused;

/// Marker component for the actual code view
#[derive(Component)]
pub struct CodeViewContainer;

/// Contains references to all lines
#[derive(Component, Default)]
pub struct CodeViewLineRegistry {
    /// Lines map, indexed by the line count (0-indexed)
    pub lines: HashMap<usize, CodeViewLineRegistryEntry>,
    /// Currently active line
    pub active: usize,
}

/// Entry for [CodeViewLineRegistry]
pub struct CodeViewLineRegistryEntry {
    /// Entity of the line count
    pub line_count: Entity,
    /// Entity of the lines content
    pub line_content: Entity,
}
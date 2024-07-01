use bevy::prelude::*;

/// Component used in every line. Stores the line index.
#[derive(Component)]
pub struct CodeViewLine {
    /// Line index, this is 0-indexed
    pub line_index: usize,
}

/// Component holding information of the cursor entity of a code view.
#[derive(Component)]
pub struct CursorEntityRef(pub Entity);

/// Component marking a code view as focused
#[derive(Component)]
pub struct CodeViewFocused;

/// Marker component for the actual code view
#[derive(Component)]
pub struct CodeViewContainer;
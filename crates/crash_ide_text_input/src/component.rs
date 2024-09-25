use bevy::prelude::*;

/// Component used in every line.
#[derive(Component)]
pub struct TextInputLine;

/// Marker component for the active line
#[derive(Component)]
pub struct HighlightedLine;

/// Component holding information of the cursor entity of a code view.
#[derive(Component)]
pub struct CursorEntityRef(pub Entity);

/// Component marking a code view as focused
#[derive(Component)]
pub struct TextInputFocused;

/// Marker component for the actual code view
#[derive(Component)]
pub struct TextInputContainer;

/// Contains references to all lines
#[derive(Component)]
pub struct TextInputLineContainer {
    /// Container for all line contents
    pub line_content_container: Entity,
}
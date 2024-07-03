use bevy::prelude::*;

/// Component used in every line.
#[derive(Component)]
pub struct CodeViewLine;

/// Marker component for the active line
#[derive(Component)]
pub struct HighlightedLine;

/// Marker component for the active line count
#[derive(Component)]
pub struct HighlightedLineCount;

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
#[derive(Component)]
pub struct CodeViewLineContainer {
    /// Container of all line counts
    pub line_count_container: Entity,
    /// Container for all line contents
    pub line_content_container: Entity,
}
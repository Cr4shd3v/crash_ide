use bevy::prelude::*;

/// Spawns a code view.
#[derive(Bundle, Default)]
pub struct CodeViewBundle {
    /// Basic style of the view
    pub node_bundle: NodeBundle,
    /// Specific code view styling
    pub code_view_style: CodeViewStyle,
    /// Used for focus of text etc.
    pub interaction: Interaction,
}

/// Defines styles of the code view.
#[derive(Component)]
pub struct CodeViewStyle {
    /// Regular font
    pub regular_font: Handle<Font>,
    /// Bold font
    pub bold_font: Handle<Font>,
    /// Italic font
    pub italic_font: Handle<Font>,
    /// Bold Italic font
    pub bold_italic_font: Handle<Font>,
    /// Font size of the code view
    pub font_size: f32,
}

impl Default for CodeViewStyle {
    fn default() -> Self {
        Self {
            regular_font: Handle::default(),
            bold_font: Handle::default(),
            italic_font: Handle::default(),
            bold_italic_font: Handle::default(),
            font_size: 17.0,
        }
    }
}
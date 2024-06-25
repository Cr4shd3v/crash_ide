use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct CodeViewBundle {
    pub node_bundle: NodeBundle,
    pub code_view_style: CodeViewStyle,
    pub interaction: Interaction,
}

#[derive(Component)]
pub struct CodeViewStyle {
    pub regular_font: Handle<Font>,
    pub bold_font: Handle<Font>,
    pub italic_font: Handle<Font>,
    pub bold_italic_font: Handle<Font>,
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
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
    /// Marker struct
    pub code_view: CodeView,
    /// Content of the code view
    pub content: CodeViewContent,
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

impl CodeViewStyle {
    /// Returns the appropriate font for a token
    pub fn get_font_for_token(&self, token: &CodeViewToken) -> Handle<Font> {
        if token.italic {
            if token.bold {
                self.bold_italic_font.clone()
            } else {
                self.italic_font.clone()
            }
        } else {
            if token.bold {
                self.bold_font.clone()
            } else {
                self.regular_font.clone()
            }
        }
    }
}

/// Marker struct for a code view.
#[derive(Component, Default)]
pub struct CodeView;

/// Content of a code view
#[derive(Component, Default, Debug)]
pub struct CodeViewContent {
    /// All lines
    pub lines: Vec<Vec<CodeViewToken>>,
}

/// Token describing a part of a line
#[derive(Default, Debug)]
pub struct CodeViewToken {
    /// Content of this token
    pub content: String,
    /// Text should be bold
    pub bold: bool,
    /// Text should be italic
    pub italic: bool,
}

impl CodeViewContent {
    /// Constructs a [CodeViewContent] from a string without any styling.
    pub fn from_string(string: String) -> Self {
        Self {
            lines: string.split("\n").map(|v| vec![CodeViewToken {
                content: v.to_string(),
                ..default()
            }]).collect(),
        }
    }
}
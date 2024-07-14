use bevy::prelude::*;
use crate::{CodeViewContent, CodeViewToken};

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
    /// Cursor position
    pub cursor_pos: CodeViewCursorPosition,
    /// Timer for cursor blinking
    pub cursor_timer: CodeViewCursorTimer,
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
            font_size: 18.0,
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

/// Information about the cursor in the current code view
#[derive(Component, Default)]
pub struct CodeViewCursorPosition {
    /// Current cursor position.
    ///
    /// X = Column, Y = Line
    pub cursor_pos: UVec2,
}

/// Contains the timer for blinking cursor
#[derive(Component)]
pub struct CodeViewCursorTimer {
    /// Timer for cursor blinking
    pub timer: Timer,
    /// Should the timer reset?
    pub reset: bool,
}

impl Default for CodeViewCursorTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            reset: false,
        }
    }
}
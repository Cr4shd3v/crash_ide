use bevy::prelude::*;
use crate::TextInputContent;

/// Spawns a code view.
#[derive(Bundle, Default)]
pub struct TextInputBundle {
    /// Basic style of the input
    pub node_bundle: NodeBundle,
    /// Specific text input styling
    pub text_style: TextInputStyle,
    /// Used for focus of text etc.
    pub interaction: Interaction,
    /// Marker struct
    pub text_input: TextInput,
    /// Content of the text input
    pub content: TextInputContent,
    /// Cursor position
    pub cursor_pos: TextInputCursorPosition,
    /// Timer for cursor blinking
    pub cursor_timer: TextInputCursorTimer,
    /// Settings for the text input
    pub settings: TextInputSettings,
}

/// Defines styles of the text input.
#[derive(Component)]
pub struct TextInputStyle {
    /// Regular font
    pub font: Handle<Font>,
    /// Font size of the text input
    pub font_size: f32,
}

impl Default for TextInputStyle {
    fn default() -> Self {
        Self {
            font: Handle::default(),
            font_size: 18.0,
        }
    }
}

/// Marker struct for a text input.
#[derive(Component, Default)]
pub struct TextInput;

/// Information about the cursor in the current text input
#[derive(Component, Default)]
pub struct TextInputCursorPosition {
    /// Current cursor position.
    ///
    /// X = Column, Y = Line
    pub cursor_pos: UVec2,
    /// Where the cursor should initialize
    pub init: TextInputInitCursorPos,
}

impl TextInputCursorPosition {
    /// Resets the cursor position to the default
    pub fn reset_to_init(&mut self, content: &TextInputContent) {
        match self.init {
            TextInputInitCursorPos::Start => {
                self.cursor_pos = UVec2::ZERO;
            }
            TextInputInitCursorPos::End => {
                let line = (content.lines.len() as u32).checked_sub(1).unwrap_or(0);
                self.cursor_pos.y = line;
                self.cursor_pos.x = content.get_line_length(line as usize).unwrap_or(0) as u32;
            }
            TextInputInitCursorPos::At(pos) => {
                self.cursor_pos = pos.clone();
            }
        }
    }
}

/// Contains the timer for blinking cursor
#[derive(Component)]
pub struct TextInputCursorTimer {
    /// Timer for cursor blinking
    pub timer: Timer,
    /// Should the timer reset?
    pub reset: bool,
}

impl Default for TextInputCursorTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            reset: false,
        }
    }
}

/// Contains the settings for a text input
#[derive(Component, Default)]
pub struct TextInputSettings {
    /// Whether this text input should be multiline or not
    pub multiline: bool,
    /// Whether this text input can be submitted or not
    pub submittable: bool,
}

/// Determines where the cursor will be initialized
#[derive(Default)]
pub enum TextInputInitCursorPos {
    /// Cursor starts at the beginning
    Start,
    /// Cursor starts at the end (default)
    #[default]
    End,
    /// Cursor starts at a specific position
    At(UVec2),
}
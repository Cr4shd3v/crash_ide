use bevy::prelude::*;

/// Struct holding all default colors.
pub struct DefaultColors;

impl DefaultColors {
    /// Background color used in the main editor view in the left menu.
    pub const DEFAULT_BACKGROUND: Color = Color::srgb(0.129, 0.145, 0.169);
    /// Background color used in the main editor view in the file view.
    pub const MAIN_VIEW_BACKGROUND: Color = Color::srgb(0.157, 0.173, 0.204);
    /// Every primary button has this color.
    pub const PRIMARY_BUTTON: Color = Color::srgb(0.341, 0.541, 0.949);
    /// Color used in many borders
    pub const GRAY: Color = Color::srgb(0.5019608, 0.5019608, 0.5019608);
}
use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;

pub(super) struct DefaultFontsPlugin;

impl Plugin for DefaultFontsPlugin {
    fn build(&self, app: &mut App) {
        load_internal_binary_asset!(
            app,
            DefaultFonts::ROBOTO_REGULAR,
            "../assets/fonts/Roboto/Roboto-Regular.ttf",
            |bytes: &[u8], _path: String| {
                Font::try_from_bytes(bytes.to_vec()).unwrap()
            }
        );
        load_internal_binary_asset!(
            app,
            DefaultFonts::ROBOTO_BOLD,
            "../assets/fonts/Roboto/Roboto-Bold.ttf",
            |bytes: &[u8], _path: String| {
                Font::try_from_bytes(bytes.to_vec()).unwrap()
            }
        );
        load_internal_binary_asset!(
            app,
            DefaultFonts::ROBOTO_ITALIC,
            "../assets/fonts/Roboto/Roboto-Italic.ttf",
            |bytes: &[u8], _path: String| {
                Font::try_from_bytes(bytes.to_vec()).unwrap()
            }
        );
        load_internal_binary_asset!(
            app,
            DefaultFonts::ROBOTO_BOLD_ITALIC,
            "../assets/fonts/Roboto/Roboto-BoldItalic.ttf",
            |bytes: &[u8], _path: String| {
                Font::try_from_bytes(bytes.to_vec()).unwrap()
            }
        );
        load_internal_binary_asset!(
            app,
            DefaultFonts::JETBRAINS_MONO_REGULAR,
            "../assets/fonts/JetbrainsMono/fonts/ttf/JetBrainsMono-Regular.ttf",
            |bytes: &[u8], _path: String| {
                Font::try_from_bytes(bytes.to_vec()).unwrap()
            }
        );
        load_internal_binary_asset!(
            app,
            DefaultFonts::JETBRAINS_MONO_BOLD,
            "../assets/fonts/JetbrainsMono/fonts/ttf/JetBrainsMono-Bold.ttf",
            |bytes: &[u8], _path: String| {
                Font::try_from_bytes(bytes.to_vec()).unwrap()
            }
        );
        load_internal_binary_asset!(
            app,
            DefaultFonts::JETBRAINS_MONO_ITALIC,
            "../assets/fonts/JetbrainsMono/fonts/ttf/JetBrainsMono-Italic.ttf",
            |bytes: &[u8], _path: String| {
                Font::try_from_bytes(bytes.to_vec()).unwrap()
            }
        );
        load_internal_binary_asset!(
            app,
            DefaultFonts::JETBRAINS_MONO_BOLD_ITALIC,
            "../assets/fonts/JetbrainsMono/fonts/ttf/JetBrainsMono-BoldItalic.ttf",
            |bytes: &[u8], _path: String| {
                Font::try_from_bytes(bytes.to_vec()).unwrap()
            }
        );
    }
}

/// Contains all default fonts.
pub struct DefaultFonts;

#[allow(missing_docs)]
impl DefaultFonts {
    pub const ROBOTO_REGULAR: Handle<Font> = Handle::weak_from_u128(0x493cdff68af949cd9d4486d1b592ba4a);
    pub const ROBOTO_BOLD: Handle<Font> = Handle::weak_from_u128(0x208f6a3079ef4ef284e2616b8da36418);
    pub const ROBOTO_ITALIC: Handle<Font> = Handle::weak_from_u128(0x23f216342a0345fb99f7c98fa8242691);
    pub const ROBOTO_BOLD_ITALIC: Handle<Font> = Handle::weak_from_u128(0x50e8e832368446faae494f79f58310ad);
    pub const JETBRAINS_MONO_REGULAR: Handle<Font> = Handle::weak_from_u128(0x1ab46244a2f24433a871c100506969de);
    pub const JETBRAINS_MONO_BOLD: Handle<Font> = Handle::weak_from_u128(0x222c6bcc31b54d508bb762d6eb219afe);
    pub const JETBRAINS_MONO_ITALIC: Handle<Font> = Handle::weak_from_u128(0x3798b873d3724060abff675d37637e42);
    pub const JETBRAINS_MONO_BOLD_ITALIC: Handle<Font> = Handle::weak_from_u128(0x7e890f909e3b427aba9d696a7d9e21d1);
}
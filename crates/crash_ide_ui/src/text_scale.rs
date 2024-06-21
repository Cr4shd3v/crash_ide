use bevy::prelude::*;
use crash_ide_widget::{TextInputInner, TextInputTextStyle};
use crate::window::DefaultWindowResolution;

pub(super) fn scale_text(
    mut query: Query<&mut Text, (Added<Text>, Without<TextInputInner>)>,
    default_window_resolution: Res<DefaultWindowResolution>,
) {
    for mut text in query.iter_mut() {
        for section in text.sections.iter_mut() {
            section.style.font_size *= default_window_resolution.0.scale_factor();
        }
    }
}

pub(super) fn scale_text_input(
    mut query: Query<&mut TextInputTextStyle, Added<TextInputTextStyle>>,
    default_window_resolution: Res<DefaultWindowResolution>,
) {
    for mut text_style in query.iter_mut() {
        text_style.0.font_size *= default_window_resolution.0.scale_factor();
    }
}
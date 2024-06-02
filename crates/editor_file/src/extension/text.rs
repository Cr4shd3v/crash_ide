//! This module contains the implementation for .txt files

use std::fs;
use bevy::prelude::*;
use editor_assets::DefaultFonts;
use editor_widget::{TextInputBundle, TextInputSettings, TextInputTextStyle, TextInputValue};
use crate::{default_file_handler_impl, FileViewInstance, OpenFileEvent};
use crate::handler::FileHandlerManager;

pub(super) struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<OpenFileEvent<TextFile>>()
            .add_systems(Startup, register_handler)
            .add_systems(Update, (spawn_file_view, save_edited_content))
        ;
    }
}

/// [FileHandler](crate::FileHandler) for .txt files
pub struct TextFile;

use crate as editor_file;
default_file_handler_impl!(TextFile, ["txt"]);

fn register_handler(mut handler_manager: ResMut<FileHandlerManager>) {
    handler_manager.register_handler::<TextFile>();
}

pub(super) fn spawn_file_view(
    mut commands: Commands,
    mut event_reader: EventReader<OpenFileEvent<TextFile>>,
) {
    for event in event_reader.read() {
        let content = fs::read_to_string(&event.event_data.path).unwrap();

        commands.entity(event.event_data.view_entity).despawn_descendants().with_children(|parent| {
            parent.spawn((TextInputBundle {
                text_input_value: TextInputValue(content),
                text_input_text_style: TextInputTextStyle(TextStyle {
                    font: DefaultFonts::JETBRAINS_MONO_REGULAR,
                    font_size: 18.0,
                    ..default()
                }),
                text_input_settings: TextInputSettings {
                    with_border: false,
                    multiline: true,
                },
                ..default()
            }, NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            }, FileViewInstance {
                path: event.event_data.path.clone(),
            }));
        });
    }
}

pub(super) fn save_edited_content(
    query: Query<(&TextInputValue, &FileViewInstance), Changed<TextInputValue>>
) {
    for (input_value, view_instance) in query.iter() {
        fs::write(&view_instance.path, &input_value.0).unwrap();
    }
}
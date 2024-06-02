//! This module contains the implementation for .txt files

use std::fs;
use bevy::prelude::*;
use editor_assets::DefaultFonts;
use editor_widget::{TextInputBundle, TextInputSettings, TextInputTextStyle, TextInputValue};
use crate::{default_file_handler_impl, FileHandlerAppExtension, FileViewInstance, OpenFileEvent};

pub(super) struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_file_handler::<TextFile>()
            .add_systems(Update, (spawn_file_view, save_edited_content))
        ;
    }
}

/// [FileHandler](crate::FileHandler) for .txt files
pub struct TextFile;

use crate as editor_file;
default_file_handler_impl!(TextFile, ["txt"]);

fn spawn_file_view(
    mut commands: Commands,
    mut event_reader: EventReader<OpenFileEvent<TextFile>>,
) {
    for event in event_reader.read() {
        // Since TextFile is the default handler, we have to ensure that we can handle non-utf-8 files via OS default
        let Ok(content) = fs::read_to_string(&event.event_data.path) else {
            open::that_detached(&event.event_data.path).unwrap();
            continue;
        };

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

fn save_edited_content(
    query: Query<(&TextInputValue, &FileViewInstance), Changed<TextInputValue>>
) {
    for (input_value, view_instance) in query.iter() {
        fs::write(&view_instance.path, &input_value.0).unwrap();
    }
}
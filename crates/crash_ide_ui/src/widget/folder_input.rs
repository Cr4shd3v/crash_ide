//! Contains the implementation of a folder text input.

use std::path::PathBuf;
use bevy::prelude::*;
use crash_ide_assets::DefaultIcons;
use crash_ide_file_picker::{DirectoryPicked, DirectoryPicker};
use crash_ide_widget::{TextInputCursorPos, TextInputValue};

pub(super) struct FolderInputPlugin;

impl Plugin for FolderInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_button, folder_input_button, folder_picked))
        ;
    }
}

/// Marks a text input as directory input and adds a button to it.
///
/// Requires 4% width.
#[derive(Component)]
pub struct FolderInput;

#[derive(Component)]
struct FolderInputButton;

fn spawn_button(
    mut commands: Commands,
    query: Query<(Entity, &Node), (Added<TextInputCursorPos>, With<FolderInput>)>,
    default_icons: Res<DefaultIcons>,
) {
    for (entity, node) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn((ImageBundle {
                image: UiImage {
                    texture: default_icons.folder.clone(),
                    ..default()
                },
                style: Style {
                    max_width: Val::Percent(4.0),
                    height: Val::Px(node.size().y),
                    ..default()
                },
                ..default()
            }, FolderInputButton, Interaction::None, Button));
        });
    }
}

fn folder_input_button(
    mut commands: Commands,
    query: Query<(&Interaction, &Parent), (Changed<Interaction>, With<FolderInputButton>)>,
    text_input_query: Query<&TextInputValue>,
) {
    for (interaction, parent) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let parent_entity = parent.get();
        let current_value = text_input_query.get(parent_entity).unwrap();

        commands.entity(parent_entity).insert(DirectoryPicker {
            start_directory: Some(PathBuf::from(&current_value.0)),
            title: "Select project path".to_string(),
        });
    }
}

fn folder_picked(
    mut input_query: Query<(&DirectoryPicked, &mut TextInputValue), (With<FolderInput>, Added<DirectoryPicked>)>,
) {
    for (directory, mut input) in input_query.iter_mut() {
        input.0 = directory.0.path().to_str().unwrap().to_string();
    }
}


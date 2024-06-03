//! Contains the implementation of a folder text input.

use bevy::prelude::*;
use bevy_file_dialog::{DialogDirectoryPicked, FileDialogExt};
use editor_assets::DefaultIcons;
use editor_widget::{TextInputCursorPos, TextInputValue};

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

        let current_value = text_input_query.get(parent.get()).unwrap();

        commands.dialog()
            .set_title("Select project path")
            .set_directory(&current_value.0)
            .pick_directory_path::<FolderInput>();
    }
}

fn folder_picked(
    mut event_reader: EventReader<DialogDirectoryPicked<FolderInput>>,
    mut input_query: Query<&mut TextInputValue, With<FolderInput>>,
) {
    for event in event_reader.read() {
        input_query.single_mut().0 = event.path.to_str().unwrap().to_string();
    }
}


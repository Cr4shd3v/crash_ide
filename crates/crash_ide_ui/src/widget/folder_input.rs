//! Contains the implementation of a folder text input.

use std::path::PathBuf;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use crash_ide_assets::DefaultIcons;
use crash_ide_file_picker::{DirectoryPicked, DirectoryPicker};
use crash_ide_text_input::{CodeViewToken, TextInputContent, TextInputContentLine, TextInputCursorPosition, TextInputStyle};
use crate::trigger::Clicked;

pub(super) struct FolderInputPlugin;

impl Plugin for FolderInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_button)
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
    query: Query<(Entity, &TextInputStyle), (Added<TextInputCursorPosition>, With<FolderInput>)>,
    default_icons: Res<DefaultIcons>,
) {
    for (entity, style) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn((ImageBundle {
                image: UiImage {
                    texture: default_icons.folder.clone(),
                    ..default()
                },
                style: Style {
                    max_width: Val::Percent(4.0),
                    height: Val::Px(style.font_size + 5.0),
                    align_self: AlignSelf::Center,
                    ..default()
                },
                z_index: ZIndex::Local(1),
                focus_policy: FocusPolicy::Block,
                ..default()
            }, FolderInputButton, Interaction::None, Button)).observe(folder_input_button);
        });
    }
}

fn folder_input_button(
    trigger: Trigger<Clicked>,
    mut commands: Commands,
    parent_query: Query<&Parent>,
    text_input_query: Query<&TextInputContent>,
) {
    let parent_entity = parent_query.get(trigger.entity()).unwrap().get();
    let current_value = text_input_query.get(parent_entity).unwrap();

    commands.entity(parent_entity).insert(DirectoryPicker {
        start_directory: Some(PathBuf::from(&current_value.to_string())),
        title: "Select project path".to_string(),
    }).observe(folder_picked);
}

fn folder_picked(
    trigger: Trigger<DirectoryPicked>,
    mut input_query: Query<&mut TextInputContent>,
) {
    let mut input  = input_query.get_mut(trigger.entity()).unwrap();
    input.lines.clear();
    input.lines.push(TextInputContentLine {
        tokens: vec![CodeViewToken {
            content: trigger.event().0.path().to_str().unwrap().to_string(),
            ..default()
        }]
    });
}


use std::fs;
use std::path::Path;

use bevy::prelude::*;
use crash_ide_util::FindComponentInParents;

use crash_ide_widget::{ActiveWindow, FocusNode, RightClicked, TextInputSubmitted, TextInputValue};

use crate::editor::editor_left_menu::{FileDisplay, FilePath, ProjectRoot};
use crate::editor::editor_left_menu::filesystem_menu::filename_dialog::{FilenameDialog, FilenameDialogConfirmButton};
use crate::editor::editor_left_menu::filesystem_menu::SelfFileRow;
use crate::widget::context_menu::{ContextMenu, ContextMenuRow};
use crate::window::AllWindows;

pub(super) struct FileContextMenuPlugin;

impl Plugin for FileContextMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                file_context_menu, handle_file_delete, handle_create_file,
                handle_create_file_submit, handle_create_file_filename_submit,
            ))
        ;
    }
}

#[derive(Component, Clone)]
struct FileContextRef(Entity);

#[derive(Component)]
struct DeleteFileButton;

#[derive(Component)]
struct CreateFileButton;

fn file_context_menu(
    mut commands: Commands,
    query: Query<&Parent, (Added<RightClicked>, With<SelfFileRow>)>,
    file_display_query: Query<(Entity, &FileDisplay, Option<&ProjectRoot>)>,
    window_query: Query<(Entity, &Window), With<ActiveWindow>>,
    all_windows: Res<AllWindows>,
) {
    for parent in query.iter() {
        let entity = parent.get();
        let Ok((file_display_entity, file_display, root)) = file_display_query.get(entity) else {
            continue;
        };

        let Ok((window_entity, window)) = window_query.get_single() else {
            continue;
        };

        let Some(cursor_pos) = window.cursor_position() else {
            continue;
        };

        commands.entity(all_windows.get(&window_entity).ui_root).with_children(|parent| {
            parent.spawn((
                ContextMenu::new_at_cursor(cursor_pos),
                FileContextRef(file_display_entity),
            )).with_children(|parent| {
                if !file_display.is_file {
                    ContextMenuRow::new(parent, "Create new file", CreateFileButton, None);
                }

                if root.is_none() {
                    ContextMenuRow::new(parent, "Delete", DeleteFileButton, None);
                }
            });
        });
    }
}

fn handle_file_delete(
    mut commands: Commands,
    query: Query<(&Parent, &Interaction), (With<DeleteFileButton>, Changed<Interaction>)>,
    context_ref: Query<&FileContextRef>,
    file_display_query: Query<&FileDisplay>,
    file_path: FilePath,
) {
    for (parent, interaction) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let context_menu_entity = parent.get();

        commands.entity(context_menu_entity).despawn_recursive();

        let file_context = context_ref.get(context_menu_entity).unwrap();
        let Ok(file_display) = file_display_query.get(file_context.0) else {
            continue;
        };

        let path = file_path.get_full_path(file_context.0);

        let fs_result = if file_display.is_file {
            fs::remove_file(path)
        } else {
            fs::remove_dir_all(path)
        };

        if let Err(e) = fs_result {
            // TODO: error notification
            println!("Could not delete: {}", e);
            continue;
        }

        commands.entity(file_context.0).despawn_recursive();
    }
}

#[derive(Component)]
struct CreateFileConfirmButton;

#[derive(Component)]
struct CreateFileFilenameInput;

fn handle_create_file(
    mut commands: Commands,
    query: Query<(&Parent, &Interaction), (With<CreateFileButton>, Changed<Interaction>)>,
    context_ref: Query<&FileContextRef>,
    window_query: Query<(Entity, &Window), With<ActiveWindow>>,
    all_windows: Res<AllWindows>,
) {
    for (parent, interaction) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let context_menu_entity = parent.get();

        commands.entity(context_menu_entity).despawn_recursive();

        let file_context = context_ref.get(context_menu_entity).unwrap();
        let (entity, window) = window_query.single();

        commands.entity(all_windows.get(&entity).ui_root).with_children(|parent| {
            FilenameDialog::new(
                parent,
                window,
                (file_context.clone(), CreateFileConfirmButton),
                (file_context.clone(), CreateFileFilenameInput),
                "Create File",
                "Create"
            );
        });
    }
}

fn handle_create_file_submit(
    mut commands: Commands,
    query: Query<(Entity, &Interaction, &FilenameDialogConfirmButton, &FileContextRef), (With<CreateFileConfirmButton>, Changed<Interaction>)>,
    find_context_menu: FindComponentInParents<FocusNode>,
    text_query: Query<&TextInputValue>,
    file_path: FilePath,
) {
    for (entity, interaction, button, file_context) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        commands.entity(find_context_menu.find_entity(entity).unwrap()).despawn_recursive();

        let base_path = file_path.get_full_path(file_context.0);
        let text_content = text_query.get(button.input_id).unwrap().0.clone();
        let full_path = format!("{}/{}", base_path, text_content);

        create_file(full_path);
    }
}

fn handle_create_file_filename_submit(
    mut commands: Commands,
    query: Query<(Entity, &TextInputSubmitted, &FileContextRef), (With<CreateFileFilenameInput>, Changed<TextInputSubmitted>)>,
    find_context_menu: FindComponentInParents<FocusNode>,
    file_path: FilePath,
) {
    for (entity, text_submitted, file_context) in query.iter() {
        let Some(text) = text_submitted.0.as_ref() else {
            continue;
        };

        commands.entity(find_context_menu.find_entity(entity).unwrap()).despawn_recursive();
        let base_path = file_path.get_full_path(file_context.0);
        let full_path = format!("{}/{}", base_path, text);

        create_file(full_path);
    }
}

fn create_file(full_path: impl AsRef<Path>) {
    if let Err(e) = fs::write(full_path, "") {
        // TODO: error notification
        println!("Could not create file: {}", e);
    }
}
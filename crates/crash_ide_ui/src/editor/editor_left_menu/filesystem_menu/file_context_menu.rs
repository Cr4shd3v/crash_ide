use std::fs;
use std::path::Path;

use bevy::prelude::*;

use crash_ide_assets::DefaultIcons;
use crash_ide_util::FindComponentInParents;
use crash_ide_widget::{ActiveWindow, RightClicked, TextInputSubmitted, TextInputValue};

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
                create_file_submenu, handle_create_file_submit, handle_create_file_filename_submit,
            ))
        ;
    }
}

#[derive(Component, Clone)]
struct FileContextRef(Entity);

fn file_context_menu(
    mut commands: Commands,
    query: Query<&Parent, (Added<RightClicked>, With<SelfFileRow>)>,
    file_display_query: Query<(Entity, &FileDisplay, Option<&ProjectRoot>)>,
    window_query: Query<(Entity, &Window), With<ActiveWindow>>,
    all_windows: Res<AllWindows>,
    icons: Res<DefaultIcons>,
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
                    ContextMenuRow::new(parent, "Create", CreateButton::default(), None, Some(icons.right.clone()));
                }

                if root.is_none() {
                    ContextMenuRow::new(parent, "Delete", DeleteFileButton, None, None);
                }
            });
        });
    }
}

#[derive(Component)]
struct DeleteFileButton;

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

#[derive(Component)]
struct CreateContext {
    is_file: bool,
}

#[derive(Component, Default)]
struct CreateButton {
    menu_id: Option<Entity>,
}

#[derive(Component)]
enum CreateFileButton {
    File,
    Folder,
}

fn create_file_submenu(
    mut commands: Commands,
    mut query: Query<(Entity, &Interaction, &Node, &mut CreateButton), Changed<Interaction>>,
    icons: Res<DefaultIcons>,
    interaction_query: Query<&Interaction>,
) {
    for (entity, interaction, node, mut button) in query.iter_mut() {
        if let Some(menu_id) = button.menu_id {
            if let Some(command) = commands.get_entity(menu_id) {
                if *interaction_query.get(menu_id).unwrap() == Interaction::None {
                    command.despawn_recursive();
                }
            }
        }

        match interaction {
            Interaction::Pressed | Interaction::Hovered => {
                let size = node.size();
                commands.entity(entity).with_children(|parent| {
                    button.menu_id = Some(parent.spawn(
                        ContextMenu::new_top(0.0, Val::Px(size.x)),
                    ).with_children(|parent| {
                        ContextMenuRow::new(parent, "Folder", CreateFileButton::Folder, Some(icons.folder.clone()), None);
                        ContextMenuRow::new(parent, "File", CreateFileButton::File, Some(icons.unknown_file.clone()), None);
                    }).id());
                });
            },
            _ => {},
        }
    }
}

fn handle_create_file(
    mut commands: Commands,
    query: Query<(Entity, &Interaction, &CreateFileButton), Changed<Interaction>>,
    context_ref: Query<&FileContextRef>,
    window_query: Query<(Entity, &Window), With<ActiveWindow>>,
    all_windows: Res<AllWindows>,
    find_file_ref: FindComponentInParents<FileContextRef>,
) {
    for (entity, interaction, is_file) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let context_menu_entity = find_file_ref.find_entity(entity).unwrap();

        commands.entity(context_menu_entity).despawn_recursive();

        let file_context = context_ref.get(context_menu_entity).unwrap();
        let (entity, window) = window_query.single();
        let is_file = matches!(is_file, CreateFileButton::File);

        commands.entity(all_windows.get(&entity).ui_root).with_children(|parent| {
            FilenameDialog::new(
                parent,
                window,
                (file_context.clone(), CreateContext { is_file }),
                CreateFileConfirmButton,
                CreateFileFilenameInput,
                if is_file { "Create File" } else { "Create Folder" },
                "Create"
            );
        });
    }
}

fn handle_create_file_submit(
    mut commands: Commands,
    query: Query<(Entity, &Interaction, &FilenameDialogConfirmButton), (With<CreateFileConfirmButton>, Changed<Interaction>)>,
    find_context_menu: FindComponentInParents<ContextMenu>,
    text_query: Query<&TextInputValue>,
    context_query: Query<(&FileContextRef, &CreateContext)>,
    file_path: FilePath,
) {
    for (entity, interaction, button) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let context_menu = find_context_menu.find_entity(entity).unwrap();
        commands.entity(find_context_menu.find_entity(entity).unwrap()).despawn_recursive();
        let (file_context, create_context) = context_query.get(context_menu).unwrap();

        let base_path = file_path.get_full_path(file_context.0);
        let text_content = text_query.get(button.input_id).unwrap().0.clone();
        let full_path = format!("{}/{}", base_path, text_content);

        create_file(full_path, create_context);
    }
}

fn handle_create_file_filename_submit(
    mut commands: Commands,
    query: Query<(Entity, &TextInputSubmitted), (With<CreateFileFilenameInput>, Changed<TextInputSubmitted>)>,
    find_context_menu: FindComponentInParents<ContextMenu>,
    context_query: Query<(&FileContextRef, &CreateContext)>,
    file_path: FilePath,
) {
    for (entity, text_submitted) in query.iter() {
        let Some(text) = text_submitted.0.as_ref() else {
            continue;
        };

        let context_menu = find_context_menu.find_entity(entity).unwrap();
        commands.entity(find_context_menu.find_entity(entity).unwrap()).despawn_recursive();
        let (file_context, create_context) = context_query.get(context_menu).unwrap();

        let base_path = file_path.get_full_path(file_context.0);
        let full_path = format!("{}/{}", base_path, text);

        create_file(full_path, create_context);
    }
}

fn create_file(full_path: impl AsRef<Path>, create_context: &CreateContext) {
    let result = if create_context.is_file {
        fs::write(full_path, "")
    } else {
        fs::create_dir_all(full_path)
    };

    if let Err(e) = result {
        // TODO: error notification
        println!("Could not create file: {}", e);
    }
}
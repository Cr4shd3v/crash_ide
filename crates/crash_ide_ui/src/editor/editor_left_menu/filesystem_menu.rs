use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crash_ide_assets::{DefaultColors, DefaultFonts, DefaultIcons};
use crash_ide_file::{FileEventData, FileExtensionManager, RawOpenFileEvent};
use crash_ide_file_watcher::FileWatcher;
use crash_ide_project::FindProjectInParents;
use crash_ide_widget::{DoubleClickButton, DoubleClicked, RightClickable, Scrollable, ScrollableContent};
pub use file_path::*;

use crate::editor::editor_left_menu::filesystem_menu::file_context_menu::{file_context_menu, FileContextMenuPlugin};
use crate::editor::editor_left_menu::filesystem_menu::file_watcher::handle_file_watcher;
use crate::editor::main_editor_screen::{EditorLeftMenu, ProjectsFileViews};

mod file_context_menu;
mod filename_dialog;
mod file_path;
mod file_watcher;

pub struct FilesystemMenuPlugin;

impl Plugin for FilesystemMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, ((spawn_left_menu, expand_directory), spawn_all_rows).chain())
            .add_systems(Update, (
                directory_expand_icon, highlight_clicked_row,
                handle_file_watcher,
            ))
            .add_event::<ExpandDirEvent>()
            .add_plugins(FileContextMenuPlugin)
        ;
    }
}

#[derive(Component)]
pub struct ProjectRoot {
    display_path: String,
    full_path: String,
}

#[derive(Component)]
pub struct FileDisplay {
    filename: String,
    is_file: bool,
    level: u16,
}

#[derive(Component)]
struct SelfFileRow;

#[derive(Component)]
struct ExpandDirIcon;

#[derive(Component)]
struct DirectoryExpanded;

impl FileDisplay {
    pub fn new(filename: String, is_file: bool, level: u16) -> Self {
        Self {
            filename,
            is_file,
            level,
        }
    }
}

fn spawn_left_menu(
    mut commands: Commands,
    query: Query<Entity, Added<EditorLeftMenu>>,
    find_project: FindProjectInParents,
) {
    for entity in query.iter() {
        let project = find_project.find(entity);

        let full_path = project.crash_ide_project.path.clone();

        // Linux: Change /home/<user>/ to ~/
        #[cfg(target_os = "linux")]
        let display_path = {
            let mut display_path = full_path.clone();
            if display_path.starts_with("/home") {
                display_path = format!("~/{}", display_path.split("/").skip(3).collect::<Vec<&str>>().join("/"));
            }
            display_path
        };

        #[cfg(not(target_os = "linux"))]
        let display_path = full_path.clone();

        commands.entity(entity).despawn_descendants().with_children(|parent| {
            parent.spawn((NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            }, Scrollable::default(), Interaction::None)).with_children(|parent| {
                parent.spawn((
                    ScrollableContent::default(),
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        ..default()
                    },
                )).with_children(|parent| {
                    parent.spawn((
                        FileDisplay::new(project.crash_ide_project.name.clone(), false, 0),
                        FileWatcher {
                            path: full_path.clone(),
                        },
                        ProjectRoot { display_path, full_path },
                    ));
                });
            });
        });
    }
}

fn spawn_all_rows(
    mut commands: Commands,
    query: Query<(Entity, &FileDisplay, Option<&ProjectRoot>), Added<FileDisplay>>,
    icons: Res<DefaultIcons>,
    mut event_writer: EventWriter<ExpandDirEvent>,
    extension_manager: Res<FileExtensionManager>,
) {
    for (entity, file_display, root) in query.iter() {
        let row_entity = commands.entity(entity).insert(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            parent.spawn((NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    padding: UiRect::vertical(Val::Px(2.0)),
                    ..default()
                },
                ..default()
            }, SelfFileRow, DoubleClickButton::default(), RightClickable, Interaction::None))
                .observe(file_context_menu)
                .observe(double_click_row)
                .with_children(|parent| {
                if !file_display.is_file {
                    parent.spawn((ImageBundle {
                        image: UiImage {
                            texture: icons.right.clone(),
                            ..default()
                        },
                        style: Style {
                            height: Val::Vh(2.0),
                            margin: UiRect::left(Val::Vh(2.0 * file_display.level as f32)),
                            ..default()
                        },
                        z_index: ZIndex::Local(1),
                        focus_policy: FocusPolicy::Block,
                        ..default()
                    }, ExpandDirIcon, Interaction::None));
                }

                parent.spawn(ImageBundle {
                    image: UiImage {
                        texture: if file_display.is_file {
                            let data = extension_manager.get_data(
                                &PathBuf::from(&file_display.filename).extension()
                                    .unwrap_or(&OsStr::new("")).to_str().unwrap().to_string()
                            );
                            if let Some(data) = data {
                                data.get_icon().clone()
                            } else {
                                icons.unknown_file.clone()
                            }
                        } else {
                            icons.folder.clone()
                        },
                        ..default()
                    },
                    style: Style {
                        height: Val::Vh(2.0),
                        margin: if file_display.is_file {
                            UiRect::left(Val::Vh(2.0 + 2.0 * file_display.level as f32))
                        } else {
                            UiRect::default()
                        },
                        ..default()
                    },
                    ..default()
                });

                parent.spawn(TextBundle {
                    text: Text::from_section(file_display.filename.clone(), TextStyle {
                        font: DefaultFonts::ROBOTO_REGULAR,
                        font_size: 18.0,
                        ..default()
                    }),
                    style: Style {
                        margin: UiRect::horizontal(Val::Px(4.0)),
                        ..default()
                    },
                    ..default()
                });

                if let Some(root) = root {
                    parent.spawn(TextBundle {
                        text: Text::from_section(root.display_path.clone(), TextStyle {
                            font: DefaultFonts::ROBOTO_REGULAR,
                            font_size: 18.0,
                            color: DefaultColors::GRAY.with_alpha(0.5),
                        }),
                        style: Style {
                            margin: UiRect::left(Val::Px(2.0)),
                            ..default()
                        },
                        ..default()
                    });
                }
            });
        }).id();

        if root.is_some() {
            event_writer.send(ExpandDirEvent {
                row_entity,
                expand: true,
            });
        }
    }
}

#[derive(Event)]
pub struct ExpandDirEvent {
    pub row_entity: Entity,
    pub expand: bool,
}

fn directory_expand_icon(
    query: Query<(&Interaction, &Parent), (Changed<Interaction>, With<ExpandDirIcon>)>,
    mut event_writer: EventWriter<ExpandDirEvent>,
    parent_query: Query<&Parent>,
    expanded_query: Query<Option<&DirectoryExpanded>>,
) {
    for (interaction, parent) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let row_entity = parent_query.get(parent.get()).unwrap().get();
        let expanded = expanded_query.get(row_entity).unwrap().is_some();

        event_writer.send(ExpandDirEvent {
            row_entity,
            expand: !expanded,
        });
    }
}

#[derive(Component)]
struct HighlightedFileRow;

fn highlight_clicked_row(
    mut commands: Commands,
    query: Query<(Entity, &Interaction), (Changed<Interaction>, With<DoubleClickButton>)>,
    mut background_color_query: Query<&mut BackgroundColor>,
    current_button_query: Query<Entity, With<HighlightedFileRow>>,
) {
    for (entity, interaction) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let mut background_color = background_color_query.get_mut(entity).unwrap();
        background_color.0 = DefaultColors::PRIMARY_BUTTON;

        for current_entity in current_button_query.iter() {
            if current_entity == entity {
                continue;
            }

            commands.entity(current_entity).remove::<HighlightedFileRow>();
            let mut background_color = background_color_query.get_mut(current_entity).unwrap();
            background_color.0 = Color::NONE;
        }

        commands.entity(entity).insert(HighlightedFileRow);
        break;
    }
}

fn double_click_row(
    trigger: Trigger<DoubleClicked>,
    mut commands: Commands,
    parent_query: Query<&Parent>,
    file_display_query: Query<(&FileDisplay, Option<&DirectoryExpanded>)>,
    mut dir_event_writer: EventWriter<ExpandDirEvent>,
    mut file_event_writer: EventWriter<RawOpenFileEvent>,
    file_path: FilePath,
    find_project_in_parents: FindProjectInParents,
    projects_file_views: Res<ProjectsFileViews>,
) {
    let parent = parent_query.get(trigger.entity()).unwrap();
    let entity = parent.get();
    let (file_display, expanded) = file_display_query.get(entity).unwrap();

    if file_display.is_file {
        let project = find_project_in_parents.find_project_ref(entity);
        let path = PathBuf::from(file_path.get_full_path(entity));

        if fs::metadata(&path).is_err() {
            commands.entity(entity).despawn_recursive();
            return;
        }

        file_event_writer.send(RawOpenFileEvent {
            event_data: FileEventData {
                path,
                view_entity: projects_file_views.get(project),
            },
        });
    } else {
        dir_event_writer.send(ExpandDirEvent {
            row_entity: entity,
            expand: expanded.is_none(),
        });
    }
}

fn expand_directory(
    mut commands: Commands,
    mut event_reader: EventReader<ExpandDirEvent>,
    query: Query<(Option<&DirectoryExpanded>, &Children)>,
    children_query: Query<&Children>,
    mut image_query: Query<&mut UiImage>,
    icons: Res<DefaultIcons>,
    file_display_query: Query<Option<&FileDisplay>>,
    file_path: FilePath,
) {
    for event in event_reader.read() {
        let (expanded, children) = query.get(event.row_entity).unwrap();
        let is_expanded = expanded.is_some();

        if is_expanded == event.expand {
            continue;
        }

        let self_row = children.first().unwrap().clone();
        let expand_icon = children_query.get(self_row).unwrap().first().unwrap().clone();
        let mut ui_image = image_query.get_mut(expand_icon).unwrap();

        if is_expanded {
            ui_image.texture = icons.right.clone();
            commands.entity(event.row_entity).remove::<DirectoryExpanded>();
            for entity in children.iter() {
                let file_display = file_display_query.get(entity.clone()).unwrap();
                if file_display.is_some() {
                    commands.entity(entity.clone()).despawn_recursive();
                }
            }
        } else {
            ui_image.texture = icons.down.clone();
            commands.entity(event.row_entity).insert(DirectoryExpanded);

            let file_display = file_display_query.get(event.row_entity).unwrap().unwrap();
            let dir_path = file_path.get_full_path(event.row_entity);
            let mut entities = vec![];

            let Ok(dir_entries) = fs::read_dir(&dir_path) else {
                commands.entity(event.row_entity).despawn_recursive();
                continue;
            };

            let mut directories = vec![];
            let mut files = vec![];

            for entry in dir_entries {
                let Ok(dir_entry) = entry else {continue;};

                let name = dir_entry.file_name().to_str().unwrap().to_string();
                if dir_entry.file_type().unwrap().is_file() {
                    files.push(name);
                } else {
                    directories.push(name);
                }
            }

            directories.sort_by(|v1, v2| v1.to_lowercase().cmp(&v2.to_lowercase()));
            files.sort_by(|v1, v2| v1.to_lowercase().cmp(&v2.to_lowercase()));

            for dir_entry in directories {
                entities.push(commands.spawn(FileDisplay {
                    filename: dir_entry,
                    level: file_display.level + 1,
                    is_file: false,
                }).id());
            }

            for file_entry in files {
                entities.push(commands.spawn(FileDisplay {
                    filename: file_entry,
                    level: file_display.level + 1,
                    is_file: true,
                }).id());
            }

            commands.entity(event.row_entity).push_children(entities.as_slice());
        }
    }
}
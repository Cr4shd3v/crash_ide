use std::fs;
use std::path::PathBuf;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use editor_assets::{DefaultFonts, DefaultIcons};
use editor_config::FindProjectInParents;
use editor_file::{FileEventData, FileExtensionManager, RawOpenFileEvent};
use editor_widget::{DoubleClickButton, DoubleClicked};

use crate::editor::main_editor_screen::{EditorLeftMenu, ProjectsFileViews};

pub struct FilesystemMenuPlugin;

impl Plugin for FilesystemMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, ((spawn_left_menu, expand_directory), spawn_all_rows).chain())
            .add_systems(Update, (directory_expand_icon, double_click_row, highlight_clicked_row))
            .add_event::<ExpandDirEvent>()
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

        let full_path = project.editor_project.path.clone();
        let mut display_path = full_path.clone();

        // Linux: Change /home/<user>/ to ~/
        #[cfg(target_os = "linux")]
        {
            if display_path.starts_with("/home") {
                display_path = format!("~/{}", display_path.split("/").skip(3).collect::<Vec<&str>>().join("/"));
            }
        }

        commands.entity(entity).despawn_descendants().with_children(|parent| {
            parent.spawn((
                FileDisplay::new(project.editor_project.name.clone(), false, 0),
                ProjectRoot { display_path, full_path },
            ));
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
            }, SelfFileRow, DoubleClickButton::default(), Interaction::None)).with_children(|parent| {
                if !file_display.is_file {
                    parent.spawn((ImageBundle {
                        image: UiImage {
                            texture: icons.right.clone(),
                            ..default()
                        },
                        style: Style {
                            height: Val::Px(20.0),
                            margin: UiRect::left(Val::Px(20.0 * file_display.level as f32)),
                            ..default()
                        },
                        ..default()
                    }, ExpandDirIcon, Interaction::None));
                }

                parent.spawn(ImageBundle {
                    image: UiImage {
                        texture: if file_display.is_file {
                            let data = extension_manager.get_data(
                                &PathBuf::from(&file_display.filename).extension().unwrap().to_str().unwrap().to_string()
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
                        height: Val::Px(20.0),
                        margin: if file_display.is_file {
                            UiRect::left(Val::Px(20.0 + 20.0 * file_display.level as f32))
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
                        font_size: 14.0,
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
                            font_size: 14.0,
                            color: Color::GRAY.with_a(0.5),
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
        background_color.0 = Color::BLUE;

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
    mut commands: Commands,
    query: Query<&Parent, Added<DoubleClicked>>,
    file_display_query: Query<(&FileDisplay, Option<&DirectoryExpanded>)>,
    mut dir_event_writer: EventWriter<ExpandDirEvent>,
    mut file_event_writer: EventWriter<RawOpenFileEvent>,
    file_path: FilePath,
    find_project_in_parents: FindProjectInParents,
    projects_file_views: Res<ProjectsFileViews>,
) {
    for parent in query.iter() {
        let entity = parent.get();
        let (file_display, expanded) = file_display_query.get(entity).unwrap();

        if file_display.is_file {
            let project = find_project_in_parents.find_project_ref(entity);
            let path = PathBuf::from(file_path.get_full_path(entity));

            if fs::metadata(&path).is_err() {
                commands.entity(entity).despawn_recursive();
                continue;
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
}

#[derive(SystemParam)]
pub struct FilePath<'w, 's> {
    query: Query<'w, 's, (&'static Parent, &'static FileDisplay, Option<&'static ProjectRoot>)>,
}

impl<'w, 's> FilePath<'w, 's> {
    pub fn get_full_path(&self, row_entity: Entity) -> String {
        let mut entity = row_entity;
        let mut path = vec![];

        loop {
            let (parent, file_display, root) = self.query.get(entity).unwrap();

            if let Some(root) = root {
                path.reverse();
                return format!("{}/{}", root.full_path, path.join("/"));
            } else {
                path.push(file_display.filename.clone());
            }

            entity = parent.get();
        }
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
            println!("Directory already expanded");
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

            for dir_entry in dir_entries {
                let Ok(dir_entry) = dir_entry else {continue;};

                entities.push(commands.spawn(FileDisplay {
                    filename: dir_entry.file_name().to_str().unwrap().to_string(),
                    level: file_display.level + 1,
                    is_file: dir_entry.file_type().unwrap().is_file(),
                }).id());
            }

            commands.entity(event.row_entity).push_children(entities.as_slice());
        }
    }
}

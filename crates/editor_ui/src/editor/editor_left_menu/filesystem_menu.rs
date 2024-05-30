use bevy::prelude::*;
use editor_config::FindProjectInParents;
use crate::editor::main_editor_screen::EditorLeftMenu;
use crate::fonts::DefaultFonts;
use crate::icons::DefaultIcons;

pub struct FilesystemMenuPlugin;

impl Plugin for FilesystemMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_left_menu, spawn_all_rows).chain())
        ;
    }
}

#[derive(Component)]
pub struct ProjectRoot {
    base_path: String,
}

#[derive(Component)]
pub struct FileDisplay {
    filename: String,
    is_file: bool,
    level: u16,
}

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

        let mut base_path = project.editor_project.path.clone();

        // Linux: Change /home/<user>/ to ~/
        #[cfg(target_os = "linux")]
        {
            if base_path.starts_with("/home") {
                base_path = format!("~/{}", base_path.split("/").skip(3).collect::<Vec<&str>>().join("/"));
            }
        }

        commands.entity(entity).despawn_descendants().with_children(|parent| {
            parent.spawn((
                FileDisplay::new(project.editor_project.name.clone(), false, 0),
                ProjectRoot { base_path: base_path.clone() },
            ));
        });
    }
}

fn spawn_all_rows(
    mut commands: Commands,
    query: Query<(Entity, &FileDisplay, Option<&ProjectRoot>), Added<FileDisplay>>,
    icons: Res<DefaultIcons>,
) {
    for (entity, file_display, root) in query.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn(ImageBundle {
                    image: UiImage {
                        texture: icons.right.clone(),
                        ..default()
                    },
                    style: Style {
                        height: Val::Px(22.5),
                        margin: UiRect::left(Val::Px(8.0 * file_display.level as f32)),
                        ..default()
                    },
                    ..default()
                });

                parent.spawn(ImageBundle {
                    image: UiImage {
                        texture: if file_display.is_file {
                            icons.unknown_file.clone()
                        } else {
                            icons.folder.clone()
                        },
                        ..default()
                    },
                    style: Style {
                        height: Val::Px(22.5),
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
                        text: Text::from_section(root.base_path.clone(), TextStyle {
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
        });
    }
}
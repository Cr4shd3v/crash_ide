use bevy::prelude::*;
use editor_config::{ProjectRef, Projects};
use crate::editor::main_editor_screen::EditorLeftMenu;
use crate::fonts::DefaultFonts;
use crate::icons::DefaultIcons;

pub struct FilesystemMenuPlugin;

impl Plugin for FilesystemMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_left_menu)
        ;
    }
}

#[derive(Component)]
pub struct FileDisplay {
    filename: String,
    is_file: bool,
}

fn spawn_left_menu(
    mut commands: Commands,
    query: Query<(Entity, &ProjectRef), Added<EditorLeftMenu>>,
    projects: Projects,
    icons: Res<DefaultIcons>,
) {
    for (entity, project_ref) in query.iter() {
        let project = projects.get_by_ref(project_ref);

        let mut base_path = project.editor_project.path.clone();

        // Linux: Change /home/<user>/ to ~/
        #[cfg(target_os = "linux")]
        {
            if base_path.starts_with("/home") {
                base_path = format!("~/{}", base_path.split("/").skip(3).collect::<Vec<&str>>().join("/"));
            }
        }

        commands.entity(entity).despawn_descendants().with_children(|parent| {
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
                        ..default()
                    },
                    ..default()
                });

                parent.spawn(ImageBundle {
                    image: UiImage {
                        texture: icons.folder.clone(),
                        ..default()
                    },
                    style: Style {
                        height: Val::Px(22.5),
                        ..default()
                    },
                    ..default()
                });

                parent.spawn(TextBundle {
                    text: Text::from_section(project.editor_project.name.clone(), TextStyle {
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

                parent.spawn(TextBundle {
                    text: Text::from_section(base_path, TextStyle {
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
            });
        });
    }
}
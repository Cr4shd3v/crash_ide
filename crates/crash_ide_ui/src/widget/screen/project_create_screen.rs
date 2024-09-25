use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use bevy::color::palettes::css::RED;
use bevy::prelude::*;

use crash_ide_config::{EditorConfigProjects, HomeDir};

use crash_ide_assets::{DefaultColors, DefaultFonts};
use crash_ide_project::{EditorProject, OpenProjectEvent, ProjectFiles};
use crash_ide_text_input::{TextInputBundle, TextInputContent, TextInputSettings, TextInputStyle};
use crate::trigger::Clicked;
use crate::widget::folder_input::FolderInput;
use crate::window::AllWindows;

pub(super) struct ProjectCreateScreenPlugin;

impl Plugin for ProjectCreateScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_project_create_screen)
        ;
    }
}

/// Marker component for the create project dialog
#[derive(Component, Default)]
pub struct CreateProjectWindow {
    pub(crate) base_window: Option<Entity>,
}

const DEFAULT_NEW_PROJECT_NAME: &'static str = "untitled";

#[derive(Component)]
struct ProjectPathInput;

#[derive(Component)]
struct CreateProjectConfirmButton;

#[derive(Component)]
struct CreateProjectPathErrorText;

fn spawn_project_create_screen(
    mut commands: Commands,
    query: Query<Entity, Added<CreateProjectWindow>>,
    home_dir: Res<HomeDir>,
    all_windows: Res<AllWindows>,
) {
    for window_ui_root in query.iter() {
        commands.entity(all_windows.get(&window_ui_root).ui_root).despawn_descendants().with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::axes(Val::Percent(10.0), Val::Percent(5.0)),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn(TextBundle::from_section("Project Path", TextStyle {
                    font: DefaultFonts::ROBOTO_REGULAR,
                    font_size: 18.0,
                    ..default()
                }));

                parent.spawn((
                    TextInputBundle {
                        content: TextInputContent::from_string(home_dir.projects_path.join(DEFAULT_NEW_PROJECT_NAME).to_str().unwrap().to_string()),
                        text_style: TextInputStyle {
                            font: DefaultFonts::ROBOTO_REGULAR,
                            font_size: 14.0,
                        },
                        settings: TextInputSettings {
                            // input_width: Val::Percent(96.0),
                            ..default()
                        },
                        ..default()
                    },
                    NodeBundle {
                        style: Style {
                            width: Val::Vw(80.0),
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        ..default()
                    },
                    ProjectPathInput,
                    FolderInput,
                ));

                parent.spawn((TextBundle {
                    text: Text::from_section("", TextStyle {
                        font: DefaultFonts::ROBOTO_REGULAR,
                        color: RED.into(),
                        font_size: 18.0,
                    }),
                    style: Style {
                        margin: UiRect::vertical(Val::Px(3.0)),
                        ..default()
                    },
                    ..default()
                }, CreateProjectPathErrorText));

                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::RowReverse,
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                padding: UiRect::axes(Val::Px(5.0), Val::Px(2.5)),
                                margin: UiRect::top(Val::Vh(2.5)),
                                ..default()
                            },
                            background_color: BackgroundColor(DefaultColors::PRIMARY_BUTTON),
                            border_radius: BorderRadius::all(Val::Px(2.0)),
                            ..default()
                        },
                        Interaction::None,
                        Button,
                        CreateProjectConfirmButton,
                    )).observe(create_project_confirm).with_children(|parent| {
                        parent.spawn(TextBundle::from_section("Create", TextStyle {
                            font_size: 18.0,
                            font: DefaultFonts::ROBOTO_REGULAR,
                            ..default()
                        }));
                    });
                });
            });
        });
    }
}

fn create_project_confirm(
    _: Trigger<Clicked>,
    mut commands: Commands,
    window_query: Query<(Entity, &CreateProjectWindow)>,
    path_input_query: Query<&TextInputContent, With<ProjectPathInput>>,
    mut projects: ResMut<EditorConfigProjects>,
    mut event_writer: EventWriter<OpenProjectEvent>,
    mut error_text: Query<&mut Text, With<CreateProjectPathErrorText>>,
) {
    let (entity, create_project_window) = window_query.single();
    let path = path_input_query.single().to_string();

    if projects.projects.contains_key(&path) {
        error_text.single_mut().sections[0].value = "A project already exists at this path".to_string();
        return;
    }

    let path_buf = PathBuf::from_str(&*path).unwrap();
    let name = path_buf.file_name().unwrap().to_str().unwrap().to_string();

    if fs::metadata(&path_buf).is_err() {
        if let Err(e) = fs::create_dir_all(&path_buf) {
            error_text.single_mut().sections[0].value = format!("Failed to create folder: {}", e);
            return;
        };
    }

    if let Err(e) = ProjectFiles::create_new_project_files(&path_buf, &name) {
        error_text.single_mut().sections[0].value = format!("Failed to create folder: {}", e);
        return;
    }

    let config = EditorProject {
        name,
        path: path.clone(),
    };

    projects.projects.insert(path, config.clone());

    commands.entity(entity).despawn();

    event_writer.send(OpenProjectEvent::new(config, create_project_window.base_window));
}
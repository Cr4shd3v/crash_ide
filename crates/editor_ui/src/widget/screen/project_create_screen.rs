use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use bevy::prelude::*;

use editor_config::{EditorConfigProjects, EditorProject, HomeDir};
use editor_widget::{TextInputBundle, TextInputTextStyle, TextInputValue};

use crate::fonts::DefaultFonts;
use crate::open_project::OpenProjectEvent;
use crate::window::{WindowCamera, WindowUiRoot};

/// Marker component for the create project dialog
#[derive(Component, Default)]
pub struct CreateProjectWindow {
    pub(crate) base_window: Option<Entity>,
}

const DEFAULT_NEW_PROJECT_NAME: &'static str = "untitled";

#[derive(Component)]
pub(super) struct ProjectPathInput;

#[derive(Component)]
pub(super) struct CreateProjectConfirmButton;

pub(super) fn spawn_project_create_screen(
    mut commands: Commands,
    query: Query<&WindowUiRoot, Added<CreateProjectWindow>>,
    home_dir: Res<HomeDir>,
) {
    for window_ui_root in query.iter() {
        commands.entity(window_ui_root.root).despawn_descendants().with_children(|parent| {
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
                    ..default()
                }));
                parent.spawn((
                    TextInputBundle {
                        text_input_value: TextInputValue(home_dir.projects_path.join(DEFAULT_NEW_PROJECT_NAME).to_str().unwrap().to_string()),
                        text_input_text_style: TextInputTextStyle::default().with_font(DefaultFonts::ROBOTO_REGULAR),
                        ..default()
                    },
                    NodeBundle {
                        style: Style {
                            width: Val::Vw(80.0),
                            ..default()
                        },
                        ..default()
                    },
                    ProjectPathInput,
                ));

                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::RowReverse,
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(5.0)),
                                margin: UiRect::top(Val::Vh(2.5)),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::hex("#578AF2").unwrap()),
                            ..default()
                        },
                        CreateProjectConfirmButton,
                    )).with_children(|parent| {
                        parent.spawn(TextBundle::from_section("Create", TextStyle {
                            font_size: 16.0,
                            font: DefaultFonts::ROBOTO_REGULAR,
                            ..default()
                        }));
                    });
                });
            });
        });
    }
}

pub(super) fn create_project_confirm(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (With<CreateProjectConfirmButton>, Changed<Interaction>)>,
    window_query: Query<(Entity, &CreateProjectWindow, &WindowUiRoot, &WindowCamera)>,
    path_input_query: Query<&TextInputValue, With<ProjectPathInput>>,
    mut projects: ResMut<EditorConfigProjects>,
    mut event_writer: EventWriter<OpenProjectEvent>,
) {
    for interaction in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let (entity, create_project_window,
            ui_root, window_camera) = window_query.single();
        let path = path_input_query.single().0.clone();

        if projects.projects.contains_key(&path) {
            // Todo: Error message
            return;
        }

        if fs::metadata(&path).is_err() {
            if let Err(_) = fs::create_dir_all(&path) {
                // Todo: Error message
            };
        }

        let config = EditorProject {
            name: PathBuf::from_str(&*path).unwrap().file_name().unwrap().to_str().unwrap().to_string(),
            path: path.clone(),
        };

        projects.projects.insert(path, config.clone());

        commands.entity(entity).despawn();
        commands.entity(ui_root.root).despawn_recursive();
        commands.entity(window_camera.camera).despawn();

        event_writer.send(OpenProjectEvent::new(config, create_project_window.base_window));
    }
}
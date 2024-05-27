use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::winit::WinitWindows;
use editor_config::{EditorConfigProjects, EditorProject, HomeDir};
use crate::fonts::DefaultFonts;
use crate::widget::input::{TextInputBundle, TextInputValue};
use crate::window::{ProjectWindow, StartupWindow, WindowCamera, WindowUiRoot};

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
    winit_windows: NonSend<WinitWindows>,
    mut window_query_resize: Query<&mut Window>,
) {
    for interaction in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let (entity, create_project_window,
            ui_root, window_camera) = window_query.single();
        let path = path_input_query.single().0.clone();

        if projects.projects.iter().any(|project| project.path == path) {
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
            path,
        };

        projects.projects.push(config.clone());

        commands.entity(entity).despawn();
        commands.entity(ui_root.root).despawn_recursive();
        commands.entity(window_camera.camera).despawn();

        let monitor_size = winit_windows.get_window(entity).unwrap().current_monitor().unwrap().size();
        let new_resolution = WindowResolution::new(monitor_size.width as f32, monitor_size.height as f32);

        if let Some(window) = create_project_window.base_window {
            commands.entity(window).remove::<StartupWindow>().insert(ProjectWindow {
                project_editor_config: config,
            });

            window_query_resize.get_mut(window).unwrap().resolution = new_resolution;
        } else {
            commands.spawn((
                Window {
                    title: config.name.clone(),
                    resolution: new_resolution,
                    ..default()
                },
                ProjectWindow {
                    project_editor_config: config,
                }
            ));
        }
    }
}
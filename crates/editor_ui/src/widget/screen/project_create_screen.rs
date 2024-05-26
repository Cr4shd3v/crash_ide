use bevy::prelude::*;
use editor_config::HomeDir;
use crate::widget::input::{TextInputBundle, TextInputValue};
use crate::window::WindowCamera;

#[derive(Component)]
struct CreateProjectRoot;

/// Marker component for the create project dialog
#[derive(Component)]
pub struct CreateProjectWindow;

const DEFAULT_NEW_PROJECT_NAME: &'static str = "untitled";

#[derive(Component)]
struct ProjectPathInput;

#[derive(Component)]
struct CreateProjectConfirmButton;

pub(super) fn spawn_project_create_screen(
    mut commands: Commands,
    query: Query<&WindowCamera, Added<CreateProjectWindow>>,
    home_dir: Res<HomeDir>,
) {
    for window_camera in query.iter() {
        commands.spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::axes(Val::Percent(10.0), Val::Percent(5.0)),
                    ..default()
                },
                ..default()
            },
            TargetCamera(window_camera.camera),
            CreateProjectRoot,
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section("Project Path", TextStyle::default()));
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
                        ..default()
                    }));
                });
            });
        });
    }
}
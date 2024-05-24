use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::{WindowRef, WindowResolution};

#[derive(Component)]
pub struct CreateProjectButton;

#[derive(Component)]
pub struct CreateProjectWindow;

#[derive(Component)]
pub struct CreateProjectWindowCamera;

#[derive(Component)]
pub struct CreateProjectRoot;

pub(super) fn create_project_button(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (With<CreateProjectButton>, Changed<Interaction>)>,
    mut create_project_window_query: Query<&mut Window, With<CreateProjectWindow>>,
) {
    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                if let Ok(mut window) = create_project_window_query.get_single_mut() {
                    window.focused = true;
                } else {
                    let new_window = commands.spawn((
                        Window {
                            title: "Create Project".to_string(),
                            resolution: WindowResolution::new(1000.0, 700.0),
                            ..default()
                        },
                        CreateProjectWindow,
                    )).id();

                    let camera = commands.spawn((
                        Camera2dBundle {
                            camera: Camera {
                                target: RenderTarget::Window(WindowRef::Entity(new_window)),
                                ..default()
                            },
                            ..default()
                        },
                        CreateProjectWindowCamera,
                    )).id();

                    commands.spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            ..default()
                        },
                        TargetCamera(camera),
                        CreateProjectRoot,
                    )).with_children(|parent| {
                        parent.spawn(TextBundle::from_section("test", TextStyle::default()));
                    });
                }
            }
            _ => {}
        }
    }
}
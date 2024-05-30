use bevy::prelude::*;
use editor_config::{LoadedEditorProject, ProjectRef};
use crate::fonts::DefaultFonts;
use crate::window::{ProjectWindow, WindowUiRoot};

pub struct MainEditorScreenPlugin;

impl Plugin for MainEditorScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_main_editor_screen)
        ;
    }
}

#[derive(Component)]
pub struct EditorTopMenu;

#[derive(Component)]
pub struct EditorLeftMenu;

#[derive(Component)]
pub struct EditorFileView;

#[derive(Component)]
pub struct EditorBottomMenu;

pub(super) fn spawn_main_editor_screen(
    mut commands: Commands,
    window_query: Query<(&WindowUiRoot, &ProjectWindow), Added<ProjectWindow>>,
    project_query: Query<&LoadedEditorProject>,
) {
    for (ui_root, project_window) in window_query.iter() {
        commands.entity(ui_root.root).despawn_descendants().with_children(|parent| {
            parent.spawn((NodeBundle {
                style: Style {
                    height: Val::Vh(4.0),
                    width: Val::Vw(100.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                background_color: BackgroundColor(Color::RED),
                ..default()
            }, EditorTopMenu, ProjectRef(project_window.project_editor_config)));

            parent.spawn((NodeBundle {
                style: Style {
                    height: Val::Vh(66.0),
                    width: Val::Vw(100.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            }, ProjectRef(project_window.project_editor_config))).with_children(|parent| {
                parent.spawn((NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(20.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::hex("#21252B").unwrap()),
                    ..default()
                }, EditorLeftMenu));
                parent.spawn((NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(80.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::hex("#282C34").unwrap()),
                    ..default()
                }, EditorFileView)).with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        format!("Editor at {}", project_query.get(project_window.project_editor_config).unwrap().editor_project.path),
                        TextStyle {
                            font: DefaultFonts::ROBOTO_REGULAR,
                            ..default()
                        },
                    ));
                });
            });

            parent.spawn((NodeBundle {
                style: Style {
                    height: Val::Vh(30.0),
                    width: Val::Vw(100.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::BLUE),
                ..default()
            }, EditorBottomMenu, ProjectRef(project_window.project_editor_config)));
        });
    }
}
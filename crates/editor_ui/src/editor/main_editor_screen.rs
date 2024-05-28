use bevy::prelude::*;
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
            }, EditorTopMenu));

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Vh(66.0),
                    width: Val::Vw(100.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn((NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(20.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::GREEN),
                    ..default()
                }, EditorLeftMenu));
                parent.spawn((NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(80.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                }, EditorFileView)).with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        format!("Editor at {}", project_window.project_editor_config.path),
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
            }, EditorBottomMenu));
        });
    }
}
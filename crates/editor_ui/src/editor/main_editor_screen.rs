use bevy::prelude::*;
use editor_config::ProjectRef;
use crate::window::{AllWindows, ProjectWindow};

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
    window_query: Query<(Entity, &ProjectWindow), Added<ProjectWindow>>,
    all_windows: Res<AllWindows>,
) {
    for (window_entity, project_window) in window_query.iter() {
        commands.entity(all_windows.get(&window_entity).ui_root).despawn_descendants().with_children(|parent| {
            parent.spawn((NodeBundle {
                style: Style {
                    height: Val::Vh(3.5),
                    width: Val::Vw(100.0),
                    flex_direction: FlexDirection::Row,
                    border: UiRect::bottom(Val::Px(2.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::hex("#282C34").unwrap()),
                border_color: BorderColor(Color::GRAY.with_a(0.1)),
                ..default()
            }, EditorTopMenu, ProjectRef(project_window.project_editor_config)));

            parent.spawn((NodeBundle {
                style: Style {
                    height: Val::Vh(66.5),
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
                }, EditorFileView));
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
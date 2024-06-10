use bevy::prelude::*;
use bevy::utils::HashMap;
use crash_ide_assets::DefaultColors;
use crash_ide_config::ProjectRef;
use crate::window::{AllWindows, ProjectWindow};

pub(super) struct MainEditorScreenPlugin;

impl Plugin for MainEditorScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_main_crash_ide_screen)
            .init_resource::<ProjectsFileViews>()
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

#[derive(Resource, Default)]
pub struct ProjectsFileViews {
    /// Maps project [Entity] to file view [Entity]
    map: HashMap<Entity, Entity>,
}

impl ProjectsFileViews {
    /// Returns the [EditorFileView](EditorFileView) [Entity] for a [ProjectRef]
    pub fn get(&self, project_ref: &ProjectRef) -> Entity {
        self.map.get(&project_ref.0).unwrap().clone()
    }
}

pub(super) fn spawn_main_crash_ide_screen(
    mut commands: Commands,
    window_query: Query<(Entity, &ProjectWindow), Added<ProjectWindow>>,
    all_windows: Res<AllWindows>,
    mut project_file_views: ResMut<ProjectsFileViews>,
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
                background_color: BackgroundColor(DefaultColors::MAIN_VIEW_BACKGROUND),
                border_color: BorderColor(Color::GRAY.with_a(0.1)),
                ..default()
            }, EditorTopMenu, ProjectRef(project_window.project_crash_ide_config)));

            parent.spawn((NodeBundle {
                style: Style {
                    height: Val::Vh(66.5),
                    width: Val::Vw(100.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            }, ProjectRef(project_window.project_crash_ide_config))).with_children(|parent| {
                parent.spawn((NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(20.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: BackgroundColor(DefaultColors::LEFT_MENU_BACKGROUND),
                    ..default()
                }, EditorLeftMenu));

                let view_id = parent.spawn((NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(80.0),
                        ..default()
                    },
                    background_color: BackgroundColor(DefaultColors::MAIN_VIEW_BACKGROUND),
                    ..default()
                }, EditorFileView)).id();

                project_file_views.map.insert(project_window.project_crash_ide_config, view_id);
            });

            parent.spawn((NodeBundle {
                style: Style {
                    height: Val::Vh(30.0),
                    width: Val::Vw(100.0),
                    border: UiRect::top(Val::Px(2.0)),
                    ..default()
                },
                border_color: BorderColor(Color::GRAY.with_a(0.1)),
                background_color: BackgroundColor(DefaultColors::MAIN_VIEW_BACKGROUND),
                ..default()
            }, EditorBottomMenu, ProjectRef(project_window.project_crash_ide_config)));
        });
    }
}
use bevy::prelude::*;
use bevy::utils::HashMap;
use crash_ide_assets::DefaultColors;
use crash_ide_project::ProjectRef;
use crate::widget::notification::NotificationContainerMap;
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

/// Marker component for the top menu.
#[derive(Component)]
pub struct EditorTopMenu;

/// Marker component for the left menu.
#[derive(Component)]
pub struct EditorLeftMenu;

/// Marker component for the file view in the center.
#[derive(Component)]
pub struct EditorFileView;

/// Marker component for the bottom menu.
#[derive(Component)]
pub struct EditorBottomMenu;

/// Resource containing the file view per project
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
    mut notification_container: ResMut<NotificationContainerMap>,
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

            let notification_container_id = parent.spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    flex_direction: FlexDirection::ColumnReverse,
                    width: Val::Vw(20.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                z_index: ZIndex::Global(10),
                ..default()
            }).id();

            notification_container.notification_map.insert(window_entity, notification_container_id);
        });
    }
}
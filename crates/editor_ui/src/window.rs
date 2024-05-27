use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::{WindowRef, WindowResolution};
use editor_config::EditorProject;

pub(super) struct EditorWindowPlugin;

impl Plugin for EditorWindowPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, initial_window)
            .add_systems(PreUpdate, (update_active_window, process_new_window))
            .add_systems(PostUpdate, check_for_exit)
        ;
    }
}

#[derive(Component)]
pub struct ActiveWindow;

fn update_active_window(
    mut commands: Commands,
    mut window_focused_event_reader: EventReader<CursorEntered>,
    current_query: Query<Entity, With<ActiveWindow>>,
) {
    for window_focused in window_focused_event_reader.read() {
        if let Ok(current_entity) = current_query.get_single() {
            commands.entity(current_entity).remove::<ActiveWindow>();
        }

        commands.entity(window_focused.window).insert(ActiveWindow);
    }
}

#[derive(Component)]
pub struct StartupWindow;

#[derive(Component)]
pub struct ProjectWindow {
    pub project_editor_config: EditorProject,
}

#[derive(Component)]
pub struct WindowCamera {
    pub camera: Entity,
}

#[derive(Component)]
pub struct WindowUiRoot {
    pub root: Entity,
}

fn initial_window(mut commands: Commands) {
    commands.spawn((Window {
        resolution: WindowResolution::new(1280.0, 720.0),
        title: String::from("Crash Editor"),
        ..default()
    }, StartupWindow));
}

fn process_new_window(mut commands: Commands, spawned_windows: Query<Entity, Added<Window>>) {
    for window_entity in spawned_windows.iter() {
        let camera_id = commands.spawn(Camera2dBundle {
            camera: Camera {
                target: RenderTarget::Window(WindowRef::Entity(window_entity)),
                ..default()
            },
            ..default()
        }).id();

        let ui_root = commands.spawn((
            TargetCamera(camera_id),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        )).id();

        commands.entity(window_entity).insert((
            WindowCamera { camera: camera_id },
            WindowUiRoot { root: ui_root },
        ));
    }
}

fn check_for_exit(
    mut app_exit: EventWriter<AppExit>,
    window_query: Query<(), Or<(With<StartupWindow>, With<ProjectWindow>)>>
) {
    if window_query.is_empty() {
        app_exit.send(AppExit);
    }
}
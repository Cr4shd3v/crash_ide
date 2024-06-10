use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::utils::HashMap;
use bevy::window::{WindowCreated, WindowRef, WindowResolution};
use bevy::winit::WinitWindows;

pub(super) struct EditorWindowPlugin;

impl Plugin for EditorWindowPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, initial_window)
            .add_systems(PreUpdate, (process_new_window, save_resolution))
            .add_systems(PostUpdate, (despawn_window, check_for_exit))
            .init_resource::<DefaultWindowResolution>()
            .init_resource::<AllWindows>()
        ;
    }
}

#[derive(Component)]
pub struct StartupWindow;

#[derive(Component)]
pub struct ProjectWindow {
    pub project_crash_ide_config: Entity,
}

#[derive(Resource, Default)]
pub struct AllWindows {
    window_data: HashMap<Entity, WindowData>,
}

impl AllWindows {
    pub fn get(&self, window: &Entity) -> &WindowData {
        self.window_data.get(window).unwrap()
    }
}

pub struct WindowData {
    pub ui_root: Entity,
    pub camera: Entity,
}

fn initial_window(mut commands: Commands) {
    commands.spawn((Window {
        resolution: WindowResolution::new(1280.0, 720.0),
        title: String::from("Crash Editor"),
        ..default()
    }, StartupWindow));
}

#[derive(Resource, Default)]
pub struct DefaultWindowResolution(pub WindowResolution);

fn save_resolution(
    mut resolution: ResMut<DefaultWindowResolution>,
    mut event_reader: EventReader<WindowCreated>,
    winit_windows: NonSend<WinitWindows>,
) {
    for WindowCreated{ window } in event_reader.read() {
        let monitor_size = winit_windows.get_window(window.clone()).unwrap().current_monitor().unwrap().size();
        resolution.0 = WindowResolution::new(monitor_size.width as f32, monitor_size.height as f32);
    }
}

fn process_new_window(
    mut commands: Commands,
    spawned_windows: Query<Entity, Added<Window>>,
    mut all_windows: ResMut<AllWindows>,
) {
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

        all_windows.window_data.insert(window_entity, WindowData { ui_root, camera: camera_id });
    }
}

fn despawn_window(
    mut commands: Commands,
    mut all_windows: ResMut<AllWindows>,
    mut removed_windows: RemovedComponents<Window>,
) {
    for removed in removed_windows.read() {
        let data = all_windows.window_data.remove(&removed).unwrap();
        commands.entity(data.ui_root).despawn_recursive();
        commands.entity(data.camera).despawn();
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
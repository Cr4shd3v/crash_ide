use bevy::app::AppExit;
use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::utils::HashMap;
use bevy::window::{WindowCreated, WindowRef, WindowResolution};
use bevy::winit::WinitWindows;
use crash_ide_assets::DefaultColors;
use crash_ide_config::{EditorConfigProjects, GeneralSettings};
use crash_ide_project::{CloseProjectEvent, LoadedEditorProject, OpenProjectEvent};
use crash_ide_state::EditorState;
use crate::startup::StartupScreenState;

pub(super) struct EditorWindowPlugin;

impl Plugin for EditorWindowPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, initial_window)
            .add_systems(OnEnter(EditorState::Loaded), open_last_projects)
            .add_systems(PreUpdate, (process_new_window, save_resolution))
            .add_systems(Update, set_startup_window_resolution)
            .add_systems(PostUpdate, (despawn_window, check_for_exit, track_open_projects))
            .init_resource::<DefaultWindowResolution>()
            .init_resource::<AllWindows>()
        ;
    }
}

pub struct StartupWindow;

impl Component for StartupWindow {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_remove(|mut world: DeferredWorld, _targeted_entity, _component_id| {
            world.resource_mut::<NextState<StartupScreenState>>().set(StartupScreenState::None);
        });
    }
}

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

fn initial_window(
    mut commands: Commands,
) {
    commands.spawn((Window::default(), StartupWindow));
}

fn set_startup_window_resolution(
    mut query: Query<&mut Window, Added<StartupWindow>>,
    default_window_resolution: Res<DefaultWindowResolution>,
) {
    for mut window in query.iter_mut() {
        window.resolution = default_window_resolution.0.clone();
        window.resolution.set_physical_resolution(
            (default_window_resolution.0.width() * 0.75).round() as u32,
            (default_window_resolution.0.height() * 0.75).round() as u32,
        );
        window.position = WindowPosition::Centered(MonitorSelection::Current);
        window.title = "Crash Editor".to_string();
    }
}

fn open_last_projects(
    config: Res<EditorConfigProjects>,
    mut open_project: EventWriter<OpenProjectEvent>,
    startup_window: Query<Entity, With<StartupWindow>>,
    settings: Res<GeneralSettings>,
) {
    if !settings.open_last_project_on_startup {
        return;
    }

    let window_entity = startup_window.single();
    let mut window_id = Some(window_entity);

    for path in &config.last_opened {
        if let Some(project) = config.projects.get(path) {
            open_project.send(OpenProjectEvent {
                base_window: window_id,
                crash_ide_project: project.clone(),
            });

            window_id = None;
        }
    }
}

#[derive(Resource, Default)]
pub struct DefaultWindowResolution(pub WindowResolution);

fn save_resolution(
    mut resolution: ResMut<DefaultWindowResolution>,
    mut event_reader: EventReader<WindowCreated>,
    winit_windows: NonSend<WinitWindows>,
) {
    for WindowCreated { window } in event_reader.read() {
        let winit_window = winit_windows.get_window(window.clone()).unwrap();
        let monitor = winit_window.current_monitor().unwrap();
        let monitor_size = monitor.size();

        resolution.0 = WindowResolution::new(monitor_size.width as f32, monitor_size.height as f32);
    }
}

fn process_new_window(
    mut commands: Commands,
    mut spawned_windows: Query<Entity, Added<Window>>,
    mut all_windows: ResMut<AllWindows>,
) {
    for window_entity in spawned_windows.iter_mut() {
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
                background_color: BackgroundColor(DefaultColors::DEFAULT_BACKGROUND),
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
        app_exit.send_default();
    }
}

fn track_open_projects(
    new_project_er: EventReader<OpenProjectEvent>,
    close_project_er: EventReader<CloseProjectEvent>,
    query: Query<&ProjectWindow>,
    project_query: Query<&LoadedEditorProject>,
    mut config: ResMut<EditorConfigProjects>,
) {
    if new_project_er.is_empty() && close_project_er.is_empty() {
        return;
    }

    let mut projects = vec![];
    for window in query.iter() {
        projects.push(project_query.get(window.project_crash_ide_config).unwrap().crash_ide_project.path.clone());
    }

    config.last_opened = projects;
}
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::winit::WinitWindows;
use editor_config::EditorProject;
use crate::widget::screen::CreateProjectWindow;
use crate::window::{ProjectWindow, StartupWindow};

pub(super) struct OpenProjectPlugin;

impl Plugin for OpenProjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<OpenProjectEvent>()
            .add_systems(Update, on_open_project)
        ;
    }
}

#[derive(Event)]
pub struct OpenProjectEvent {
    editor_project: EditorProject,
    base_window: Option<Entity>,
    resolution_window: Entity,
}

impl OpenProjectEvent {
    pub fn new(editor_project: EditorProject, base_window: Option<Entity>, resolution_window: Entity) -> Self {
        Self {
            editor_project,
            base_window,
            resolution_window,
        }
    }
}

fn on_open_project(
    mut commands: Commands,
    mut event_reader: EventReader<OpenProjectEvent>,
    mut window_query_resize: Query<&mut Window>,
    winit_windows: NonSend<WinitWindows>,
) {
    for open_project_event in event_reader.read() {
        let monitor_size = winit_windows.get_window(open_project_event.resolution_window).unwrap().current_monitor().unwrap().size();
        let new_resolution = WindowResolution::new(monitor_size.width as f32, monitor_size.height as f32);

        if let Some(window) = open_project_event.base_window {
            commands.entity(window).remove::<(StartupWindow, CreateProjectWindow, ProjectWindow)>().insert(ProjectWindow {
                project_editor_config: open_project_event.editor_project.clone(),
            });

            window_query_resize.get_mut(window).unwrap().resolution = new_resolution;
        } else {
            commands.spawn((
                Window {
                    title: open_project_event.editor_project.name.clone(),
                    resolution: new_resolution,
                    ..default()
                },
                ProjectWindow {
                    project_editor_config: open_project_event.editor_project.clone(),
                }
            ));
        }
    }
}
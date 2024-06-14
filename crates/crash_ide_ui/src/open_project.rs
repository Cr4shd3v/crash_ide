use bevy::prelude::*;
use crash_ide_project::{LoadedEditorProject, OpenProjectEvent};
use crate::widget::screen::CreateProjectWindow;
use crate::window::{DefaultWindowResolution, ProjectWindow, StartupWindow};

pub(super) struct OpenProjectPlugin;

impl Plugin for OpenProjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, on_open_project)
        ;
    }
}

fn on_open_project(
    mut commands: Commands,
    mut event_reader: EventReader<OpenProjectEvent>,
    mut window_query_resize: Query<&mut Window>,
    default_window_resolution: Res<DefaultWindowResolution>,
) {
    for open_project_event in event_reader.read() {
        let project = commands.spawn(LoadedEditorProject {
            crash_ide_project: open_project_event.crash_ide_project.clone(),
        }).id();

        if let Some(window) = open_project_event.base_window {
            commands.entity(window).remove::<(StartupWindow, CreateProjectWindow, ProjectWindow)>().insert(ProjectWindow {
                project_crash_ide_config: project,
            });

            window_query_resize.get_mut(window).unwrap().resolution = default_window_resolution.0.clone();
        } else {
            commands.spawn((
                Window {
                    title: open_project_event.crash_ide_project.name.clone(),
                    resolution: default_window_resolution.0.clone(),
                    ..default()
                },
                ProjectWindow {
                    project_crash_ide_config: project,
                }
            ));
        }
    }
}
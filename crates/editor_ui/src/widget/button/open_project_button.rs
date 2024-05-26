use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::winit::WinitWindows;
use bevy_file_dialog::{DialogDirectoryPicked, FileDialogExt};
use editor_config::{EditorConfigProjects, EditorProject, HomeDir};
use crate::window::{ActiveWindow, ProjectWindow, StartupWindow};

#[derive(Component)]
pub(crate) struct OpenProjectButton;

pub struct OpenProjectDialog;

pub(super) fn open_project_button(
    mut commands: Commands,
    home_dir: Res<HomeDir>,
    interaction_query: Query<&Interaction, (With<OpenProjectButton>, Changed<Interaction>)>,
) {
    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                commands.dialog()
                    .set_directory(&home_dir.projects_path)
                    .set_title("Open Project")
                    .pick_directory_path::<OpenProjectDialog>();
            }
            _ => {}
        }
    }
}

pub(super) fn open_project_directory_picked(
    mut commands: Commands,
    mut folder_picked: EventReader<DialogDirectoryPicked<OpenProjectDialog>>,
    mut projects_config: ResMut<EditorConfigProjects>,
    winit_windows: NonSend<WinitWindows>,
    window_query: Query<Entity, With<ActiveWindow>>,
    window_query_2: Query<Option<&StartupWindow>>,
    mut window_query_resize: Query<&mut Window>,
) {
    for picked in folder_picked.read() {
        let picked_path = picked.path.to_str().unwrap().to_string();

        let config = if let Some(config) = projects_config.projects.iter()
            .find(|project| project.path == picked_path) {
            config.clone()
        } else {
            let config = EditorProject {
                path: picked_path,
                name: picked.path.file_name().unwrap().to_str().unwrap().to_string(),
            };

            projects_config.projects.push(config.clone());

            config
        };

        let window_entity = window_query.single();

        let monitor_size = winit_windows.get_window(window_entity).unwrap().current_monitor().unwrap().size();
        let new_resolution = WindowResolution::new(monitor_size.width as f32, monitor_size.height as f32);

        if window_query_2.get(window_entity).is_ok() {
            commands.entity(window_entity).remove::<StartupWindow>().insert(ProjectWindow {
                project_editor_config: config,
            });

            window_query_resize.get_mut(window_entity).unwrap().resolution = new_resolution;
        } else {
            commands.spawn((
                Window {
                    title: config.name.clone(),
                    resolution: new_resolution,
                    ..default()
                },
                ProjectWindow {
                    project_editor_config: config,
                }
            ));
        }
    }
}
use bevy::prelude::*;
use crash_ide_config::{EditorConfigProjects, EditorProject, HomeDir};
use crash_ide_file_picker::{DirectoryPicked, DirectoryPicker};
use crate::open_project::OpenProjectEvent;

#[derive(Component, Default)]
pub struct OpenProjectButton {
    pub base_window: Option<Entity>,
}

pub(super) fn open_project_button(
    mut commands: Commands,
    home_dir: Res<HomeDir>,
    interaction_query: Query<(&Interaction, Entity), (With<OpenProjectButton>, Changed<Interaction>)>,
) {
    for (interaction, entity) in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                commands.entity(entity).insert(DirectoryPicker {
                    title: "Open Project".to_string(),
                    start_directory: Some(home_dir.projects_path.clone()),
                });
            }
            _ => {}
        }
    }
}

pub(super) fn open_project_directory_picked(
    folder_query: Query<(&DirectoryPicked, &OpenProjectButton), Added<DirectoryPicked>>,
    mut projects_config: ResMut<EditorConfigProjects>,
    mut event_writer: EventWriter<OpenProjectEvent>,
) {
    for (picked, button) in folder_query.iter() {
        let picked_path = picked.0.path().to_str().unwrap().to_string();

        let config = if let Some(config) = projects_config.projects.get(&picked_path) {
            config.clone()
        } else {
            let config = EditorProject {
                name: picked.0.file_name(),
                path: picked_path.clone(),
            };

            projects_config.projects.insert(picked_path, config.clone());

            config
        };

        event_writer.send(OpenProjectEvent::new(config, button.base_window));
    }
}
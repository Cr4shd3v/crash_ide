use bevy::prelude::*;
use bevy_file_dialog::{DialogDirectoryPicked, FileDialogExt};
use editor_config::{EditorConfigProjects, EditorProject, HomeDir};

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
    mut folder_picked: EventReader<DialogDirectoryPicked<OpenProjectDialog>>,
    mut projects_config: ResMut<EditorConfigProjects>,
) {
    for picked in folder_picked.read() {
        let picked_path = picked.path.to_str().unwrap().to_string();

        if projects_config.projects.iter().any(|project| project.path == picked_path) {
            continue;
        }

        projects_config.projects.push(EditorProject {
            path: picked_path,
            name: picked.path.file_name().unwrap().to_str().unwrap().to_string(),
        });
    }
}
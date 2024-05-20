use bevy::prelude::*;
use bevy_file_dialog::{DialogDirectoryPicked, FileDialogExt};
use editor_config::{EditorConfigProjects, EditorProject, HomeDir};
use crate::file_dialog::OpenProjectDialog;

#[derive(Component)]
pub(crate) struct OpenProjectButton;

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
        projects_config.projects.push(EditorProject {
            path: picked.path.to_str().unwrap().to_string(),
            name: picked.path.file_name().unwrap().to_str().unwrap().to_string(),
        });
        println!("Hello");
    }
}
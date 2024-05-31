use std::fs;
use bevy::prelude::*;
use editor_config::FindProjectInParents;
use crate::editor::editor_left_menu::{FilePath, OpenFileEvent};
use crate::editor::main_editor_screen::ProjectsFileViews;
use crate::fonts::DefaultFonts;

pub(super) fn spawn_file_view(
    mut commands: Commands,
    mut event_reader: EventReader<OpenFileEvent>,
    file_path: FilePath,
    find_project_in_parents: FindProjectInParents,
    projects_file_views: Res<ProjectsFileViews>,
) {
    for event in event_reader.read() {
        let path = file_path.get_full_path(event.row_entity);
        let project = find_project_in_parents.find_project_ref(event.row_entity);
        let view = projects_file_views.get(project);

        let content = fs::read_to_string(path).unwrap();

        commands.entity(view).despawn_descendants().with_children(|parent| {
            parent.spawn(TextBundle::from_section(content, TextStyle {
                font: DefaultFonts::JETBRAINS_MONO_REGULAR,
                font_size: 18.0,
                ..default()
            }));
        });
    }
}
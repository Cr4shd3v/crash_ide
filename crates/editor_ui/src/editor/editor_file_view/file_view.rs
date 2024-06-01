use std::fs;
use bevy::prelude::*;
use editor_config::FindProjectInParents;
use editor_widget::{TextInputBundle, TextInputSettings, TextInputTextStyle, TextInputValue};
use crate::editor::editor_left_menu::{FilePath, OpenFileEvent};
use crate::editor::main_editor_screen::ProjectsFileViews;
use crate::fonts::DefaultFonts;

#[derive(Component)]
pub(super) struct FileViewInstance {
    path: String,
}

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

        let content = fs::read_to_string(&path).unwrap();

        commands.entity(view).despawn_descendants().with_children(|parent| {
            parent.spawn((TextInputBundle {
                text_input_value: TextInputValue(content),
                text_input_text_style: TextInputTextStyle(TextStyle {
                    font: DefaultFonts::JETBRAINS_MONO_REGULAR,
                    font_size: 18.0,
                    ..default()
                }),
                text_input_settings: TextInputSettings {
                    with_border: false,
                    multiline: true,
                },
                ..default()
            }, NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            }, FileViewInstance {
                path,
            }));
        });
    }
}

pub(super) fn save_edited_content(
    query: Query<(&TextInputValue, &FileViewInstance), Changed<TextInputValue>>
) {
    for (input_value, view_instance) in query.iter() {
        fs::write(&view_instance.path, &input_value.0).unwrap();
    }
}
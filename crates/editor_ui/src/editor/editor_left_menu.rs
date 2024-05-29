use bevy::prelude::*;
use editor_config::{ProjectRef, Projects};
use crate::editor::main_editor_screen::EditorLeftMenu;
use crate::fonts::DefaultFonts;

pub struct EditorLeftMenuPlugin;

impl Plugin for EditorLeftMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_left_menu)
        ;
    }
}

fn spawn_left_menu(
    mut commands: Commands,
    query: Query<(Entity, &ProjectRef), Added<EditorLeftMenu>>,
    projects: Projects,
) {
    for (entity, project_ref) in query.iter() {
        let project = projects.get_by_ref(project_ref);

        commands.entity(entity).despawn_descendants().with_children(|parent| {
            parent.spawn(TextBundle::from_section(project.editor_project.path.clone(), TextStyle {
                font: DefaultFonts::ROBOTO_REGULAR,
                font_size: 14.0,
                ..default()
            }));
        });
    }
}
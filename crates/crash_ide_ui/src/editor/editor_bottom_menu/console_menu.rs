use std::path::PathBuf;
use bevy::prelude::*;
use crash_ide_config::FindProjectInParents;

use crash_ide_console::Console;

use crate::editor::main_editor_screen::EditorBottomMenu;

pub(super) struct ConsoleMenuPlugin;

impl Plugin for ConsoleMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_console_menu)
        ;
    }
}

fn spawn_console_menu(
    mut commands: Commands,
    query: Query<Entity, Added<EditorBottomMenu>>,
    find_project_in_parents: FindProjectInParents,
) {
    for entity in query.iter() {
        let project = find_project_in_parents.find(entity);

        commands.entity(entity).despawn_descendants().with_children(|parent| {
            parent.spawn((Console {
                start_dir: PathBuf::from(&project.crash_ide_project.path),
            }, NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            }));
        });
    }
}
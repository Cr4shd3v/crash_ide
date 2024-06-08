use bevy::prelude::*;

use editor_console::Console;

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
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_descendants().with_children(|parent| {
            parent.spawn((Console, NodeBundle {
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
use bevy::prelude::*;
use crate::window::{ProjectWindow, WindowUiRoot};

pub(super) fn spawn_main_editor_screen(
    mut commands: Commands,
    window_query: Query<(&WindowUiRoot, &ProjectWindow), Added<ProjectWindow>>,
) {
    for (ui_root, project_window) in window_query.iter() {
        commands.entity(ui_root.root).despawn_descendants().with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("Editor at {}", project_window.project_editor_config.path),
                TextStyle::default(),
            ));
        });
    }
}
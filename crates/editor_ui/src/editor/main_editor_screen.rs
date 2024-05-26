use bevy::prelude::*;
use crate::window::{ActiveWindow, WindowUiRoot};

pub(super) fn spawn_main_editor_screen(
    mut commands: Commands,
    window_query: Query<&WindowUiRoot, With<ActiveWindow>>,
) {
    let ui_root = window_query.single();

    commands.entity(ui_root.root).despawn_descendants().with_children(|parent| {
        parent.spawn(TextBundle::from_section("Editor", TextStyle::default()));
    });
}
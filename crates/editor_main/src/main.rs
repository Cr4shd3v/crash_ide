use bevy::prelude::*;
use bevy::window::{ExitCondition, WindowResolution};

use editor_config::EditorConfigPlugin;
use editor_state::EditorStatePlugin;
use editor_ui::EditorUiPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(1280.0, 720.0),
            title: String::from("Crash Editor"),
            ..default()
        }),
        exit_condition: ExitCondition::OnPrimaryClosed,
        ..default()
    }));

    app.add_plugins(EditorStatePlugin);
    app.add_plugins(EditorConfigPlugin);
    app.add_plugins(EditorUiPlugin);

    app.run();
}

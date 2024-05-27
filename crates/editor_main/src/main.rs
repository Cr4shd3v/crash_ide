//! Starting point of the editor

#![warn(missing_docs)]

use std::env::current_dir;
use bevy::prelude::*;
use bevy::window::ExitCondition;

use editor_config::EditorConfigPlugin;
use editor_state::EditorStatePlugin;
use editor_ui::EditorUiPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: None,
        exit_condition: ExitCondition::DontExit,
        ..default()
    }).set(AssetPlugin {
        file_path: format!("{}/assets", current_dir().unwrap().to_str().unwrap()),
        ..default()
    }));

    app.add_plugins(EditorStatePlugin);
    app.add_plugins(EditorConfigPlugin);
    app.add_plugins(EditorUiPlugin);

    app.run();
}

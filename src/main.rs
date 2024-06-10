//! Starting point of the editor

#![warn(missing_docs)]

use bevy::prelude::*;
use bevy::window::ExitCondition;
use crash_ide_assets::EditorAssetPlugin;

use crash_ide_config::EditorConfigPlugin;
use crash_ide_console::EditorConsolePlugin;
use crash_ide_file::EditorFilePlugin;
use crash_ide_file_picker::EditorFilePickerPlugin;
use crash_ide_state::EditorStatePlugin;
use crash_ide_ui::EditorUiPlugin;
use crash_ide_widget::WidgetPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: None,
        exit_condition: ExitCondition::DontExit,
        ..default()
    }).set(AssetPlugin {
        #[cfg(all(not(debug_assertions), target_os = "linux"))]
        file_path: "/var/lib/crash-ide".to_string(),
        ..default()
    }));

    app.add_plugins(EditorStatePlugin);
    app.add_plugins(EditorConfigPlugin);
    app.add_plugins(EditorUiPlugin);
    app.add_plugins(EditorFilePlugin);
    app.add_plugins(EditorAssetPlugin);
    app.add_plugins(WidgetPlugin);
    app.add_plugins(EditorFilePickerPlugin);
    app.add_plugins(EditorConsolePlugin);

    app.run();
}

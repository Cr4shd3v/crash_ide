//! Starting point of the editor

#![warn(missing_docs)]

use bevy::prelude::*;
use bevy::window::ExitCondition;
use crash_ide_assets::CrashIDEAssetPlugin;

use crash_ide_config::CrashIDEConfigPlugin;
use crash_ide_console::CrashIDEConsolePlugin;
use crash_ide_file::CrashIDEFilePlugin;
use crash_ide_file_picker::CrashIDEFilePickerPlugin;
use crash_ide_project::CrashIDEProjectPlugin;
use crash_ide_state::CrashIDEStatePlugin;
use crash_ide_ui::CrashIDEUiPlugin;
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

    app.add_plugins(CrashIDEStatePlugin);
    app.add_plugins(CrashIDEConfigPlugin);
    app.add_plugins(CrashIDEUiPlugin);
    app.add_plugins(CrashIDEFilePlugin);
    app.add_plugins(CrashIDEAssetPlugin);
    app.add_plugins(WidgetPlugin);
    app.add_plugins(CrashIDEFilePickerPlugin);
    app.add_plugins(CrashIDEConsolePlugin);
    app.add_plugins(CrashIDEProjectPlugin);

    app.run();
}

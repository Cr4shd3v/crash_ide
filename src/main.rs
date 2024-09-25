//! Starting point of the editor

#![warn(missing_docs)]

use bevy::prelude::*;
use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;
use bevy::window::ExitCondition;
use crash_ide_assets::CrashIDEAssetPlugin;
use crash_ide_clipboard::CrashIDEClipboardPlugin;
use crash_ide_code_view::CrashIDECodeViewPlugin;
use crash_ide_config::CrashIDEConfigPlugin;
use crash_ide_console::CrashIDEConsolePlugin;
use crash_ide_discord_rpc::CrashIDEDiscordRpcPlugin;
use crash_ide_file::CrashIDEFilePlugin;
use crash_ide_file_picker::CrashIDEFilePickerPlugin;
use crash_ide_file_watcher::CrashIDEFileWatcherPlugin;
use crash_ide_notification::CrashIDENotificationPlugin;
use crash_ide_project::CrashIDEProjectPlugin;
use crash_ide_state::CrashIDEStatePlugin;
use crash_ide_text_input::CrashIDETextInputPlugin;
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
    }).disable::<PipelinedRenderingPlugin>());

    app.add_plugins(CrashIDEStatePlugin);
    app.add_plugins(CrashIDEConfigPlugin);
    app.add_plugins(CrashIDEUiPlugin);
    app.add_plugins(CrashIDEFilePlugin);
    app.add_plugins(CrashIDEAssetPlugin);
    app.add_plugins(WidgetPlugin);
    app.add_plugins(CrashIDEFilePickerPlugin);
    app.add_plugins(CrashIDEConsolePlugin);
    app.add_plugins(CrashIDEProjectPlugin);
    app.add_plugins(CrashIDENotificationPlugin);
    app.add_plugins(CrashIDEClipboardPlugin);
    app.add_plugins(CrashIDEFileWatcherPlugin);
    app.add_plugins(CrashIDECodeViewPlugin);
    app.add_plugins(CrashIDETextInputPlugin);

    // Built-in Plugins
    app.add_plugins(CrashIDEDiscordRpcPlugin);

    app.run();
}

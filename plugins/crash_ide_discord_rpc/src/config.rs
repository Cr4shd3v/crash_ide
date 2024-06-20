use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crash_ide_assets::DefaultFonts;
use crash_ide_config::{ConfigAppExt, EditorConfig, PluginConfig};
use crash_ide_ui::settings::{PluginSettingsMenuMarker, SettingsMenuRegistryExt};
use crash_ide_widget::{Checkbox, CheckboxBundle, CheckboxLabel};

pub(super) struct DiscordRpcConfigPlugin;

impl Plugin for DiscordRpcConfigPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_plugin_config::<DiscordRpcConfig>()
            .register_settings_menu_entry::<DiscordRpcConfig>()
            .add_systems(Update, (
                spawn_discord_rpc_setting_menu, handle_active_checkbox,
                handle_show_project_checkbox, handle_show_filename_checkbox,
            ))
        ;
    }
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct DiscordRpcConfig {
    pub active: bool,
    pub show_project: bool,
    pub show_filename: bool,
}

impl EditorConfig for DiscordRpcConfig {
    const FILENAME: &'static str = "discord_rpc.json";
}

impl PluginConfig for DiscordRpcConfig {
    const DISPLAY_NAME: &'static str = "Discord RPC";
}

#[derive(Component)]
struct DiscordRpcActiveCheckbox;

#[derive(Component)]
struct DiscordRpcShowProjectCheckbox;

#[derive(Component)]
struct DiscordRpcShowFilenameCheckbox;

fn spawn_discord_rpc_setting_menu(
    mut commands: Commands,
    query: Query<(Entity, &PluginSettingsMenuMarker), Added<PluginSettingsMenuMarker>>,
    settings: Res<DiscordRpcConfig>,
) {
    for (entity, menu_marker) in query.iter() {
        if menu_marker.0 != DiscordRpcConfig::DISPLAY_NAME {
            continue;
        }

        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                CheckboxBundle {
                    checkbox: Checkbox::from_bool(settings.active),
                    label: CheckboxLabel::End(TextSection::new("Active", TextStyle {
                        font: DefaultFonts::ROBOTO_REGULAR,
                        font_size: 18.0,
                        ..default()
                    })),
                    ..default()
                },
                DiscordRpcActiveCheckbox,
            ));

            parent.spawn((
                CheckboxBundle {
                    checkbox: Checkbox::from_bool(settings.show_project),
                    label: CheckboxLabel::End(TextSection::new("Show project", TextStyle {
                        font: DefaultFonts::ROBOTO_REGULAR,
                        font_size: 18.0,
                        ..default()
                    })),
                    ..default()
                },
                DiscordRpcShowProjectCheckbox,
            ));

            parent.spawn((
                CheckboxBundle {
                    checkbox: Checkbox::from_bool(settings.show_filename),
                    label: CheckboxLabel::End(TextSection::new("Show filename", TextStyle {
                        font: DefaultFonts::ROBOTO_REGULAR,
                        font_size: 18.0,
                        ..default()
                    })),
                    ..default()
                },
                DiscordRpcShowFilenameCheckbox,
            ));
        });
    }
}

macro_rules! checkbox_handle {
    ($name:ident, $settings_type:ty, $marker:ty, $field:tt) => {
        fn $name(
            query: Query<&Checkbox, (Changed<Checkbox>, With<$marker>)>,
            mut settings: ResMut<$settings_type>,
        ) {
            for checkbox in query.iter() {
                settings.$field = checkbox.is_checked();
            }
        }
    };
}

checkbox_handle!(handle_active_checkbox, DiscordRpcConfig, DiscordRpcActiveCheckbox, active);
checkbox_handle!(handle_show_project_checkbox, DiscordRpcConfig, DiscordRpcShowProjectCheckbox, show_project);
checkbox_handle!(handle_show_filename_checkbox, DiscordRpcConfig, DiscordRpcShowFilenameCheckbox, show_filename);
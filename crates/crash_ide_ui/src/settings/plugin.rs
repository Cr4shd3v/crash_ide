use bevy::prelude::*;
use crash_ide_config::PluginConfig;

pub(super) struct SettingsMenuRegistryPlugin;

impl Plugin for SettingsMenuRegistryPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SettingsMenuPluginRegistry>()
        ;
    }
}

/// Marking the base of a plugin settings menu.
#[derive(Component)]
pub struct PluginSettingsMenuMarker(pub String);

/// Handles which menus needs to be shown.
#[derive(Resource, Default)]
pub struct SettingsMenuPluginRegistry {
    pub(super) entries: Vec<SettingsMenuPluginRegistryEntry>,
}

impl SettingsMenuPluginRegistry {
    /// Adds an entry to the settings menu
    pub fn add_entry(&mut self, name: String) {
        self.entries.push(SettingsMenuPluginRegistryEntry {
            name,
        });
    }
}

pub(super) struct SettingsMenuPluginRegistryEntry {
    pub(super) name: String,
}

/// Extension for [App] to register a new settings menu.
pub trait SettingsMenuRegistryExt {
    /// Register a new settings menu.
    ///
    /// The spawned menu will be marked with [PluginSettingsMenuMarker] with `name`.
    fn register_settings_menu_entry<T: PluginConfig>(&mut self) -> &mut Self;
}

impl SettingsMenuRegistryExt for App {
    fn register_settings_menu_entry<T: PluginConfig>(&mut self) -> &mut Self {
        self.add_systems(Startup, add_registry_entry(T::DISPLAY_NAME.to_string()));

        self
    }
}

fn add_registry_entry(name: String) -> impl FnMut(ResMut<SettingsMenuPluginRegistry>) {
    move |mut registry: ResMut<SettingsMenuPluginRegistry>| {
        registry.add_entry(name.clone());
    }
}
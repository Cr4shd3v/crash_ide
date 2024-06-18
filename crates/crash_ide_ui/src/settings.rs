use bevy::prelude::*;
use crash_ide_assets::DefaultFonts;
use crate::window::AllWindows;

pub(super) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_settings_screen, settings_window))
        ;
    }
}

#[derive(Component)]
pub struct SettingsWindow;

fn settings_window(
    mut commands: Commands,
    query: Query<Entity, Added<SettingsWindow>>,
    all_windows: Res<AllWindows>,
) {
    for entity in query.iter() {
        commands.entity(all_windows.get(&entity).ui_root).despawn_descendants().with_children(|parent| {
            parent.spawn((NodeBundle::default(), SettingsScreen));
        });
    }
}

#[derive(Component)]
pub struct SettingsScreen;

fn spawn_settings_screen(
    mut commands: Commands,
    query: Query<Entity, Added<SettingsScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_descendants().with_children(|parent| {
            parent.spawn(TextBundle::from_section("Settings", TextStyle {
                font: DefaultFonts::ROBOTO_REGULAR,
                font_size: 18.0,
                ..default()
            }));
        });
    }
}
use bevy::prelude::*;
use crate::settings::SettingsWindow;

pub(super) struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, open_settings_menu)
        ;
    }
}

#[derive(Component)]
pub(super) struct SettingsMenu;

fn open_settings_menu(
    mut commands: Commands,
    query: Query<&Interaction, (With<SettingsMenu>, Changed<Interaction>)>,
    mut settings_window: Query<&mut Window, With<SettingsWindow>>,
) {
    for interaction in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Ok(mut window) = settings_window.get_single_mut() {
            window.focused = true;
        } else {
            commands.spawn((
                Window {
                    title: "Settings".to_string(),
                    ..default()
                },
                SettingsWindow,
            ));
        }
    }
}
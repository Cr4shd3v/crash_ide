use bevy::prelude::*;
use crash_ide_assets::DefaultFonts;

pub(super) struct GeneralSettingsMenuPlugin;

impl Plugin for GeneralSettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_general_settings_menu)
        ;
    }
}

#[derive(Component)]
pub struct GeneralSettingsMenu;

fn spawn_general_settings_menu(
    mut commands: Commands,
    query: Query<Entity, Added<GeneralSettingsMenu>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_descendants().with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("test", TextStyle {
                    font: DefaultFonts::ROBOTO_REGULAR,
                    font_size: 18.0,
                    ..default()
                }),
                ..default()
            });
        });
    }
}

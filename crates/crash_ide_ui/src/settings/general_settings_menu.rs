use bevy::prelude::*;
use crash_ide_assets::DefaultFonts;
use crash_ide_config::GeneralSettings;
use crash_ide_widget::{Checkbox, CheckboxBundle, CheckboxLabel};

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
    settings: Res<GeneralSettings>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_descendants().with_children(|parent| {
            parent.spawn((
                CheckboxBundle {
                    checkbox: Checkbox::from_bool(settings.open_last_project_on_startup),
                    label: CheckboxLabel::End(TextSection::new("Open last opened project on startup", TextStyle {
                        font: DefaultFonts::ROBOTO_REGULAR,
                        font_size: 18.0,
                        ..default()
                    })),
                    ..default()
                }
            ));
        });
    }
}

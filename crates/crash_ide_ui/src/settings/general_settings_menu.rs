use bevy::prelude::*;
use crash_ide_assets::DefaultFonts;
use crash_ide_config::GeneralSettings;
use crash_ide_widget::{Checkbox, CheckboxBundle, CheckboxLabel};

pub(super) struct GeneralSettingsMenuPlugin;

impl Plugin for GeneralSettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_general_settings_menu, update_settings))
        ;
    }
}

#[derive(Component)]
pub struct GeneralSettingsMenu;

#[derive(Component)]
struct OpenLastProjectCheckbox;

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
                        font_size: 14.0,
                        ..default()
                    })),
                    ..default()
                },
                OpenLastProjectCheckbox,
            ));
        });
    }
}

fn update_settings(
    open_last_project_query: Query<&Checkbox, (With<OpenLastProjectCheckbox>, Changed<Checkbox>)>,
    mut settings: ResMut<GeneralSettings>,
) {
    for checkbox in open_last_project_query.iter() {
        settings.open_last_project_on_startup = checkbox.is_checked();
    }
}

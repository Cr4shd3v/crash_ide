use bevy::prelude::*;
use crate::editor::main_editor_screen::EditorFileView;
use editor_assets::DefaultFonts;

pub(super) fn spawn_default_file_view(
    mut commands: Commands,
    query: Query<Entity, Added<EditorFileView>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_descendants().with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section("No File opened", TextStyle {
                        font: DefaultFonts::ROBOTO_REGULAR,
                        font_size: 16.0,
                        color: Color::GRAY.with_a(0.5),
                    }),
                    ..default()
                });
            });
        });
    }
}
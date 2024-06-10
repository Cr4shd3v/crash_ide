use bevy::prelude::*;

use editor_assets::DefaultFonts;
use editor_widget::Hoverable;

use crate::editor::editor_top_menu::file_menu::FileMenu;
use crate::editor::main_editor_screen::EditorTopMenu;

pub(super) fn spawn_top_menu(
    mut commands: Commands,
    query: Query<Entity, Added<EditorTopMenu>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_descendants().with_children(|parent| {
            // Base margin
            parent.spawn(NodeBundle {
                style: Style {
                    margin: UiRect::left(Val::Vw(1.0)),
                    ..default()
                },
                ..default()
            });

            // menu entry, will be a widget
            parent.spawn((NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    height: Val::Percent(100.0),
                    padding: UiRect::horizontal(Val::Vw(0.5)),
                    margin: UiRect::horizontal(Val::Vw(0.1)),
                    ..default()
                },
                ..default()
            },
                          Hoverable::new(Color::GRAY.with_a(0.2)),
                          Interaction::None,
                          FileMenu,
            )).with_children(|parent| {
                parent.spawn(TextBundle::from_section("File", TextStyle {
                    font: DefaultFonts::ROBOTO_REGULAR,
                    font_size: 18.0,
                    ..default()
                }));
            });

            parent.spawn((NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    height: Val::Percent(100.0),
                    padding: UiRect::horizontal(Val::Vw(0.5)),
                    margin: UiRect::horizontal(Val::Vw(0.1)),
                    ..default()
                },
                ..default()
            }, Hoverable::new(Color::GRAY.with_a(0.2)), Interaction::None)).with_children(|parent| {
                parent.spawn(TextBundle::from_section("Help", TextStyle {
                    font: DefaultFonts::ROBOTO_REGULAR,
                    font_size: 18.0,
                    ..default()
                }));
            });
        });
    }
}
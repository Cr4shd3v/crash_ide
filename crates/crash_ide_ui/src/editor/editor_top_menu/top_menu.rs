use bevy::prelude::*;

use crash_ide_assets::DefaultFonts;
use crash_ide_widget::Hoverable;

use crate::editor::editor_top_menu::file_menu::FileMenu;
use crate::editor::editor_top_menu::help_menu::HelpMenu;
use crate::editor::editor_top_menu::settings_menu::SettingsMenu;
use crate::editor::main_editor_screen::EditorTopMenu;

pub(super) fn spawn_top_menu(
    mut commands: Commands,
    query: Query<Entity, Added<EditorTopMenu>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_descendants().with_children(|parent| {
            spawn_menu_entry(parent, FileMenu, "File");
            spawn_menu_entry(parent, HelpMenu, "Help");
            spawn_menu_entry(parent, SettingsMenu, "Settings");
        });
    }
}

fn spawn_menu_entry(parent: &mut ChildBuilder, marker: impl Bundle, title: &str) {
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
                  marker,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(title, TextStyle {
            font: DefaultFonts::ROBOTO_REGULAR,
            font_size: 18.0,
            ..default()
        }));
    });
}

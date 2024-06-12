use std::ops::Neg;

use bevy::prelude::*;

use crash_ide_assets::DefaultFonts;

use crate::widget::context_menu::ContextMenu;

pub(super) struct FileMenuPlugin;

impl Plugin for FileMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, open_file_menu)
        ;
    }
}

#[derive(Component)]
pub(super) struct FileMenu;

fn open_file_menu(
    mut commands: Commands,
    query: Query<(&Interaction, &Node, &Style, Entity), (With<FileMenu>, Changed<Interaction>)>
) {
    for (interaction, node, style, entity) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let size = node.size();

        commands.entity(entity).with_children(|parent| {
            parent.spawn(ContextMenu::new_top(size.y, style.margin.left.neg())).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section("test text", TextStyle {
                        font: DefaultFonts::ROBOTO_REGULAR,
                        font_size: 18.0,
                        ..default()
                    }).with_no_wrap(),
                    ..default()
                });
            });
        });
    }
}


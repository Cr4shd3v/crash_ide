use std::ops::Neg;

use bevy::prelude::*;
use crash_ide_widget::ActiveWindow;

use crate::widget::button::{CloseProjectButton, CreateProjectButton, OpenProjectButton};

use crate::widget::context_menu::{ContextMenu, ContextMenuRow};

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
    query: Query<(&Interaction, &Node, &Style, Entity), (With<FileMenu>, Changed<Interaction>)>,
    window_query: Query<Entity, With<ActiveWindow>>,
) {
    for (interaction, node, style, entity) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let size = node.size();

        commands.entity(entity).with_children(|parent| {
            parent.spawn(ContextMenu::new_top(size.y, style.margin.left.neg())).with_children(|parent| {
                ContextMenuRow::new(parent, "New Project", CreateProjectButton::default(), None);
                ContextMenuRow::new(parent, "Open Project", OpenProjectButton::default(), None);
                ContextMenuRow::new(parent, "Close Project", CloseProjectButton { window_entity: window_query.single() }, None);
            });
        });
    }
}


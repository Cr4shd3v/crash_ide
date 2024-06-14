use std::ops::Neg;

use bevy::prelude::*;

use crash_ide_assets::{DefaultFonts, DefaultIcons};

use crate::widget::button::{GithubButton, GithubIssueButton};
use crate::widget::context_menu::{ContextMenu, ContextMenuRow};

pub(super) struct HelpMenuPlugin;

impl Plugin for HelpMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, open_help_menu)
        ;
    }
}

#[derive(Component)]
pub(super) struct HelpMenu;

fn open_help_menu(
    mut commands: Commands,
    query: Query<(&Interaction, &Node, &Style, Entity), (With<HelpMenu>, Changed<Interaction>)>,
    icons: Res<DefaultIcons>,
) {
    for (interaction, node, style, entity) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let size = node.size();

        commands.entity(entity).with_children(|parent| {
            parent.spawn(ContextMenu::new_top(size.y, style.margin.left.neg())).with_children(|parent| {
                ContextMenuRow::new(parent, "Github", GithubButton, Some(icons.github.clone()), None);
                ContextMenuRow::new(parent, "Create issue", GithubIssueButton, Some(icons.github.clone()), None);

                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::axes(Val::Px(6.0), Val::Px(3.0)),
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(format!("v{}", env!("CARGO_PKG_VERSION")), TextStyle {
                            font: DefaultFonts::ROBOTO_REGULAR,
                            font_size: 16.0,
                            color: Color::GRAY.with_a(0.8),
                        }),
                        ..default()
                    });
                });
            });
        });
    }
}

use bevy::prelude::*;
use crash_ide_assets::{DefaultColors, DefaultFonts};
use crash_ide_widget::{TextInputBundle, TextInputTextStyle};
use crate::widget::context_menu::ContextMenu;

#[derive(Component)]
pub struct FilenameDialogConfirmButton {
    pub input_id: Entity,
}

pub struct FilenameDialog;

impl FilenameDialog {
    pub fn new(parent: &mut ChildBuilder, window: &Window, marker: impl Bundle, title: &str, button_title: &str) {
        parent.spawn(
            ContextMenu::new_top(window.resolution.height() / 2.0 - 50.0, Val::Px(window.resolution.width() / 2.0 - 150.0)),
        ).with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(300.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(title, TextStyle {
                        font: DefaultFonts::ROBOTO_REGULAR,
                        font_size: 18.0,
                        ..default()
                    }),
                    style: Style {
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                });

                let input_id = parent.spawn((
                    TextInputBundle {
                        text_input_text_style: TextInputTextStyle::default().with_font(DefaultFonts::ROBOTO_REGULAR),
                        ..default()
                    },
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(95.0),
                            ..default()
                        },
                        ..default()
                    },
                )).id();

                parent.spawn(NodeBundle {
                    style: Style {
                        margin: UiRect::top(Val::Px(5.0)),
                        flex_direction: FlexDirection::RowReverse,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(5.0)),
                                margin: UiRect::right(Val::Percent(2.5)),
                                ..default()
                            },
                            background_color: BackgroundColor(DefaultColors::PRIMARY_BUTTON),
                            ..default()
                        },
                        Interaction::None,
                        Button,
                        marker,
                        FilenameDialogConfirmButton {
                            input_id,
                        },
                    )).with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section(button_title, TextStyle {
                                font: DefaultFonts::ROBOTO_REGULAR,
                                font_size: 18.0,
                                ..default()
                            }),
                            ..default()
                        });
                    });
                });
            });
        });
    }
}
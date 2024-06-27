use bevy::prelude::*;
use crate::{CodeView, CodeViewContent, CodeViewStyle};
use crate::component::CodeViewLine;

pub(super) fn create_code_view(
    mut commands: Commands,
    mut query: Query<(Entity, &CodeViewStyle, &CodeViewContent, &mut Style), Added<CodeView>>,
) {
    for (entity, code_view_style, content, mut style) in query.iter_mut() {
        style.width = Val::Percent(100.0);
        style.flex_direction = FlexDirection::Column;

        commands.entity(entity).with_children(|parent| {
            for (index, line) in content.lines.iter().enumerate() {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            height: Val::Px(code_view_style.font_size + 2.0),
                            ..default()
                        },
                        ..default()
                    },
                    CodeViewLine {
                        line_index: index,
                    },
                )).with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            border: UiRect::right(Val::Px(1.0)),
                            margin: UiRect::right(Val::Px(3.0)),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::GRAY.with_a(0.1)),
                        ..default()
                    }).with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section((index + 1).to_string(), TextStyle {
                                font: code_view_style.regular_font.clone(),
                                font_size: code_view_style.font_size,
                                ..default()
                            }),
                            style: Style {
                                width: Val::Px(code_view_style.font_size * 1.5),
                                margin: UiRect::new(Val::Px(10.0), Val::Px(15.0), Val::ZERO, Val::ZERO),
                                ..default()
                            },
                            ..default()
                        });
                    });

                    for token in line {
                        parent.spawn(TextBundle {
                            text: Text::from_section(&token.content, TextStyle {
                                font: code_view_style.get_font_for_token(token),
                                font_size: code_view_style.font_size,
                                ..default()
                            }).with_no_wrap(),
                            ..default()
                        });
                    }
                });
            }
        });
    }
}
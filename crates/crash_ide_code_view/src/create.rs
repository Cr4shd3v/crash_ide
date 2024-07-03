use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use crate::{CodeView, CodeViewContainer, CodeViewContent, CodeViewLineContainer, CodeViewStyle};
use crate::component::CodeViewLine;

pub(super) fn create_code_view(
    mut commands: Commands,
    mut query: Query<(Entity, &CodeViewStyle, &CodeViewContent, &mut Style), Added<CodeView>>,
) {
    for (entity, code_view_style, content, mut style) in query.iter_mut() {
        style.width = Val::Percent(100.0);
        style.flex_direction = FlexDirection::Row;

        let line_count_container = commands.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            for (index, _) in content.lines.iter().enumerate() {
                parent.spawn(NodeBundle {
                    style: Style {
                        border: UiRect::right(Val::Px(1.0)),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    border_color: BorderColor(Color::GRAY.with_a(0.1)),
                    ..default()
                }).with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            width: Val::Px(code_view_style.font_size * 1.5),
                            height: Val::Px(code_view_style.font_size + 2.0),
                            margin: UiRect::new(Val::Px(10.0), Val::Px(15.0), Val::ZERO, Val::ZERO),
                            ..default()
                        },
                        ..default()
                    }).with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section((index + 1).to_string(), TextStyle {
                                font: code_view_style.regular_font.clone(),
                                font_size: code_view_style.font_size,
                                ..default()
                            }),
                            ..default()
                        });
                    });
                });
            }
        }).id();

        let line_content_container = commands.spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                ..default()
            },
            ..default()
        }, Interaction::None, RelativeCursorPosition::default(), CodeViewContainer)).with_children(|parent| {
            for (index, line) in content.lines.iter().enumerate() {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            height: Val::Px(code_view_style.font_size + 2.0),
                            padding: UiRect::left(Val::Px(3.0)),
                            ..default()
                        },
                        ..default()
                    },
                    CodeViewLine {
                        line_index: index,
                    },
                )).with_children(|parent| {
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
        }).id();

        commands.entity(entity).despawn_descendants().push_children(&[line_count_container, line_content_container]).insert(CodeViewLineContainer {
            line_content_container,
            line_count_container,
        });
    }
}
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use crash_ide_assets::DefaultColors;
use crate::{CodeView, CodeViewContainer, CodeViewContent, CodeViewContentLine, CodeViewLineContainer, CodeViewStyle};
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
                build_line_count(parent, code_view_style, index + 1);
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
            for line in content.lines.iter() {
                build_line_parent(parent, code_view_style, line);
            }
        }).id();

        commands.entity(entity).despawn_descendants().push_children(&[line_count_container, line_content_container]).insert(CodeViewLineContainer {
            line_content_container,
            line_count_container,
        });
    }
}

pub(crate) fn build_line_command(commands: &mut Commands, code_view_style: &CodeViewStyle, line: &CodeViewContentLine) -> Entity {
    commands.spawn(create_line_bundle(code_view_style)).with_children(|parent| {
        spawn_lines(parent, code_view_style, line);
    }).id()
}

pub(crate) fn build_line_parent(parent: &mut ChildBuilder, code_view_style: &CodeViewStyle, line: &CodeViewContentLine) {
    parent.spawn(create_line_bundle(code_view_style)).with_children(|parent| {
        spawn_lines(parent, code_view_style, line);
    });
}

fn create_line_bundle(code_view_style: &CodeViewStyle) -> impl Bundle {
    (
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
        CodeViewLine,
    )
}

fn spawn_lines(parent: &mut ChildBuilder, code_view_style: &CodeViewStyle, line: &CodeViewContentLine) {
    for token in line.tokens.iter() {
        parent.spawn(TextBundle {
            text: Text::from_section(&token.content, TextStyle {
                font: code_view_style.get_font_for_token(token),
                font_size: code_view_style.font_size,
                ..default()
            }).with_no_wrap(),
            ..default()
        });
    }
}

pub(crate) fn build_line_count(parent: &mut ChildBuilder, code_view_style: &CodeViewStyle, line: usize) {
    parent.spawn(NodeBundle {
        style: Style {
            border: UiRect::right(Val::Px(1.0)),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        border_color: BorderColor(DefaultColors::GRAY.with_alpha(0.1)),
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
                text: Text::from_section(line.to_string(), TextStyle {
                    font: code_view_style.regular_font.clone(),
                    font_size: code_view_style.font_size,
                    ..default()
                }),
                ..default()
            });
        });
    });
}
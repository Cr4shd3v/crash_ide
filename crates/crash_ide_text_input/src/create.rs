use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use crate::{TextInputContainer, TextInputLineContainer, TextInput, TextInputContent, TextInputStyle, TextInputContentLine};
use crate::component::TextInputLine;

pub(super) fn create_code_view(
    mut commands: Commands,
    mut query: Query<(Entity, &TextInputStyle, &TextInputContent, &mut Style), Added<TextInput>>,
) {
    for (entity, text_input_style, content, mut style) in query.iter_mut() {
        style.width = Val::Percent(100.0);
        style.flex_direction = FlexDirection::Row;

        let line_content_container = commands.spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                ..default()
            },
            ..default()
        }, Interaction::None, RelativeCursorPosition::default(), TextInputContainer)).with_children(|parent| {
            for line in content.lines.iter() {
                build_line_parent(parent, text_input_style, line);
            }
        }).id();

        commands.entity(entity).despawn_descendants().push_children(&[line_content_container]).insert(TextInputLineContainer {
            line_content_container,
        });
    }
}

pub(crate) fn build_line_command(commands: &mut Commands, code_view_style: &TextInputStyle, line: &TextInputContentLine) -> Entity {
    commands.spawn(create_line_bundle(code_view_style)).with_children(|parent| {
        spawn_lines(parent, code_view_style, line);
    }).id()
}

pub(crate) fn build_line_parent(parent: &mut ChildBuilder, code_view_style: &TextInputStyle, line: &TextInputContentLine) {
    parent.spawn(create_line_bundle(code_view_style)).with_children(|parent| {
        spawn_lines(parent, code_view_style, line);
    });
}

fn create_line_bundle(code_view_style: &TextInputStyle) -> impl Bundle {
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
        TextInputLine,
    )
}

fn spawn_lines(parent: &mut ChildBuilder, code_view_style: &TextInputStyle, line: &TextInputContentLine) {
    for token in line.tokens.iter() {
        parent.spawn(TextBundle {
            text: Text::from_section(&token.content, TextStyle {
                font: code_view_style.font.clone(),
                font_size: code_view_style.font_size,
                ..default()
            }).with_no_wrap(),
            ..default()
        });
    }
}
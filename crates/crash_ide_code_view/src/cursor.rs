use std::ops::Mul;
use ab_glyph::{Font, ScaleFont};
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use crash_ide_assets::DefaultColors;
use crash_ide_util::FindComponentInParents;

use crate::{CodeView, CodeViewContainer, CodeViewContent, CodeViewCursorPosition, CodeViewCursorTimer, CodeViewFocused, CodeViewLineContainer, CodeViewStyle, CursorEntityRef, HighlightedLine, HighlightedLineCount};
use crate::line_container::GetLineContainer;

pub(super) fn init_cursor(
    mut commands: Commands,
    query: Query<(Entity, &Parent), Added<CodeViewContainer>>,
    find_code_view: FindComponentInParents<CodeView>,
    view_query: Query<(&CodeViewCursorPosition, &CodeViewStyle)>,
    font_assets: Res<Assets<bevy::text::Font>>,
) {
    for (container_entity, parent) in query.iter() {
        let code_view_entity = find_code_view.find_entity(parent.get()).unwrap();
        let (cursor, style) = view_query.get(code_view_entity).unwrap();

        let scaled_font = font_assets.get(&style.regular_font).unwrap().font.as_scaled(style.font_size);
        let advance = scaled_font.h_advance(scaled_font.font.glyph_id(' '));

        let cursor_id = commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(((style.font_size + 2.0) * cursor.cursor_pos.y as f32) + 1.0),
                left: Val::Px(advance * cursor.cursor_pos.x as f32),
                width: Val::Px(2.0),
                height: Val::Px(style.font_size),
                margin: UiRect::left(Val::Px(1.5)),
                ..default()
            },
            z_index: ZIndex::Local(1),
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        }).id();

        commands.entity(container_entity).insert_children(0, &[cursor_id]);
        commands.entity(code_view_entity).insert(CursorEntityRef(cursor_id));
    }
}

pub(super) fn update_cursor_pos(
    mut commands: Commands,
    mut query: Query<(
        &CodeViewCursorPosition,
        &CursorEntityRef,
        &CodeViewStyle,
        &mut CodeViewCursorTimer,
        &CodeViewLineContainer,
    ), Changed<CodeViewCursorPosition>>,
    mut style_query: Query<&mut Style>,
    mut background_query: Query<&mut BackgroundColor>,
    get_line_container: GetLineContainer,
    highlighted_line_query: Query<Entity, With<HighlightedLine>>,
    highlighted_line_count_query: Query<Entity, With<HighlightedLineCount>>,
    children_query: Query<&Children>,
    font_assets: Res<Assets<bevy::text::Font>>,
) {
    for (cursor, cursor_entity,
        code_style, mut timer, lines) in query.iter_mut() {
        let mut style = style_query.get_mut(cursor_entity.0).unwrap();

        let scaled_font = font_assets.get(&code_style.regular_font).unwrap().font.as_scaled(code_style.font_size);
        let advance = scaled_font.h_advance(scaled_font.font.glyph_id(' '));

        style.top = Val::Px(((code_style.font_size + 2.0) * cursor.cursor_pos.y as f32) + 1.0);
        style.left = Val::Px(advance * cursor.cursor_pos.x as f32);
        timer.reset = true;

        let line_children = children_query.get(lines.line_content_container).unwrap();
        for line_entity in highlighted_line_query.iter_many(line_children) {
            background_query.get_mut(line_entity).unwrap().0 = Color::NONE;
            commands.entity(line_entity).remove::<HighlightedLine>();
        }

        let line_count_children = children_query.get(lines.line_count_container).unwrap();
        for line_entity in highlighted_line_count_query.iter_many(line_count_children) {
            background_query.get_mut(line_entity).unwrap().0 = Color::NONE;
            commands.entity(line_entity).remove::<HighlightedLineCount>();
        }

        let (line_count, line_content) = get_line_container.get_line(lines, cursor.cursor_pos.y as usize);
        background_query.get_mut(line_count).unwrap().0 = DefaultColors::GRAY.with_alpha(0.1);
        background_query.get_mut(line_content).unwrap().0 = DefaultColors::GRAY.with_alpha(0.1);
        commands.entity(line_content).insert(HighlightedLine);
        commands.entity(line_count).insert(HighlightedLineCount);
    }
}

pub(super) fn cursor_blinking(
    mut query: Query<(&mut CodeViewCursorTimer, &CursorEntityRef, Option<&CodeViewFocused>)>,
    mut background_query: Query<&mut BackgroundColor>,
    time: Res<Time>,
) {
    for (mut timer, cursor_entity, focused) in query.iter_mut() {
        if focused.is_none() {
            continue;
        }

        if timer.reset {
            timer.timer.reset();
            timer.reset = false;
            if let Ok(mut background) = background_query.get_mut(cursor_entity.0) {
                background.0 = Color::WHITE;
            }
            continue;
        }

        if !timer.timer.tick(time.delta()).just_finished() {
            continue;
        }

        if let Ok(mut background) = background_query.get_mut(cursor_entity.0) {
            if background.0 == Color::WHITE {
                background.0 = Color::NONE;
            } else {
                background.0 = Color::WHITE;
            }
        }
    }
}

pub(super) fn cursor_to_click(
    query: Query<(
        &Interaction,
        &RelativeCursorPosition,
        &Node,
        &Parent,
    ), (Changed<Interaction>, With<CodeViewContainer>)>,
    find_code_view: FindComponentInParents<CodeViewContent>,
    mut code_view_query: Query<(
        &CodeViewStyle,
        &mut CodeViewCursorPosition,
        &CodeViewContent,
    )>,
    font_assets: Res<Assets<bevy::text::Font>>,
) {
    for (interaction, relative_cursor_pos, node, parent) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let code_view_entity = find_code_view.find_entity(parent.get()).unwrap();
        let (code_style, mut cursor_pos, content) = code_view_query.get_mut(code_view_entity).unwrap();

        let node_size = node.size();
        let cursor_pos_normalized = relative_cursor_pos.normalized.unwrap();
        let font_size = code_style.font_size;

        let cursor_pos_relative = cursor_pos_normalized.mul(node_size);

        let scaled_font = font_assets.get(&code_style.regular_font).unwrap().font.as_scaled(code_style.font_size);
        let advance = scaled_font.h_advance(scaled_font.font.glyph_id(' '));

        let calculated_line = (cursor_pos_relative.y / (font_size + 2.0)).floor() as u32;
        let mut calculated_column = ((cursor_pos_relative.x / advance).round() as u32).max(0);

        if let Some(length) = content.get_line_length(calculated_line as usize) {
            if calculated_column > length as u32 {
                calculated_column = length as u32;
            }
        }

        cursor_pos.cursor_pos = UVec2::new(calculated_column, calculated_line);
    }
}
use std::ops::Mul;

use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

use crash_ide_util::FindComponentInParents;
use crash_ide_widget::ActiveWindow;

use crate::{CodeView, CodeViewContainer, CodeViewContent, CodeViewCursorPosition, CodeViewCursorTimer, CodeViewFocused, CodeViewLineRegistry, CodeViewStyle, CursorEntityRef};

pub(crate) const FONT_MULTIPLIER: f32 = 0.606;

pub(super) fn init_cursor(
    mut commands: Commands,
    query: Query<(Entity, &Parent), Added<CodeViewContainer>>,
    find_code_view: FindComponentInParents<CodeView>,
    view_query: Query<(&CodeViewCursorPosition, &CodeViewStyle)>,
) {
    for (container_entity, parent) in query.iter() {
        let code_view_entity = find_code_view.find_entity(parent.get()).unwrap();
        let (cursor, style) = view_query.get(code_view_entity).unwrap();

        let cursor_id = commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(((style.font_size + 2.0) * cursor.cursor_pos.y as f32) + 1.0),
                left: Val::Px((style.font_size * FONT_MULTIPLIER) * cursor.cursor_pos.x as f32),
                width: Val::Px(2.0),
                height: Val::Px(style.font_size),
                ..default()
            },
            z_index: ZIndex::Local(1),
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        }).id();

        commands.entity(container_entity).add_child(cursor_id);
        commands.entity(code_view_entity).insert(CursorEntityRef(cursor_id));
    }
}

pub(super) fn update_cursor_pos(
    mut query: Query<(
        &CodeViewCursorPosition,
        &CursorEntityRef,
        &CodeViewStyle,
        &mut CodeViewCursorTimer,
        &mut CodeViewLineRegistry,
    ), Changed<CodeViewCursorPosition>>,
    mut style_query: Query<&mut Style>,
    mut background_query: Query<&mut BackgroundColor>,
) {
    for (cursor, cursor_entity,
        code_style, mut timer, mut lines) in query.iter_mut() {
        let mut style = style_query.get_mut(cursor_entity.0).unwrap();
        style.top = Val::Px(((code_style.font_size + 2.0) * cursor.cursor_pos.y as f32) + 1.0);
        style.left = Val::Px((code_style.font_size * FONT_MULTIPLIER) * cursor.cursor_pos.x as f32);
        timer.reset = true;

        if let Some(active_line) = lines.lines.get(&lines.active) {
            background_query.get_mut(active_line.line_content).unwrap().0 = Color::NONE;
            background_query.get_mut(active_line.line_count).unwrap().0 = Color::NONE;
        }

        let line = lines.lines.get(&(cursor.cursor_pos.y as usize)).unwrap();
        background_query.get_mut(line.line_content).unwrap().0 = Color::GRAY.with_a(0.1);
        background_query.get_mut(line.line_count).unwrap().0 = Color::GRAY.with_a(0.1);

        lines.active = cursor.cursor_pos.y as usize;
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
    window: Query<&Window, With<ActiveWindow>>,
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

        let scale = window.single().resolution.scale_factor();
        let cursor_pos_relative = cursor_pos_normalized.mul(node_size) * scale;

        let calculated_line = (cursor_pos_relative.y / (font_size + 2.0)).floor() as i32;
        let mut calculated_column = ((cursor_pos_relative.x / (font_size * FONT_MULTIPLIER)).round() as i32).max(0);

        if let Some(line_content) = content.lines.get(calculated_line as usize) {
            let length = line_content.iter().map(|v| v.content.len()).sum::<usize>() as i32;
            if calculated_column > length {
                calculated_column = length;
            }
        }

        cursor_pos.cursor_pos = IVec2::new(calculated_column, calculated_line);
    }
}
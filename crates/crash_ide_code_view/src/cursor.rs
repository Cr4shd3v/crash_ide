use bevy::prelude::*;
use crate::{CodeViewCursorPosition, CodeViewCursorTimer, CodeViewFocused, CodeViewStyle, CursorEntityRef};

pub(crate) const FONT_MULTIPLIER: f32 = 0.606;

pub(super) fn init_cursor(
    mut commands: Commands,
    query: Query<(Entity, &CodeViewCursorPosition, &CodeViewStyle), Added<CodeViewCursorPosition>>,
) {
    for (entity, cursor, style) in query.iter() {
        let cursor_id = commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(((style.font_size + 2.0) * cursor.cursor_pos.y as f32) + 1.0),
                left: Val::Px((style.font_size * 1.5) + 28.0 + ((style.font_size * FONT_MULTIPLIER) * cursor.cursor_pos.x as f32)),
                width: Val::Px(2.0),
                height: Val::Px(style.font_size),
                ..default()
            },
            z_index: ZIndex::Local(1),
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        }).id();

        commands.entity(entity).add_child(cursor_id).insert(CursorEntityRef(cursor_id));
    }
}

pub(super) fn update_cursor_pos(
    mut query: Query<(&CodeViewCursorPosition, &CursorEntityRef, &CodeViewStyle, &mut CodeViewCursorTimer), Changed<CodeViewCursorPosition>>,
    mut style_query: Query<&mut Style>,
) {
    for (cursor, cursor_entity,
        code_style, mut timer) in query.iter_mut() {
        let mut style = style_query.get_mut(cursor_entity.0).unwrap();
        style.top = Val::Px(((code_style.font_size + 2.0) * cursor.cursor_pos.y as f32) + 1.0);
        style.left = Val::Px((code_style.font_size * 1.5) + 28.0 + ((code_style.font_size * FONT_MULTIPLIER) * cursor.cursor_pos.x as f32));
        println!("Top: {:?}, Left: {:?}, Cursor: {:?}", style.top, style.left, cursor.cursor_pos);
        timer.reset = true;
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
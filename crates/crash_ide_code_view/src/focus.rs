use std::ops::Mul;

use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

use crash_ide_widget::{ActiveWindow, SetCursorEvent};

use crate::{CodeViewCursorPosition, CodeViewFocused, CodeViewStyle};
use crate::cursor::FONT_MULTIPLIER;

pub(super) fn focus_code_view(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        Option<&CodeViewFocused>,
        &Interaction,
        &RelativeCursorPosition,
        &Node,
        &CodeViewStyle,
        &mut CodeViewCursorPosition
    ), Changed<Interaction>>,
    current_focus: Query<Entity, With<CodeViewFocused>>,
    window: Query<&Window, With<ActiveWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut cursor_writer: EventWriter<SetCursorEvent>,
) {
    let current_focus_entity = current_focus.get_single();
    let mut click_on_code = false;

    for (entity, focused, interaction, relative_cursor_pos,
        node, code_style, mut cursor_pos) in query.iter_mut() {
        if *interaction == Interaction::None {
            cursor_writer.send(SetCursorEvent::reset());
        } else {
            cursor_writer.send(SetCursorEvent::cursor(CursorIcon::Text));
        }

        if *interaction != Interaction::Pressed {
            continue;
        }

        click_on_code = true;

        let node_size = node.size();
        let cursor_pos_normalized = relative_cursor_pos.normalized.unwrap();
        let font_size = code_style.font_size;

        let scale = window.single().resolution.scale_factor();
        let cursor_pos_relative = cursor_pos_normalized.mul(node_size) * scale;

        let calculated_line = (cursor_pos_relative.y / (font_size + 2.0)).floor() as i32;
        let calculated_column = (((cursor_pos_relative.x - ((code_style.font_size * 1.5) + 28.0)) / (font_size * FONT_MULTIPLIER)).round() as i32).max(0);

        cursor_pos.cursor_pos = IVec2::new(calculated_column, calculated_line);

        if focused.is_some() {
            continue;
        }

        if let Ok(current_focus_entity) = current_focus_entity {
            commands.entity(current_focus_entity).remove::<CodeViewFocused>();
        }

        commands.entity(entity).insert(CodeViewFocused);
    }

    if !click_on_code && buttons.any_just_pressed([MouseButton::Left, MouseButton::Right, MouseButton::Middle]) {
        if let Ok(current_focus_entity) = current_focus_entity {
            commands.entity(current_focus_entity).remove::<CodeViewFocused>();
        }
    }
}
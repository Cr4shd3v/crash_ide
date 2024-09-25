use bevy::prelude::*;

use crash_ide_widget::SetCursorEvent;

use crate::{TextInput, TextInputFocused};

pub(super) fn focus_code_view(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        Option<&TextInputFocused>,
        &Interaction,
    ), (Changed<Interaction>, With<TextInput>)>,
    current_focus: Query<Entity, With<TextInputFocused>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut cursor_writer: EventWriter<SetCursorEvent>,
) {
    let current_focus_entity = current_focus.get_single();
    let mut click_on_code = false;

    for (entity, focused, interaction) in query.iter_mut() {
        if *interaction == Interaction::None {
            cursor_writer.send(SetCursorEvent::reset());
        } else {
            cursor_writer.send(SetCursorEvent::cursor(CursorIcon::Text));
        }

        if *interaction != Interaction::Pressed {
            continue;
        }

        click_on_code = true;

        if focused.is_some() {
            continue;
        }

        if let Ok(current_focus_entity) = current_focus_entity {
            commands.entity(current_focus_entity).remove::<TextInputFocused>();
        }

        commands.entity(entity).insert(TextInputFocused);
    }

    if !click_on_code && buttons.any_just_pressed([MouseButton::Left, MouseButton::Right, MouseButton::Middle]) {
        if let Ok(current_focus_entity) = current_focus_entity {
            commands.entity(current_focus_entity).remove::<TextInputFocused>();
        }
    }
}
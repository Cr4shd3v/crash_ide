use bevy::prelude::*;

use crash_ide_widget::SetCursorEvent;

pub(crate) fn button_cursor(
    btn_query: Query<&Interaction, (With<Button>, Changed<Interaction>)>,
    mut cursor_writer: EventWriter<SetCursorEvent>,
) {
    let mut is_hovered_this_run = false;

    for interaction in btn_query.iter() {
        match interaction {
            Interaction::Hovered => {
                cursor_writer.send(SetCursorEvent::cursor(CursorIcon::Pointer));
                is_hovered_this_run = true;
            }
            _ => {
                if !is_hovered_this_run {
                    cursor_writer.send(SetCursorEvent::reset());
                }
            }
        }
    }
}
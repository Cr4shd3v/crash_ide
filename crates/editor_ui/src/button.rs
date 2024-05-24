use bevy::prelude::*;

pub(crate) fn button_cursor(
    btn_query: Query<&Interaction, (With<Button>, Changed<Interaction>)>,
    mut windows_query: Query<&mut Window>,
) {
    let mut is_hovered_this_run = false;

    for interaction in btn_query.iter() {
        match interaction {
            Interaction::Hovered => {
                for mut window in windows_query.iter_mut() {
                    window.cursor.icon = CursorIcon::Pointer;
                }
                is_hovered_this_run = true;
            }
            _ => {
                if !is_hovered_this_run {
                    for mut window in windows_query.iter_mut() {
                        window.cursor.icon = CursorIcon::Default;
                    }
                }
            }
        }
    }
}
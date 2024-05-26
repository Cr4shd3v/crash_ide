use bevy::prelude::*;
use crate::window::ActiveWindow;

pub(crate) fn button_cursor(
    btn_query: Query<&Interaction, (With<Button>, Changed<Interaction>)>,
    mut windows_query: Query<&mut Window, With<ActiveWindow>>,
) {
    let mut is_hovered_this_run = false;

    for interaction in btn_query.iter() {
        match interaction {
            Interaction::Hovered => {
                if let Ok(mut window) = windows_query.get_single_mut() {
                    window.cursor.icon = CursorIcon::Pointer;
                }
                is_hovered_this_run = true;
            }
            _ => {
                if !is_hovered_this_run {
                    if let Ok(mut window) = windows_query.get_single_mut() {
                        window.cursor.icon = CursorIcon::Default;
                    }
                }
            }
        }
    }
}
use bevy::prelude::*;
use crate::ActiveWindow;

pub(super) struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SetCursorEvent>()
            .add_systems(PostUpdate, set_cursor)
        ;
    }
}

/// Event to set a cursor. Handles the correct setting of a cursor.
#[derive(Event)]
pub struct SetCursorEvent {
    cursor: Option<CursorIcon>,
}

impl SetCursorEvent {
    /// Constructs an event to reset the cursor back to [CursorIcon::Default]
    pub fn reset() -> Self {
        Self {
            cursor: None,
        }
    }

    /// Creates an event to set the cursor icon to `cursor`
    pub fn cursor(cursor: CursorIcon) -> Self {
        Self {
            cursor: Some(cursor),
        }
    }
}

fn set_cursor(
    mut event_reader: EventReader<SetCursorEvent>,
    mut active_window: Query<&mut Window, With<ActiveWindow>>,
) {
    let Ok(mut window) = active_window.get_single_mut() else {
        return;
    };

    let mut reset = false;
    let mut new_cursor = None;

    for event in event_reader.read() {
        if event.cursor.is_none() {
            reset = true;
        } else {
            new_cursor = event.cursor;
        }
    }

    if let Some(new_cursor) = new_cursor {
        window.cursor.icon = new_cursor;
    } else if reset {
        window.cursor.icon = CursorIcon::Default;
    }
}



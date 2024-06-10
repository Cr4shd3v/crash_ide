use bevy::prelude::*;

pub(super) struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_active_window);
    }
}

/// Marks the active window.
///
/// Active means, that this is the window where the cursor is right now.
#[derive(Component)]
pub struct ActiveWindow;

fn update_active_window(
    mut commands: Commands,
    mut window_focused_event_reader: EventReader<CursorEntered>,
    current_query: Query<Entity, With<ActiveWindow>>,
) {
    for window_focused in window_focused_event_reader.read() {
        if let Ok(current_entity) = current_query.get_single() {
            commands.entity(current_entity).remove::<ActiveWindow>();
        }

        commands.entity(window_focused.window).insert(ActiveWindow);
    }
}
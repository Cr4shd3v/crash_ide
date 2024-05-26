use bevy::prelude::*;

#[derive(Component)]
pub struct ActiveWindow;

pub(super) fn update_active_window(
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
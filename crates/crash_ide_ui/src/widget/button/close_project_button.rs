use bevy::prelude::*;
use crate::window::{ProjectWindow, StartupWindow};

#[derive(Component)]
pub struct CloseProjectButton {
    pub window_entity: Entity,
}

pub(super) fn close_project_button(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &CloseProjectButton), Changed<Interaction>>,
    startup_window_query: Query<Entity, With<StartupWindow>>,
    mut window_query: Query<&mut Window>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Ok(startup_window) = startup_window_query.get_single() {
            window_query.get_mut(startup_window).unwrap().focused = true;
            commands.entity(button.window_entity).despawn_recursive();
        } else {
            commands.entity(button.window_entity).remove::<ProjectWindow>().insert(StartupWindow);
            let mut window = window_query.get_mut(button.window_entity).unwrap();
            window.resolution = StartupWindow::get_resolution();
            window.position = WindowPosition::Centered(MonitorSelection::Current);
        }
    }
}
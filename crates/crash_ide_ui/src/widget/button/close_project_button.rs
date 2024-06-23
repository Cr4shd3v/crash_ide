use bevy::prelude::*;
use crash_ide_plugin_api::PluginboundMessage;
use crash_ide_plugin_manager::SendPluginMessage;
use crash_ide_project::CloseProjectEvent;
use crate::window::{ProjectWindow, StartupWindow};

/// Component marking the close project button.
#[derive(Component)]
pub struct CloseProjectButton {
    /// Window entity to be closed
    pub window_entity: Entity,
}

pub(super) fn close_project_button(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &CloseProjectButton), Changed<Interaction>>,
    startup_window_query: Query<Entity, With<StartupWindow>>,
    mut window_query: Query<&mut Window>,
    mut event_writer: EventWriter<CloseProjectEvent>,
    mut plugin_ew: EventWriter<SendPluginMessage>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        event_writer.send(CloseProjectEvent {
            window_entity: button.window_entity,
        });

        plugin_ew.send(SendPluginMessage::new(PluginboundMessage::CloseProject, None));

        if let Ok(startup_window) = startup_window_query.get_single() {
            window_query.get_mut(startup_window).unwrap().focused = true;
            commands.entity(button.window_entity).despawn_recursive();
        } else {
            commands.entity(button.window_entity).remove::<ProjectWindow>().insert(StartupWindow);
        }
    }
}
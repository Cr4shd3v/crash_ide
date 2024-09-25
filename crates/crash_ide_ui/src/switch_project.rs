use bevy::prelude::*;
use bevy::window::WindowFocused;
use crate::window::{ProjectWindow, StartupWindow};

pub(super) struct SwitchProjectPlugin;

impl Plugin for SwitchProjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SwitchProjectWindowEvent>()
            .add_systems(Update, check_switch_window)
        ;
    }
}

/// Event fired when switching between project windows
#[derive(Event)]
pub struct SwitchProjectWindowEvent {
    /// Project that has been switched to. This points to an entity with [LoadedEditorProject](crash_ide_config::LoadedEditorProject).
    ///
    /// None when in startup screen.
    pub new_project: Option<Entity>,
}

fn check_switch_window(
    mut event_reader: EventReader<WindowFocused>,
    query: Query<(Option<&ProjectWindow>, Option<&StartupWindow>)>,
    mut event_writer: EventWriter<SwitchProjectWindowEvent>,
) {
    for event in event_reader.read() {
        if !event.focused {
            continue;
        }

        let Ok((project, startup)) = query.get(event.window) else {
            continue;
        };

        if !project.is_some() && !startup.is_some() {
            continue;
        }

        if startup.is_some() {
            event_writer.send(SwitchProjectWindowEvent {
                new_project: None,
            });
            continue;
        }

        let project_window = project.unwrap();
        event_writer.send(SwitchProjectWindowEvent {
            new_project: Some(project_window.project_crash_ide_config),
        });
    }
}


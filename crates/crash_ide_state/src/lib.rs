//! Crate implementing global state of the editor

#![warn(missing_docs)]

use bevy::prelude::*;

/// Initializes the global editor state
pub struct CrashIDEStatePlugin;

impl Plugin for CrashIDEStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_state(EditorState::Loading)
            .init_resource::<EditorLoadStatus>()
            .add_systems(Update, check_loading_finished.run_if(in_state(EditorState::Loading)))
        ;
    }
}

/// Global editor state
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum EditorState {
    /// Editor is in loading state, initializing resources
    Loading,
    /// Editor loaded
    Loaded,
}

/// Loading status of the editor
///
/// If all fields are true, the editor will enter [EditorState::Loaded] in system [check_loading_finished]
#[derive(Resource, Default)]
pub struct EditorLoadStatus {
    /// All configs are loaded
    pub config_loaded: bool,
}

fn check_loading_finished(loading_status: Res<EditorLoadStatus>, mut next_state: ResMut<NextState<EditorState>>) {
    if loading_status.is_changed() {
        if loading_status.config_loaded {
            next_state.set(EditorState::Loaded);
            info!("Loading finished, switching to project select...");
        }
    }
}
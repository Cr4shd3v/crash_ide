use bevy::prelude::*;

/// Initializes the global editor state
pub struct EditorStatePlugin;

impl Plugin for EditorStatePlugin {
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
    Loading,
    StartupScreen,
    Project,
}

/// Loading status of the editor
#[derive(Resource, Default)]
pub struct EditorLoadStatus {
    pub config_loaded: bool,
}

fn check_loading_finished(loading_status: Res<EditorLoadStatus>, mut next_state: ResMut<NextState<EditorState>>) {
    if loading_status.is_changed() {
        if loading_status.config_loaded {
            next_state.set(EditorState::StartupScreen);
            println!("Loading finished, switching to project select...");
        }
    }
}
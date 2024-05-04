use bevy::prelude::*;

/// Initializes the global editor state
pub struct EditorStatePlugin;

impl Plugin for EditorStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_state(EditorState::Loading)
            .init_resource::<LoadingStatus>()
            .add_systems(Update, check_loading_finished.run_if(in_state(EditorState::Loading)))
        ;
    }
}

/// Global editor state
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]

pub enum EditorState {
    Loading,
    ProjectSelect,
    Project,
}

/// Loading status of the editor
#[derive(Resource, Default)]
pub struct LoadingStatus {
    pub config_loaded: bool,
}

fn check_loading_finished(loading_status: Res<LoadingStatus>, mut next_state: ResMut<NextState<EditorState>>) {
    if loading_status.config_loaded {
        next_state.set(EditorState::ProjectSelect);
    }
}
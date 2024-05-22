use bevy::prelude::*;
use crate::startup::StartupScreenState;

#[derive(Component)]
pub(crate) struct CreateProjectButton;

pub(super) fn create_project_button(
    interaction_query: Query<&Interaction, (With<CreateProjectButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<StartupScreenState>>,
) {
    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                next_state.set(StartupScreenState::ProjectCreate);
            }
            _ => {}
        }
    }
}
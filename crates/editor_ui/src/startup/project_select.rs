use bevy::prelude::*;
use crate::startup::{StartupContent, StartupScreenState};

pub(crate) struct ProjectSelectPlugin;

impl Plugin for ProjectSelectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(StartupScreenState::ProjectSelect), build_project_select)
        ;
    }
}

fn build_project_select(mut commands: Commands, content_parent: Query<Entity, With<StartupContent>>) {
    let entity = content_parent.single();
    commands.entity(entity).despawn_descendants().with_children(|_parent| {

    });
}
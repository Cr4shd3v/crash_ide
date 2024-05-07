use bevy::prelude::*;
use crate::startup::{StartupContentRoot, StartupScreenState};

pub(crate) struct ProjectSelectPlugin;

impl Plugin for ProjectSelectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(StartupScreenState::ProjectSelect), build_project_select)
        ;
    }
}

fn build_project_select(mut commands: Commands, content_parent: Query<Entity, With<StartupContentRoot>>) {
    let entity = content_parent.single();
    commands.entity(entity).despawn_descendants().with_children(|parent| {
        parent.spawn(TextBundle::from_section("Projects", TextStyle::default()));
    });
}
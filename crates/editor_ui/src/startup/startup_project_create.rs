use bevy::prelude::*;
use crate::startup::{StartupContentRoot, StartupScreenState};
use crate::widget::input::TextInputBundle;

pub(super) struct StartupProjectCreatePlugin;

impl Plugin for StartupProjectCreatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(StartupScreenState::ProjectCreate), build_create_project)
        ;
    }
}

fn build_create_project(
    mut commands: Commands,
    content_parent: Query<Entity, With<StartupContentRoot>>,
) {
    let entity = match content_parent.get_single() {
        Ok(entity) => entity,
        Err(_) => {
            return;
        }
    };

    commands.entity(entity).despawn_descendants().with_children(|parent| {
        parent.spawn((NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            border_color: BorderColor(Color::GRAY),
            ..default()
        }, TextInputBundle::default()));
    });
}
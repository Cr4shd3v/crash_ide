use bevy::prelude::*;
use crate::settings::SettingsScreen;
use crate::startup::{StartupContentRoot, StartupScreenState};

pub(crate) struct StartupSettingsPlugin;

impl Plugin for StartupSettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(StartupScreenState::Settings), build_settings)
        ;
    }
}

fn build_settings(mut commands: Commands, content_parent: Query<Entity, With<StartupContentRoot>>) {
    let entity = content_parent.single();
    commands.entity(entity).despawn_descendants().with_children(|parent| {
        parent.spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }, SettingsScreen::default()));
    });
}
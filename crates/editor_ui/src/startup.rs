mod startup_left_menu;
mod project_select;

use bevy::prelude::*;
use editor_state::EditorState;
use crate::startup::project_select::ProjectSelectPlugin;
use crate::startup::startup_left_menu::startup_left_menu;
use crate::UiRoot;

pub(crate) struct StartupScreenPlugin;

impl Plugin for StartupScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_state(StartupScreenState::None)
            .add_systems(OnEnter(EditorState::StartupScreen), spawn_startup_screen)
            .add_plugins(ProjectSelectPlugin)
        ;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum StartupScreenState {
    None,
    ProjectSelect,
}

#[derive(Component)]
pub(crate) struct StartupContent;

fn spawn_startup_screen(mut commands: Commands, mut ui_root: ResMut<UiRoot>, mut startup_state: ResMut<NextState<StartupScreenState>>) {
    commands.entity(ui_root.root).despawn_recursive();

    ui_root.root = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Percent(2.0)),
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        startup_left_menu(parent);
        parent.spawn((StartupContent, NodeBundle {
            style: Style {
                width: Val::Percent(70.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::GREEN),
            ..default()
        }));
    }).id();

    startup_state.set(StartupScreenState::ProjectSelect);
}
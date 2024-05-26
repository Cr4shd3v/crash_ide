mod startup_left_menu;
mod startup_project_select;
mod startup_settings;
mod startup_project_create;

use bevy::prelude::*;
use editor_state::EditorState;
use crate::root::UiRoot;
use crate::startup::startup_project_select::StartupProjectSelectPlugin;
use crate::startup::startup_settings::StartupSettingsPlugin;
use crate::startup::startup_left_menu::{handle_left_menu_state_change, startup_left_menu, startup_left_menu_click};
use crate::startup::startup_project_create::StartupProjectCreatePlugin;
use crate::window::{StartupWindow, WindowCamera};

pub(crate) struct StartupScreenPlugin;

impl Plugin for StartupScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_state(StartupScreenState::None)
            .add_systems(OnEnter(EditorState::StartupScreen), spawn_startup_screen)
            .add_systems(Update, (startup_left_menu_click, handle_left_menu_state_change).run_if(in_state(EditorState::StartupScreen)))
            .add_plugins((StartupProjectSelectPlugin, StartupSettingsPlugin, StartupProjectCreatePlugin))
        ;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum StartupScreenState {
    None,
    ProjectSelect,
    Settings,
    ProjectCreate,
}

impl StartupScreenState {
    pub fn title(&self) -> &'static str {
        match self {
            StartupScreenState::None => "None",
            StartupScreenState::ProjectSelect => "Projects",
            StartupScreenState::Settings => "Settings",
            StartupScreenState::ProjectCreate => "Create Project",
        }
    }
}

#[derive(Component)]
pub(crate) struct StartupContentRoot;

fn spawn_startup_screen(
    mut commands: Commands,
    mut ui_root: ResMut<UiRoot>,
    mut startup_state: ResMut<NextState<StartupScreenState>>,
    window_query: Query<&WindowCamera, With<StartupWindow>>
) {
    commands.entity(ui_root.root).despawn_recursive();

    ui_root.root = commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Percent(2.0)),
            ..default()
        },
        ..default()
    }, TargetCamera(window_query.single().camera))).with_children(|parent| {
        startup_left_menu(parent);
        parent.spawn((StartupContentRoot, NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Percent(1.0)),
                ..default()
            },
            ..default()
        }));
    }).id();

    startup_state.set(StartupScreenState::ProjectSelect);
}
mod startup_left_menu;
mod startup_project_select;
mod startup_settings;

use bevy::prelude::*;
use crash_ide_assets::DefaultIcons;
use crate::startup::startup_project_select::StartupProjectSelectPlugin;
use crate::startup::startup_settings::StartupSettingsPlugin;
use crate::startup::startup_left_menu::{handle_left_menu_state_change, startup_left_menu, startup_left_menu_click};
use crate::window::{AllWindows, StartupWindow};

pub(crate) struct StartupScreenPlugin;

impl Plugin for StartupScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_state(StartupScreenState::None)
            .add_systems(Update, (spawn_startup_screen, startup_left_menu_click, handle_left_menu_state_change))
            .add_plugins((StartupProjectSelectPlugin, StartupSettingsPlugin))
        ;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum StartupScreenState {
    None,
    ProjectSelect,
    Settings,
}

impl StartupScreenState {
    pub fn title(&self) -> &'static str {
        match self {
            StartupScreenState::None => "None",
            StartupScreenState::ProjectSelect => "Projects",
            StartupScreenState::Settings => "Settings",
        }
    }
}

#[derive(Component)]
pub(crate) struct StartupContentRoot;

fn spawn_startup_screen(
    mut commands: Commands,
    mut startup_state: ResMut<NextState<StartupScreenState>>,
    window_query: Query<Entity, Added<StartupWindow>>,
    all_windows: Res<AllWindows>,
    icons: Res<DefaultIcons>,
) {
    let Ok(window_entity) = window_query.get_single() else {
        return;
    };

    commands.entity(all_windows.get(&window_entity).ui_root).despawn_descendants().with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Percent(2.0)),
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            startup_left_menu(parent, &icons);
            parent.spawn((StartupContentRoot, NodeBundle {
                style: Style {
                    width: Val::Percent(80.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            }));
        });
    });

    startup_state.set(StartupScreenState::ProjectSelect);
}
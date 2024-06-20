mod general_settings_menu;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crash_ide_assets::DefaultFonts;
use crash_ide_util::FindComponentInParents;
use crash_ide_widget::Hoverable;
use crate::settings::general_settings_menu::{GeneralSettingsMenu, GeneralSettingsMenuPlugin};
use crate::window::AllWindows;

pub(super) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (spawn_settings_screen, settings_window, change_menu))
            .add_plugins(GeneralSettingsMenuPlugin)
        ;
    }
}

#[derive(Component)]
pub struct SettingsWindow;

fn settings_window(
    mut commands: Commands,
    query: Query<Entity, Added<SettingsWindow>>,
    all_windows: Res<AllWindows>,
) {
    for entity in query.iter() {
        commands.entity(all_windows.get(&entity).ui_root).despawn_descendants().with_children(|parent| {
            parent.spawn((NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            }, SettingsScreen::default()));
        });
    }
}

#[derive(Component)]
pub struct PluginSettingsMenuMarker(pub String);

#[derive(Component, Default, PartialEq, Clone)]
pub enum SettingsScreen {
    #[default]
    General,
    Plugin(String),
}

impl SettingsScreen {
    pub fn title(&self) -> String {
        match self {
            SettingsScreen::General => "General".to_string(),
            SettingsScreen::Plugin(v) => v.clone(),
        }
    }

    pub fn marker<'a>(&self, builder: &'a mut ChildBuilder) -> EntityCommands<'a> {
        match self {
            SettingsScreen::General => builder.spawn(GeneralSettingsMenu),
            SettingsScreen::Plugin(v) => builder.spawn(PluginSettingsMenuMarker(v.clone())),
        }
    }
}

fn spawn_settings_screen(
    mut commands: Commands,
    query: Query<(Entity, &SettingsScreen), Changed<SettingsScreen>>,
) {
    for (entity, screen) in query.iter() {
        commands.entity(entity).despawn_descendants().with_children(|parent| {
            spawn_left_menu(parent, screen);

            screen.marker(parent).insert(NodeBundle {
                style: Style {
                    width: Val::Percent(80.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            });
        });
    }
}

fn spawn_left_menu(builder: &mut ChildBuilder, active_screen: &SettingsScreen) {
    builder.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(20.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            border: UiRect::right(Val::Px(2.0)),
            ..default()
        },
        border_color: BorderColor(Color::GRAY.with_a(0.16)),
        ..default()
    }).with_children(|parent| {
        left_menu_entry(parent, SettingsScreen::General, active_screen);

        parent.spawn(NodeBundle {
            style: Style {
                margin: UiRect::top(Val::Px(15.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                border: UiRect::bottom(Val::Px(2.0)),
                ..default()
            },
            border_color: BorderColor(Color::GRAY.with_a(0.1)),
            ..default()
        }).with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("Plugins", TextStyle {
                    font: DefaultFonts::ROBOTO_REGULAR,
                    font_size: 16.0,
                    ..default()
                }),
                style: Style {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
                ..default()
            });
        });
    });
}

#[derive(Component)]
struct SettingsLeftMenuEntry(SettingsScreen);

fn left_menu_entry(builder: &mut ChildBuilder, menu: SettingsScreen, active_screen: &SettingsScreen) {
    let title = menu.title();

    builder.spawn((ButtonBundle {
        style: Style {
            padding: UiRect::axes(Val::Percent(5.0), Val::Percent(4.0)),
            justify_content: JustifyContent::FlexEnd,
            ..default()
        },
        background_color: BackgroundColor(if *active_screen == menu {
            Color::GRAY.with_a(0.2)
        } else {
            Color::NONE
        }),
        ..default()
    }, SettingsLeftMenuEntry(menu), Hoverable::new(Color::GRAY.with_a(0.2)),
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(title, TextStyle {
            font_size: 16.0,
            font: DefaultFonts::ROBOTO_REGULAR,
            ..default()
        }));
    });
}

fn change_menu(
    query: Query<(Entity, &SettingsLeftMenuEntry, &Interaction), Changed<Interaction>>,
    mut settings_screen: ParamSet<(
        FindComponentInParents<SettingsScreen>,
        Query<&mut SettingsScreen>,
    )>,
) {
    for (entity, menu_entry, interaction) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let screen = settings_screen.p0().find_entity(entity).unwrap();
        if let Ok(mut screen) = settings_screen.p1().get_mut(screen) {
            if *screen != menu_entry.0 {
                *screen = menu_entry.0.clone();
            }
        }
    }
}



use bevy::prelude::*;
use editor_config::EditorConfigProjects;
use crate::fonts::DefaultFonts;
use crate::startup::{StartupContentRoot, StartupScreenState};
use crate::widget::button::{CreateProjectButton, OpenProjectButton};
use crate::window::StartupWindow;

pub(crate) struct StartupProjectSelectPlugin;

impl Plugin for StartupProjectSelectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(StartupScreenState::ProjectSelect), build_project_select)
            .add_systems(Update, build_project_select.run_if(resource_changed::<EditorConfigProjects>))
        ;
    }
}

fn build_project_select(
    mut commands: Commands,
    content_parent: Query<Entity, With<StartupContentRoot>>,
    projects: Res<EditorConfigProjects>,
    window_query: Query<&Window, With<StartupWindow>>,
) {
    // Check if the window still exists
    if window_query.get_single().is_err() {
        return;
    }

    let entity = match content_parent.get_single() {
        Ok(entity) => entity,
        Err(_) => {
            return;
        }
    };

    commands.entity(entity).despawn_descendants().with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::RowReverse,
                border: UiRect::bottom(Val::Px(1.0)),
                padding: UiRect::bottom(Val::Vh(1.0)),
                ..default()
            },
            border_color: BorderColor(Color::GRAY.with_a(0.1)),
            ..default()
        }).with_children(|parent| {
            parent.spawn((ButtonBundle {
                style: Style {
                    padding: UiRect::axes(Val::Vw(2.0), Val::Vh(0.6)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::hex("#578AF2").unwrap()),
                ..default()
            }, CreateProjectButton)).with_children(|parent| {
                parent.spawn(TextBundle::from_section("New Project", TextStyle {
                    font_size: 14.0,
                    font: DefaultFonts::ROBOTO_REGULAR,
                    ..default()
                }));
            });

            parent.spawn((ButtonBundle {
                style: Style {
                    padding: UiRect::axes(Val::Vw(2.0), Val::Vh(0.6)),
                    justify_content: JustifyContent::Center,
                    margin: UiRect::right(Val::Vw(2.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::hex("#578AF2").unwrap()),
                ..default()
            }, OpenProjectButton)).with_children(|parent| {
                parent.spawn(TextBundle::from_section("Open", TextStyle {
                    font_size: 14.0,
                    font: DefaultFonts::ROBOTO_REGULAR,
                    ..default()
                }));
            });
        });

        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                padding: UiRect::horizontal(Val::Vw(2.0)),
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            for project in &projects.projects {
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::vertical(Val::Vh(1.0)),
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    parent.spawn(TextBundle::from_section(&project.name, TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        font: DefaultFonts::ROBOTO_REGULAR,
                    }));
                    parent.spawn(TextBundle::from_section(&project.path, TextStyle {
                        font_size: 13.0,
                        color: Color::GRAY,
                        font: DefaultFonts::ROBOTO_REGULAR,
                    }));
                });
            }
        });
    });
}
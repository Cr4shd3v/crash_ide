use std::fs;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use crash_ide_config::EditorConfigProjects;
use crash_ide_widget::Hoverable;
use crash_ide_assets::{DefaultColors, DefaultFonts, DefaultIcons};
use crash_ide_project::OpenProjectEvent;
use crate::startup::{StartupContentRoot, StartupScreenState};
use crate::widget::button::{CreateProjectButton, OpenProjectButton};
use crate::window::StartupWindow;

pub(crate) struct StartupProjectSelectPlugin;

impl Plugin for StartupProjectSelectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(StartupScreenState::ProjectSelect), build_project_select)
            .add_systems(Update, build_project_select.run_if(resource_changed::<EditorConfigProjects>))
            .add_systems(Update, (handle_row_click, delete_project))
        ;
    }
}

#[derive(Component)]
struct ProjectRow {
    path: String,
    folder_exists: bool,
}

#[derive(Component)]
struct DeleteProjectButton;

fn build_project_select(
    mut commands: Commands,
    content_parent: Query<Entity, With<StartupContentRoot>>,
    projects: Res<EditorConfigProjects>,
    window_query: Query<Entity, With<StartupWindow>>,
    icons: Res<DefaultIcons>,
) {
    // Check if the window still exists
    let Ok(startup_window) = window_query.get_single() else {
        return;
    };

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
                background_color: BackgroundColor(DefaultColors::PRIMARY_BUTTON),
                ..default()
            }, CreateProjectButton {
                base_window: Some(startup_window),
            })).with_children(|parent| {
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
                background_color: BackgroundColor(DefaultColors::PRIMARY_BUTTON),
                ..default()
            }, OpenProjectButton {
                base_window: Some(startup_window),
            })).with_children(|parent| {
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
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            for (path, project) in &projects.projects {
                let folder_exists = fs::metadata(path).is_ok();

                parent.spawn((NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::axes(Val::Vw(2.0), Val::Vh(1.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..default()
                }, ProjectRow {
                    path: path.clone(),
                    folder_exists,
                }, Hoverable::new(if folder_exists { Color::BLACK.with_a(0.2) } else { Color::NONE }),
                              Interaction::None)).with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    }).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(&project.name, TextStyle {
                            font_size: 20.0,
                            color: if folder_exists { Color::WHITE } else { Color::GRAY.with_a(0.8) },
                            font: DefaultFonts::ROBOTO_REGULAR,
                        }));

                        parent.spawn(TextBundle::from_section(path, TextStyle {
                            font_size: 13.0,
                            color: Color::GRAY,
                            font: DefaultFonts::ROBOTO_REGULAR,
                        }));
                    });

                    parent.spawn((ImageBundle {
                        image: UiImage::new(icons.cross.clone()),
                        z_index: ZIndex::Local(0),
                        focus_policy: FocusPolicy::Block,
                        style: Style {
                            width: Val::Px(18.0),
                            ..default()
                        },
                        ..default()
                    }, Interaction::None, Button, DeleteProjectButton));
                });
            }
        });
    });
}

fn handle_row_click(
    query: Query<(&Interaction, &ProjectRow), Changed<Interaction>>,
    mut event_writer: EventWriter<OpenProjectEvent>,
    projects: Res<EditorConfigProjects>,
    window: Query<Entity, With<StartupWindow>>,
) {
    for (interaction, row) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if !row.folder_exists {
            continue;
        }

        let Ok(window_entity) = window.get_single() else { continue; };

        let crash_ide_project = projects.projects.get(&row.path).unwrap();

        event_writer.send(OpenProjectEvent::new(crash_ide_project.clone(), Some(window_entity)));
    }
}

fn delete_project(
    interaction_query: Query<(&Interaction, &Parent), (With<DeleteProjectButton>, Changed<Interaction>)>,
    project_row_query: Query<&ProjectRow>,
    mut projects: ResMut<EditorConfigProjects>,
) {
    for (interaction, parent) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let row = project_row_query.get(parent.get()).unwrap();
        projects.projects.remove(&row.path);
    }
}
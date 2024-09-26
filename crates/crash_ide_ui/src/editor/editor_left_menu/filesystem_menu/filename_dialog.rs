use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;

use crash_ide_assets::DefaultFonts;
use crash_ide_text_input::{TextInputBundle, TextInputContent, TextInputFocused, TextInputSettings, TextInputStyle};
use crate::widget::context_menu::ContextMenu;

/// Marker component for a context menu for changing/creating filenames.
#[derive(Component)]
pub struct FilenameDialog;

impl FilenameDialog {
    pub fn new<E: Event, B: Bundle, M>(parent: &mut ChildBuilder, window: &Window, input_marker: impl Bundle, title: &str, init_value: String, observer: impl IntoObserverSystem<E, B, M>) {
        parent.spawn((
            ContextMenu::new_top(window.resolution.height() / 2.0 - 50.0, Val::Px(window.resolution.width() / 2.0 - 150.0)),
            FilenameDialog,
        )).with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(300.0),
                    height: Val::Px(65.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(title, TextStyle {
                        font: DefaultFonts::ROBOTO_REGULAR,
                        font_size: 18.0,
                        ..default()
                    }),
                    style: Style {
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                });

                parent.spawn((
                    TextInputBundle {
                        node_bundle: NodeBundle {
                            style: Style {
                                width: Val::Percent(95.0),
                                ..default()
                            },
                            ..default()
                        },
                        text_style: TextInputStyle {
                            font: DefaultFonts::ROBOTO_REGULAR,
                            font_size: 14.0,
                        },
                        settings: TextInputSettings {
                            submittable: true,
                            ..default()
                        },
                        content: TextInputContent::from_string(init_value),
                        ..default()
                    },
                    TextInputFocused,
                    input_marker,
                )).observe(observer);
            });
        });
    }
}
use bevy::prelude::*;
use crash_ide_assets::{DefaultColors, DefaultFonts};
use crash_ide_widget::Hoverable;

/// Component for context menu entries
///
/// Create rows with the `new` function to add all components and children.
#[derive(Component)]
pub struct ContextMenuRow;

impl ContextMenuRow {
    /// Creates a new [ContextMenuRow] with all components and children.
    pub fn new(parent: &mut ChildBuilder, title: &str, marker: impl Bundle, pre_icon: Option<Handle<Image>>, post_icon: Option<Handle<Image>>) {
        parent.spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                padding: UiRect::axes(Val::Px(6.0), Val::Px(3.0)),
                ..default()
            },
            ..default()
        }, Interaction::None, ContextMenuRow, marker, Hoverable::new(DefaultColors::GRAY.with_alpha(0.2)))).with_children(|parent| {
            let has_pre_icon = pre_icon.is_some();
            if let Some(icon) = pre_icon {
                parent.spawn(ImageBundle {
                    image: UiImage::new(icon),
                    style: Style {
                        height: Val::Px(15.0),
                        margin: UiRect::right(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                });
            }

            parent.spawn(TextBundle {
                text: Text::from_section(title, TextStyle {
                    font: DefaultFonts::ROBOTO_REGULAR,
                    font_size: 18.0,
                    ..default()
                }).with_no_wrap(),
                style: Style {
                    padding: UiRect::left(Val::Px(if has_pre_icon { 0.0 } else { 20.0 })),
                    ..default()
                },
                ..default()
            });

            if let Some(post_icon) = post_icon {
                parent.spawn(ImageBundle {
                    image: UiImage::new(post_icon),
                    style: Style {
                        height: Val::Px(15.0),
                        margin: UiRect::left(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                });
            }
        });
    }
}
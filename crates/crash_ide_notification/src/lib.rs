use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::utils::HashMap;

use crash_ide_assets::{DefaultColors, DefaultFonts, DefaultIcons};

pub struct CrashIDENotificationPlugin;

impl Plugin for CrashIDENotificationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NotificationContainerMap>()
            .add_systems(Update, (handle_new_notifications, cleanup_notification_map, close_notification_button))
        ;
    }
}

#[derive(Resource, Default)]
pub struct NotificationContainerMap {
    /// Maps window id to notification container id
    pub notification_map: HashMap<Entity, Entity>,
}

impl NotificationContainerMap {
    pub fn get(&self, window_entity: &Entity) -> Option<Entity> {
        self.notification_map.get(window_entity).map(|v| v.clone())
    }
}

#[allow(unused)]
pub enum NotificationIcon {
    Info,
    Warning,
    Error,
    Custom(Handle<Image>),
}

#[derive(Component)]
pub struct Notification {
    pub window_id: Entity,
    pub title: String,
    pub description: String,
    pub icon: NotificationIcon,
}

impl Notification {
    pub fn new(window_id: Entity, title: String, description: String, icon: NotificationIcon) -> Self {
        Self {
            window_id,
            title,
            description,
            icon,
        }
    }
}

#[derive(Component)]
struct NotificationCloseButton {
    notification: Entity,
}

fn cleanup_notification_map(
    mut removed: RemovedComponents<Window>,
    mut notification_container_map: ResMut<NotificationContainerMap>,
) {
    for entity in removed.read() {
        notification_container_map.notification_map.remove(&entity);
    }
}

fn handle_new_notifications(
    mut commands: Commands,
    query: Query<(Entity, &Notification), Added<Notification>>,
    icons: Res<DefaultIcons>,
    notification_container_map: Res<NotificationContainerMap>,
) {
    for (entity, notification) in query.iter() {
        let container_id = notification_container_map.get(&notification.window_id);

        let Some(container_id) = container_id else {
            continue;
        };

        commands.entity(entity).insert(NodeBundle {
            style: Style {
                margin: UiRect::axes(Val::Px(10.0), Val::Px(20.0)),
                flex_direction: FlexDirection::Row,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(DefaultColors::DEFAULT_BACKGROUND),
            border_color: BorderColor(Color::GRAY.with_a(0.1)),
            focus_policy: FocusPolicy::Block,
            ..default()
        }).with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Start,
                    width: Val::Px(20.0),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn(ImageBundle {
                    image: UiImage::new(match &notification.icon {
                        NotificationIcon::Info => icons.info.clone(),
                        NotificationIcon::Warning => icons.warning.clone(),
                        NotificationIcon::Error => icons.error.clone(),
                        NotificationIcon::Custom(handle) => handle.clone(),
                    }),
                    ..default()
                });
            });

            parent.spawn(NodeBundle {
                style: Style {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(notification.title.clone(), TextStyle {
                        font: DefaultFonts::ROBOTO_BOLD,
                        font_size: 18.0,
                        ..default()
                    }),
                    style: Style {
                        margin: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    ..default()
                });

                parent.spawn(TextBundle {
                    text: Text::from_section(notification.description.clone(), TextStyle {
                        font: DefaultFonts::ROBOTO_REGULAR,
                        font_size: 16.0,
                        ..default()
                    }),
                    style: Style {
                        margin: UiRect::axes(Val::Px(2.0), Val::Px(3.0)),
                        ..default()
                    },
                    ..default()
                });
            });

            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Start,
                    width: Val::Px(20.0),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn((ImageBundle {
                    image: UiImage::new(icons.cross.clone()),
                    ..default()
                }, NotificationCloseButton {
                    notification: entity,
                }, Interaction::None, Button));
            });
        });

        commands.entity(container_id).add_child(entity);
    }
}

fn close_notification_button(
    mut commands: Commands,
    query: Query<(&NotificationCloseButton, &Interaction), Changed<Interaction>>,
) {
    for (button, interaction) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        commands.entity(button.notification).despawn_recursive();
    }
}


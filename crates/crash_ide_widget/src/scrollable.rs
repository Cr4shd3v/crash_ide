use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

pub(super) struct ScrollablePlugin;

impl Plugin for ScrollablePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (create_scrollable, scroll, scroll_update))
        ;
    }
}

/// Marks an entity as scrollable.
///
/// Requires [Interaction].
#[derive(Component)]
pub struct Scrollable {
    /// Scroll speed, defaults to 200
    pub scroll_speed: f32,
}

impl Default for Scrollable {
    fn default() -> Self {
        Self {
            scroll_speed: 5000.0,
        }
    }
}

/// Child entity of [Scrollable], includes all content that should be scrollable.
#[derive(Component, Default)]
pub struct ScrollableContent {
    /// Scroll container offset to its parent [Scrollable]
    pub pos_y: f32,
}

fn create_scrollable(
    mut query: Query<&mut Style, Added<Scrollable>>,
) {
    for mut style in query.iter_mut() {
        style.overflow = Overflow::clip();
        style.justify_content = JustifyContent::Start;
        style.flex_direction = FlexDirection::Column;
    }
}

fn scroll(
    mut event_reader: EventReader<MouseWheel>,
    query: Query<(&Children, &Interaction, &Scrollable, &Node)>,
    time: Res<Time>,
    mut content_query: Query<(&mut ScrollableContent, &Node)>,
) {
    for event in event_reader.read() {
        for (children, interaction, scrollable, node) in query.iter() {
            if *interaction != Interaction::Hovered {
                continue;
            }

            let y = match event.unit {
                MouseScrollUnit::Line => {
                    event.y * time.delta().as_secs_f32() * scrollable.scroll_speed
                }
                MouseScrollUnit::Pixel => event.y,
            };

            let container_height = node.size().y;

            for &child in children.iter() {
                if let Ok((mut scrollable_content, content_node)) = content_query.get_mut(child) {
                    let max_scroll = (content_node.size().y - container_height).max(0.0);
                    scrollable_content.pos_y += y;
                    scrollable_content.pos_y = scrollable_content.pos_y.clamp(-max_scroll, 0.0);
                }
            }
        }
    }
}

fn scroll_update(
    mut query: Query<(&ScrollableContent, &mut Style), Changed<ScrollableContent>>,
) {
    for (content, mut style) in query.iter_mut() {
        style.top = Val::Px(content.pos_y);
    }
}

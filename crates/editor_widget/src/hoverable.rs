use bevy::prelude::*;

pub(super) struct HoverablePlugin;

impl Plugin for HoverablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (init_hover, on_hover));
    }
}

/// This component marks an ui element as hoverable.
///
/// When hovered, the background color of the element will become the `hover_color`.
#[derive(Component)]
pub struct Hoverable {
    hover_color: Color,
    saved_color: Option<Color>,
}

/// Marker component for hovered ui elements.
#[derive(Component)]
pub struct Hovered;

impl Hoverable {
    /// Creates a new [Hoverable] component with a given `hover_color`
    pub fn new(hover_color: Color) -> Self {
        Self {
            hover_color,
            saved_color: None,
        }
    }
}

fn init_hover(mut query: Query<(&mut Hoverable, &BackgroundColor), Added<Hoverable>>) {
    for (mut hoverable, background_color) in query.iter_mut() {
        hoverable.saved_color = Some(background_color.0);
    }
}

fn on_hover(
    mut commands: Commands,
    mut interaction_query: Query<(Entity, &Interaction, &Hoverable, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (entity, interaction, hoverable, mut background_color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                commands.entity(entity).insert(Hovered);
                background_color.0 = hoverable.hover_color;
            }
            Interaction::None => {
                commands.entity(entity).remove::<Hovered>();
                if let Some(ref color) = hoverable.saved_color {
                    background_color.0 = color.clone();
                }
            }
            _ => {},
        }
    }
}
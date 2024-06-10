use std::marker::PhantomData;
use bevy::prelude::*;

/// Extension to register an expandable menu
pub trait ExpandableMenuExtension {
    /// Register a type as marker for an expandable menu
    fn register_expandable_menu<T: Send + Sync + 'static>(&mut self);
}

impl ExpandableMenuExtension for App {
    fn register_expandable_menu<T: Send + Sync + 'static>(&mut self) {
        self
            .add_event::<ExpandMenuEvent<T>>()
            .add_systems(Update, expand_menu::<T>)
        ;
    }
}

/// Marker component for an expandable button
#[derive(Component, Default)]
pub struct ExpandableMenuButton<T: Send + Sync + 'static> {
    phantom_data: PhantomData<T>,
    override_pos: Option<Vec2>,
}

impl<T: Send + Sync + 'static> ExpandableMenuButton<T> {
    /// Set the `override_pos` for self
    pub fn with_override_pos(mut self, override_pos: Vec2) -> Self {
        self.override_pos = Some(override_pos);
        self
    }
}

/// Event produced when a expandable button should be expanded
#[derive(Event)]
pub struct ExpandMenuEvent<T: Send + Sync + 'static> {
    phantom_data: PhantomData<T>,
    /// Position where the menu should appear
    pub position: Vec2,
    /// Clicked entity
    pub entity: Entity,
}

fn expand_menu<T: Send + Sync + 'static>(
    query: Query<(&GlobalTransform, &Interaction, Entity, &ExpandableMenuButton<T>), Changed<Interaction>>,
    mut event_writer: EventWriter<ExpandMenuEvent<T>>,
) {
    for (transform, interaction, entity, menu_button) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let position = transform.translation().truncate();

        event_writer.send(ExpandMenuEvent {
            phantom_data: PhantomData,
            position: menu_button.override_pos.unwrap_or(position),
            entity,
        });
    }
}


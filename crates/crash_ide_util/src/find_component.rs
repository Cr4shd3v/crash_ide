use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

#[derive(SystemParam)]
pub struct FindComponentInParents<'w, 's, T: Component> {
    query: Query<'w, 's, (Entity, Option<&'static Parent>, Option<&'static T>)>,
}

impl<'w, 's, T: Component> FindComponentInParents<'w, 's, T> {
    pub fn find_entity(&self, entity: Entity) -> Option<Entity> {
        match self.query.get(entity) {
            Ok((entity, parent, value)) => {
                if value.is_some() {
                    Some(entity)
                } else if let Some(parent) = parent {
                    self.find_entity(parent.get())
                } else {
                    None
                }
            }
            Err(_) => {
                None
            }
        }
    }
}
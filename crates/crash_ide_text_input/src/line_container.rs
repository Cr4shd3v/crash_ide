use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::TextInputLineContainer;

/// Utility system param to retrieve the corresponding container of a line
#[derive(SystemParam)]
pub struct GetLineContainer<'w, 's> {
    children_query: Query<'w, 's, &'static Children>,
}

impl<'w, 's> GetLineContainer<'w, 's> {
    /// Returns the line content container for a line.
    pub fn get_line(&self, line_container: &TextInputLineContainer, line_index: usize) -> Entity {
        let line_content = self.children_query.get(line_container.line_content_container)
            .unwrap().get(line_index + 1).unwrap().clone();
        line_content
    }
}
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::CodeViewLineContainer;

/// Utility system param to retrieve the corresponding container of a line
#[derive(SystemParam)]
pub struct GetLineContainer<'w, 's> {
    children_query: Query<'w, 's, &'static Children>,
}

impl<'w, 's> GetLineContainer<'w, 's> {
    /// Returns both the line count and line content container for a line.
    ///
    /// First entity is line count, second is line content
    pub fn get_line(&self, line_container: &CodeViewLineContainer, line_index: usize) -> (Entity, Entity) {
        let line_count = self.children_query.get(line_container.line_count_container)
            .unwrap().get(line_index).unwrap().clone();
        let line_content = self.children_query.get(line_container.line_content_container)
            .unwrap().get(line_index + 1).unwrap().clone();
        (line_count, line_content)
    }
}
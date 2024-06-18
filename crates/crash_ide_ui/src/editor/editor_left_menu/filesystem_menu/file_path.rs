use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::editor::editor_left_menu::{FileDisplay, ProjectRoot};

#[derive(SystemParam)]
pub struct FilePath<'w, 's> {
    query: Query<'w, 's, (&'static Parent, &'static FileDisplay, Option<&'static ProjectRoot>)>,
}

impl<'w, 's> FilePath<'w, 's> {
    pub fn get_full_path(&self, row_entity: Entity) -> String {
        let mut entity = row_entity;
        let mut path = vec![];

        loop {
            let (parent, file_display, root) = self.query.get(entity).unwrap();

            if let Some(root) = root {
                path.reverse();
                return format!("{}/{}", root.full_path, path.join("/"));
            } else {
                path.push(file_display.filename.clone());
            }

            entity = parent.get();
        }
    }

    pub fn get_directory(&self, row_entity: Entity) -> String {
        self.get_full_path(self.query.get(row_entity).unwrap().0.get())
    }
}